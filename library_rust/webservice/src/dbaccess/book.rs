use sqlx::postgres::PgPool;


use crate::{models::{book::{Book, BorrowBook, CreateBook, UpdateBook}, record::bRecord}, error::MyError};

pub async fn get_all_books_db(pool: &PgPool)->Result<Vec<Book>,MyError>
{
    let rows = sqlx::query!("SELECT id,name,writer,press,is_borrowed FROM book order by id")
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
        r#"INSERT INTO record (user_id, book_id,borrow_time,return_time, is_return)
        VALUES ($1, $2, (select NOW() at time zone 'Asia/Shanghai'), NULL, FALSE)
        RETURNING id, user_id, book_id, borrow_time, return_time, is_return"#,
        user_id, borrow_book.id
        )
    .fetch_one(pool)
    .await;

    if let Ok(_record) = record_row {
        Ok(book_row)
    } else {
        //手动事务
        let _book_row = sqlx::query_as!(
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


pub async fn delete_book_db(pool: &PgPool, book_id: i32) -> Result<String, MyError> {
    let book_row = sqlx::query!(
        "DELETE FROM book where id=$1",
        book_id,
    )
    .execute(pool)
    .await?;
    Ok(format!("DeletedI{:?}record", book_row.rows_affected()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use sqlx::{ PgPool, postgres::PgPoolOptions};
    use std::env;


    #[actix_rt::test]
    async fn test_get_all_books_db() -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let expected_books = vec![
            Book {
                id: 1,
                name: "Book1".to_string(),
                writer: "Writer1".to_string(),
                press: "Press1".to_string(),
                is_borrowed: false,
            },
            Book {
                id: 2,
                name: "Book2".to_string(),
                writer: "Writer2".to_string(),
                press: "Press2".to_string(),
                is_borrowed: true,
            },
        ];
        let actual_books = get_all_books_db(&pool).await?;
        assert_eq!(actual_books[0], expected_books[0]);
        Ok(())
    }

    #[rstest]
    #[case(1, "Book1".to_string(), "Writer1".to_string(), "Press1".to_string(),false)]
    #[case(2, "Book2".to_string(), "Writer2".to_string(), "Press2".to_string(),true)]
    #[case(3, "Book3".to_string(), "Writer3".to_string(), "Press3".to_string(),false)]
    #[actix_rt::test]
    async fn test_borrow_book_db(#[case] id: i32, #[case] name: String, #[case] writer: String,#[case] press: String,#[case] is_borrowed: bool) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let user_id = 1;
        let borrow_book = BorrowBook { id: id };
        let expected_book = Book {
            id: id,
            name: name,
            writer: writer,
            press: press,
            is_borrowed: true,
        };
        let actual_book = borrow_book_db(&pool, user_id, borrow_book).await;
        if id>2 {
            assert!(actual_book.is_err());
        }
        else{
            if is_borrowed {
                assert!(actual_book.is_err());
                }
            else { assert_eq!(actual_book?, expected_book); }
        }
        
        Ok(())
    }

    #[rstest]
    #[case("Book3".to_string(), "Writer3".to_string(), "Press3".to_string())]
    #[case("".to_string(), "".to_string(), "".to_string())]
    #[case("锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈".to_string(), "Writer3".to_string(), "Press3".to_string())]
    #[actix_rt::test]
    async fn test_add_book_db(#[case] name: String, #[case] writer: String,#[case] press: String) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let lenth=name.len();
        let new_book = CreateBook {
            name: name,
            writer: writer,
            press: press,
        };
        let actual_book = add_book_db(&pool, new_book).await;
        if lenth>140||lenth==0
        {
            assert!(actual_book.is_err())
        }
        else {
            assert_eq!(actual_book?.id, 3);            
        }
        Ok(())
    }

    #[rstest]
    #[case(1,"NewName".to_string(), "NewWriter".to_string(), "NewPress".to_string())]
    #[case(1,"".to_string(), "".to_string(), "".to_string())]
    #[case(66,"NewName".to_string(), "NewWriter".to_string(), "NewPress".to_string())]
    #[actix_rt::test]
    async fn test_edit_book_db(#[case] id: i32,#[case] name: String, #[case] writer: String,#[case] press: String) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let lenth=name.len();
        let update_book = UpdateBook {
            id: id,
            name: Some(name),
            writer: Some(writer),
            press: Some(press),
        };
        let expected_book = Book {
            id: 1,
            name: "NewName".to_string(),
            writer: "NewWriter".to_string(),
            press: "NewPress".to_string(),
            is_borrowed: false,
        };
        let actual_book = edit_book_db(&pool, update_book).await;
        if id!=1||lenth==0 {
            assert!(actual_book.is_err())
        }
        else {
            assert_eq!(actual_book?, expected_book);
        }
        Ok(())
    }

    #[rstest]
    #[case(1)]
    #[case(66)]
    #[actix_rt::test]
    async fn test_delete_book_db(#[case] id:i32) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let book_id = id;
        let expected_message;
        if id==1 {
            expected_message = format!("DeletedI{:?}record", 1);
        }
        else {
            expected_message = format!("DeletedI{:?}record", 0);
        }
        let actual_message = delete_book_db(&pool, book_id).await?;
        assert_eq!(actual_message, expected_message);
        Ok(())
    }

    async fn get_test_db_pool() -> Result<PgPool, MyError> {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await?;
        sqlx::query("TRUNCATE TABLE book RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await?;
        sqlx::query("TRUNCATE TABLE record RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await?;
        sqlx::query(
            "INSERT INTO book (name, writer, press, is_borrowed)
            VALUES ('Book1', 'Writer1', 'Press1', FALSE),
                   ('Book2', 'Writer2', 'Press2', TRUE)",
        )
        .execute(&pool)
        .await?;
        Ok(pool)
    }
}
