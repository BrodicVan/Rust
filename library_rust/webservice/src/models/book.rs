use actix_web::web;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug,Clone, Eq)]
pub struct Book{
    pub id:i32,
    pub name:String,
    pub writer:String,
    pub press:String,
    pub is_borrowed:bool,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct BorrowBook{
    pub id:i32,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct ReturnBook{
    pub record_id:i32,
    pub book_id:i32,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct CreateBook{
    pub name:String,
    pub writer:String,
    pub press:String,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct UpdateBook{
    pub id:i32,
    pub name:Option<String>,
    pub writer:Option<String>,
    pub press:Option<String>,
}

#[derive(Deserialize,Debug,Clone)]
pub struct DeleteBook{
    pub id:i32,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.name == other.name &&
        self.writer == other.writer &&
        self.press == other.press &&
        self.is_borrowed == other.is_borrowed
    }
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

impl From<web::Json<CreateBook>> for CreateBook{
    fn from(create_book: web::Json<CreateBook>) -> Self {
        CreateBook { 
            name:create_book.name.clone() ,
            writer:create_book.writer.clone(),
            press:create_book.press.clone(),
        }        
    }
}

impl From<web::Json<UpdateBook>> for UpdateBook{
    fn from(update_book: web::Json<UpdateBook>) -> Self {
        UpdateBook {
            id:update_book.id, 
            name:update_book.name.clone() ,
            writer:update_book.writer.clone(),
            press:update_book.press.clone(),
        }        
    }
}

impl From<web::Json<DeleteBook>> for DeleteBook{
    fn from(delete_book: web::Json<DeleteBook>) -> Self {
        DeleteBook {
            id:delete_book.id, 
            
        }        
    }
}