use actix_web::web;
use serde::{Deserialize,Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct Record{
    pub id:i32,
    pub user_id:i32,
    pub book_id:i32,
    pub borrow_time: Option<NaiveDateTime>,
    pub return_time: Option<NaiveDateTime>,
    pub is_return:bool,
}