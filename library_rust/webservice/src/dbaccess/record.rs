use sqlx::postgres::PgPool;

use chrono::NaiveDateTime;
use crate::{models::{record::bRecord, book::{ReturnBook, Book}}, error::MyError};

pub async fn get_all_records_by_user_id_db(pool: &PgPool,user_id:i32)->Result<Vec<bRecord>,MyError>
{
    let rows = sqlx::query!("SELECT id,user_id,book_id,borrow_time,return_time,is_return FROM record where user_id=$1",user_id)
        .fetch_all(pool)
        .await?;
    let records: Vec<bRecord> = rows
        .iter()
        .map(|r| bRecord{
            id: r.id,
            user_id:r.user_id,
            book_id: r.book_id,
            borrow_time: r.borrow_time.clone(),
            return_time: r.return_time.clone(),
            is_return: r.is_return,
        })
        .collect();
    match records.len(){
        0 => Err(MyError::NotFound("No records found".into())),
        _ => Ok(records),
    }
}

pub async fn return_book_db(
    pool: &PgPool,
    return_book: ReturnBook,
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
        "UPDATE book SET is_borrowed = FALSE
            where id = $1
            RETURNING id, name, writer, press, is_borrowed",
        return_book.book_id,
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Book Id not found".into()))?;
    

    let record_row = sqlx::query_as!(
        bRecord,
        "UPDATE record SET is_return = TRUE,return_time=NOW()
            where id = $1
            RETURNING id, user_id, book_id, borrow_time,return_time, is_return",
        return_book.record_id,
        )
    .fetch_one(pool)
    .await;

    if let Ok(record) = record_row {
        Ok(book_row)
    } else {
        //手动事务
        let book_row = sqlx::query_as!(
            Book,
            "UPDATE book SET is_borrowed = TRUE
                where id = $1
                RETURNING id, name, writer, press, is_borrowed",
            return_book.book_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|_err| MyError::NotFound("Book Id not found".into()))?;
        Err(MyError::DBError("DB operation fail".into()))
    }
    
}