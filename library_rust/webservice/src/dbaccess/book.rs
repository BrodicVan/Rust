use sqlx::postgres::PgPool;
use sqlx::Pool;

use crate::{models::{book::{Book, BorrowBook, CreateBook, UpdateBook, DeleteBook}, record::bRecord}, error::MyError};

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
        bRecord,
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

pub async fn add_book_db(
    pool: &PgPool,
    new_book: CreateBook,
) -> Result<Book, MyError> {
    let row = sqlx::query_as!(
        Book,
        r#"INSERT INTO book (name,writer,press,is_borrowed)
        VALUES ($1, $2, $3,FALSE)
        RETURNING id, name, writer, press,is_borrowed"#,
        new_book.name,new_book.writer,new_book.press
        )
    .fetch_one(pool)
    .await?;

    Ok(row)//Ok(User?
}

pub async fn edit_book_db(
    pool: &PgPool,
    update_book: UpdateBook,
) -> Result<Book, MyError> {
    let current_book_row = sqlx::query_as!(
        Book,
        "SELECT * FROM book where id=$1",
        update_book.id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Book Id not found".into()))?;

    let name: String = if let Some(name) = update_book.name {
        name
    } else {
        current_book_row.name
    };
    let writer: String = if let Some(writer) = update_book.writer {
        writer
    } else {
        current_book_row.writer
    };
    let press: String = if let Some(press) = update_book.press {
        press
    } else {
        current_book_row.press
    };
    
    let book_row = sqlx::query_as!(
        Book,
        "UPDATE book SET name = $1, writer = $2, press = $3
            where id = $4
            RETURNING id, name, writer,press,is_borrowed",
        name,
        writer,
        press,
        update_book.id
    )
    .fetch_one(pool)
    .await;
    if let Ok(book) = book_row {
        Ok(book)
    } else {
        Err(MyError::NotFound("Book id not found".into()))
    }
}


pub async fn delete_book_db(pool: &PgPool, deletebook: DeleteBook) -> Result<String, MyError> {
    let book_row = sqlx::query!(
        "DELETE FROM book where id=$1",
        deletebook.id,
    )
    .execute(pool)
    .await?;
    Ok(format!("DeletedI{:?}record", book_row))
}