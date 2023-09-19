use std::fmt::Display;
use std::str::Chars;
use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::{json, to_value, Value};
use sqlx::{Executor, query, query_as, query_as_with, Row};
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
            let str: Value = to_value(customer).unwrap();
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
    }
}


pub async fn update_customer_handler(
    State(data): State<Arc<AppConfig>>,
    Json(body): Json<CreateCustomerRequest>,
) -> impl IntoResponse {
    let result = query("   UPDATE customer_info set  age = $1 , email = $2, password=$3 , username=$4  where id = $5 ")
        .bind(body.age)
        .bind(body.email)
        .bind(body.password)
        .bind(body.username)
        .bind(body.id.unwrap())
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
    }
}

pub async fn delete_customer_handler(
    State(data): State<Arc<AppConfig>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    print!(";f;f;");
    let result = query("   delete from  customer_info  where id = $1 ")
        .bind(id)
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
    }
}
