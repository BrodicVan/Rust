use sqlx::postgres::PgPool;
use sqlx::Pool;

use crate::{models::{book::{Book, BorrowBook}, record::Record}, error::MyError};

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

pub async fn borrow_book_db(
    pool: &PgPool,
    user_id: i32,
    borrow_book: BorrowBook,
) -> Result<Book, MyError> {
    // let current_book_row = sqlx::query_as!(
    //     Book,
    //     "SELECT * FROM book where id=$1",
    //     borrow_book.id
    // )
    // .fetch_one(pool)
    // .await
    // .map_err(|_err| MyError::NotFound("Book Id not found".into()))?;

    let book_row = sqlx::query_as!(
        Book,
        "UPDATE book SET is_borrowed = TRUE
            where id = $1
            RETURNING id, name, writer, press, is_borrowed",
        borrow_book.id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Book Id not found".into()))?;
    

    let record_row = sqlx::query_as!(
        Record,
        r#"INSERT INTO record (user_id, book_id,return_time, is_return)
        VALUES ($1, $2, NULL, FALSE)
        RETURNING id, user_id, book_id, borrow_time, return_time, is_return"#,
        user_id, borrow_book.id
        )
    .fetch_one(pool)
    .await;

    if let Ok(record) = record_row {
        Ok(book_row)
    } else {
        //手动事务
        let book_row = sqlx::query_as!(
            Book,
            "UPDATE book SET is_borrowed = FALSE
                where id = $1
                RETURNING id, name, writer, press, is_borrowed",
            borrow_book.id,
        )
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFound("Book Id not found".into()))?;
        Err(MyError::DBError("DB operation fail".into()))
    }
    
}