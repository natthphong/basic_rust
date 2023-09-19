use std::any::{Any, TypeId};
use std::fmt::{Display, Error};
use std::ptr::null;
use std::str::Chars;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use redis::{Commands, RedisResult};
use redis::Value::Nil;
use serde_json::{json, to_value, Value};
use sqlx::{Connection, Executor, query, query_as, query_as_with, Row};
use crate::AppConfig;
use crate::model::entity::CustomerInfo;
use crate::model::request::CreateCustomerRequest;


pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Hello world";

    let json_response = json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response).into_response()
}

pub async fn get_customer_by_id_handler(
    Path(id): Path<i32>,
    State(data): State<Arc<AppConfig>>,
) -> impl IntoResponse {
    let  fields:RedisResult<Option<String>> = data.get_redis().lock().unwrap().hget("customer",id);
    let resp =fields.unwrap();
    if !resp.is_none(){
        let json_str = resp.unwrap();
        let parsed_json = serde_json::from_str::<Value>(&json_str).ok().unwrap();
        let json_response = json!({
                "status": "success",
                "message": "success",
                "body": parsed_json
            });
        return Json(json_response).into_response();
    }
    let result = query("select * from customer_info where id = $1")
        .bind(id)
        .fetch_one(&data.pool).await;
    return match result
    {
        Ok(row) => {
            let customer = CustomerInfo::
            new(row.get("id")
                , row.try_get("age").ok().unwrap()
                , row.get("email"), row.try_get("password").ok().unwrap()
                , row.try_get("username").ok().unwrap(), row.try_get("is_delete").ok().unwrap(),
            );
            let str: Value = to_value(&customer).unwrap();
            let mut redis_lock = data.get_redis().lock().unwrap();
            let _: () = redis_lock.hset("customer", customer.get_id(), str.to_string()).unwrap();
            let json_response = json!({
        "status": "success",
        "message": "success",
                "body":str
            });

            Json(json_response).into_response()
        }
        Err(e) => {
            let json_response = json!({
        "status": "success",
        "message": e.to_string()
            });

            Json(json_response).into_response()
        }
    };
}


pub async fn get_all_customers_handler(
    State(data): State<Arc<AppConfig>>,
) -> impl IntoResponse {
    let mut fields: Vec<(String, String)> = data.get_redis().lock().unwrap().hgetall("customer").unwrap();
    fields.sort_by(|a, b| a.0.parse::<i32>().unwrap().cmp(&b.0.parse::<i32>().unwrap()));

    if !fields.is_empty() {
        let resp: Vec<Value> = fields.iter().map(|(_, value)| serde_json::from_str(&value.clone()).unwrap()).collect();
        let json_response = json!({
                "status": "success",
                "message": "success",
                "body": resp
            });
        return Json(json_response).into_response();
    }

    let result = query("SELECT * FROM customer_info")
        .fetch_all(&data.pool)
        .await;

    match result {
        Ok(rows) => {
            let customers: Vec<CustomerInfo> = rows
                .into_iter()
                .map(|row| CustomerInfo::new(
                    row.get("id"),
                    row.try_get("age").ok().unwrap(),
                    row.get("email"),
                    row.try_get("password").ok().unwrap(),
                    row.try_get("username").ok().unwrap(),
                    row.try_get("is_delete").ok().unwrap(),
                ))
                .collect();

            let mut redis_lock = data.get_redis().lock().unwrap();

            for customer in customers.iter().clone() {
                let _: () = redis_lock.hset("customer", customer.get_id(), to_value(customer).unwrap().to_string()).unwrap();
            }

            let str: Value = to_value(customers).unwrap();


            let json_response = json!({
                "status": "success",
                "message": "success",
                "body": str
            });

            Json(json_response).into_response()
        }
        Err(e) => {
            let json_response = json!({
                "status": "error",
                "message": e.to_string()
            });

            Json(json_response).into_response()
        }
    }
}


// pub async  fn get_customer_all_redis( data: &Arc<AppConfig>)->Result<impl IntoResponse,Error>{
//     let mut redis_lock = data.get_redis().lock().unwrap();
//     let mut fields:Vec<(String, String)>  =  redis_lock.hgetall("customer").unwrap();
//     fields.sort_by(|a, b| a.0.parse::<i32>().unwrap().cmp(&b.0.parse::<i32>().unwrap()));
//     if fields.is_empty() { Err("Not Found") }
//     return Ok(Json(fields.clone()));
// }
pub async fn create_customer_handler(
    State(data): State<Arc<AppConfig>>,
    Json(body): Json<CreateCustomerRequest>,
) -> impl IntoResponse {
    let result = query("  INSERT INTO customer_info(age, email, password, username) VALUES( $1 , $2 , $3 , $4 ) ")
        .bind(body.age)
        .bind(body.email)
        .bind(body.password)
        .bind(body.username)
        .execute(&data.pool)
        .await;
     return match result {
        Ok(_) => {
            let json_response = json!({
                "status": "success",
                "message": "success"
            });
            Json(json_response).into_response()
        }
        Err(e) => {
            let json_response = json!({
                "status": "error",
                "message": e.to_string()
            });

            Json(json_response).into_response()
        }
    };
}


pub async fn update_customer_handler(
    State(data): State<Arc<AppConfig>>,
    Json(body): Json<CreateCustomerRequest>,
) -> impl IntoResponse {
    let result = query("   UPDATE customer_info set  age = $1 , email = $2, password=$3 , username=$4  where id = $5 ")
        .bind(&body.age)
        .bind(&body.email)
        .bind(&body.password)
        .bind(&body.username)
        .bind(&body.id.unwrap())
        .execute(&data.pool)
        .await;
    return match result {
        Ok(_) => {
            let json_response = json!({
                "status": "success",
                "message": "success"
            });
            let mut redis_lock = data.get_redis().lock().unwrap();
            let _: () = redis_lock.hset("customer", &body.id, to_value(&body).unwrap().to_string()).unwrap();
            Json(json_response).into_response()
        }
        Err(e) => {
            let json_response = json!({
                "status": "error",
                "message": e.to_string()
            });

            Json(json_response).into_response()
        }
    };
}

pub async fn delete_customer_handler(
    State(data): State<Arc<AppConfig>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let result = query("   delete from  customer_info  where id = $1 ")
        .bind(&id)
        .execute(&data.pool)
        .await;
    return match result {
        Ok(_) => {
            let json_response = json!({
                "status": "success",
                "message": "success"
            });
            let mut redis_lock = data.get_redis().lock().unwrap();
            let _: () = redis_lock.hdel("customer",&id).unwrap();
            Json(json_response).into_response()
        }
        Err(e) => {
            let json_response = json!({
                "status": "error",
                "message": e.to_string()
            });

            Json(json_response).into_response()
        }
    };
}
