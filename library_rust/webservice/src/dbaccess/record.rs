use sqlx::postgres::PgPool;

use crate::{models::record::bRecord, error::MyError};

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