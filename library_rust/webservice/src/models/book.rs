use actix_web::web;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Book{
    pub id:i32,
    pub name:String,
    pub writer:String,
    pub press:String,
    pub is_borrowed:bool,
}