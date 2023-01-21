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

#[derive(Deserialize,Debug,Clone)]
pub struct BorrowBook{
    pub id:i32,
}

impl From<web::Json<BorrowBook>> for BorrowBook{
    fn from(borrow_book: web::Json<BorrowBook>) -> Self {
        BorrowBook { 
            id:borrow_book.id ,
        }        
    }
}