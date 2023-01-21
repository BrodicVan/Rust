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

#[derive(Deserialize,Debug,Clone)]
pub struct ReturnBook{
    pub record_id:i32,
    pub book_id:i32,
}

impl From<web::Json<BorrowBook>> for BorrowBook{
    fn from(borrow_book: web::Json<BorrowBook>) -> Self {
        BorrowBook { 
            id:borrow_book.id ,
        }        
    }
}

impl From<web::Json<ReturnBook>> for ReturnBook{
    fn from(return_book: web::Json<ReturnBook>) -> Self {
        ReturnBook { 
            record_id:return_book.record_id ,
            book_id:return_book.book_id,
        }        
    }
}