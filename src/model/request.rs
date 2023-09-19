use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub  struct  CreateCustomerRequest{
    pub id:Option<i64>,
    pub  age:i32,
    pub  email:String,
    pub  username:String,
    pub  password:String

}