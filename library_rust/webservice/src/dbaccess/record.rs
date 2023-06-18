use sqlx::postgres::PgPool;

use crate::{models::{record::bRecord, book::{ReturnBook, Book}}, error::MyError};

pub async fn get_all_records_by_user_id_db(pool: &PgPool,user_id:i32)->Result<Vec<bRecord>,MyError>
{
    let rows = sqlx::query!("SELECT id,user_id,book_id,borrow_time,return_time,is_return FROM record where user_id=$1 order by is_return,id",user_id)
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

pub async fn get_all_records_db(pool: &PgPool)->Result<Vec<bRecord>,MyError>
{
    let rows = sqlx::query!("SELECT id,user_id,book_id,borrow_time,return_time,is_return FROM record order by id")
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
    
    // let a= sqlx::query("select LOCALTIME").fetch_one(pool).await;
    // println!("{:?}", a.unwrap().into());
    let record_row = sqlx::query_as!(
        bRecord,
        "UPDATE record SET is_return = TRUE,return_time=(select NOW() at time zone 'Asia/Shanghai')
            where id = $1
            RETURNING id, user_id, book_id, borrow_time,return_time, is_return",
        return_book.record_id,
        )
    .fetch_one(pool)
    .await;

    if let Ok(_record) = record_row {
        Ok(book_row)
    } else {
        //手动事务
        let _book_row = sqlx::query_as!(
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
#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use rstest::rstest;
    use sqlx::{postgres::PgPoolOptions};

    #[rstest]
    #[case(1)]
    #[case(3)]
    #[actix_rt::test]
    async fn test_get_all_records_by_user_id_db(#[case] id:i32) -> Result<(), MyError>{
        let pool = get_test_db_pool().await?;
        let user_id = id;
        let records = get_all_records_by_user_id_db(&pool, user_id).await;
        if id!=1 {
            assert!(records.is_err());
        }
        else {
            let records_ref = &mut records.unwrap();
            assert_eq!(records_ref.len(), 2);
            assert_eq!(records_ref[0].user_id, user_id);
        }
        
        Ok(())
    }

    #[actix_rt::test]
    async fn test_get_all_records_db() -> Result<(), MyError>{
        let pool = get_test_db_pool().await?;
        let records = get_all_records_db(&pool).await?;
        assert_eq!(records.len(), 3);
        Ok(())
    }

    #[rstest]
    #[case(1,2)]
    #[case(3,1)]
    #[actix_rt::test]
    async fn test_return_book_db(#[case] record_id: i32, #[case] book_id: i32) -> Result<(), MyError>{
        let pool = get_test_db_pool().await?;
        let return_book = ReturnBook { record_id, book_id };
        let book = return_book_db(&pool, return_book).await;
        if record_id==3
        {
            assert!(book.is_err());
        }
        else {
            assert_eq!(book?.is_borrowed, false);
        }
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
        sqlx::query("TRUNCATE TABLE user1 RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await?;
        sqlx::query(
            "INSERT INTO book (name, writer, press, is_borrowed)
            VALUES ('Book1', 'Writer1', 'Press1', FALSE),
                   ('Book2', 'Writer2', 'Press2', TRUE),
                   ('Book3', 'Writer3', 'Press3', TRUE)",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO user1 (name, password, is_mana)
            VALUES ('李应东', '123456', TRUE),
                ('范兆基', '123456', FALSE),
                ('李昱航', '123456', FALSE)",
        )
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO record (user_id, book_id,borrow_time,return_time, is_return)
            VALUES (1, 2, (select NOW() at time zone 'Asia/Shanghai'), NULL, FALSE),
            (2, 3, (select NOW() at time zone 'Asia/Shanghai'), NULL, FALSE),
            (1, 1, (select NOW() at time zone 'Asia/Shanghai'), NULL, TRUE)",
        )
        .execute(&pool)
        .await?;
        Ok(pool)
    }
}
