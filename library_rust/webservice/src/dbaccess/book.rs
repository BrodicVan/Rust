use sqlx::PgPool;

use crate::{models::book::Book, error::MyError};

pub async fn get_all_books_db(pool: &PgPool)->Result<Vec<Book>,MyError>
{
    let rows = sqlx::query!("SELECT id,name,writer,press,is_borrowed FROM book")
        .fetch_all(pool)
        .await?;
    let books: Vec<Book> = rows
        .iter()
        .map(|r| Book{
            id: r.id,
            name: r.name.clone(),
            writer: r.writer.clone(),
            press: r.press.clone(),
            is_borrowed: r.is_borrowed,
        })
        .collect();
    match books.len(){
        0 => Err(MyError::NotFound("No books found".into())),
        _ => Ok(books),
    }

    

}