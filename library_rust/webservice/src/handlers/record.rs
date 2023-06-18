use actix_web::{web, HttpResponse};

use crate::{state::AppState, error::MyError, db_access::record::{get_all_records_by_user_id_db, return_book_db, get_all_records_db}, models::book::ReturnBook};

pub async fn get_all_records_by_user_id(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    // let teacher_id = i32::try_from(params.0).unwrap();
    let user_id = params.into_inner();
    get_all_records_by_user_id_db(&app_state.db,user_id)
        .await
        .map(|records| HttpResponse::Ok().json(records))
}

pub async fn get_all_records(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    println!("进来这里了");
    get_all_records_db(&app_state.db)
        .await
        .map(|records| HttpResponse::Ok().json(records))
}

pub async fn return_book(
    app_state: web::Data<AppState>,
    return_book: web::Json<ReturnBook>,
) -> Result<HttpResponse, MyError> {
    
    return_book_db(&app_state.db,ReturnBook::from(return_book))
        .await
        .map(|records| HttpResponse::Ok().json(records))
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use actix_web::{http::StatusCode};
    use rstest::rstest;
    use sqlx::PgPool;
    use std::env;
    async fn create_test_pool() -> PgPool {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPool::connect(&database_url).await.unwrap();
        sqlx::query("TRUNCATE TABLE book RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await.unwrap();
        sqlx::query("TRUNCATE TABLE record RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await.unwrap();
        sqlx::query("TRUNCATE TABLE user1 RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await.unwrap();
        sqlx::query(
            "INSERT INTO book (name, writer, press, is_borrowed)
            VALUES ('Book1', 'Writer1', 'Press1', FALSE),
                   ('Book2', 'Writer2', 'Press2', TRUE),
                   ('Book3', 'Writer3', 'Press3', TRUE)",
        )
        .execute(&pool)
        .await.unwrap();
        sqlx::query(
            "INSERT INTO user1 (name, password, is_mana)
            VALUES ('李应东', '123456', TRUE),
                ('范兆基', '123456', FALSE),
                ('李昱航', '123456', FALSE)",
        )
        .execute(&pool)
        .await.unwrap();
        sqlx::query(
            "INSERT INTO record (user_id, book_id,borrow_time,return_time, is_return)
            VALUES (1, 2, (select NOW() at time zone 'Asia/Shanghai'), NULL, FALSE),
            (2, 3, (select NOW() at time zone 'Asia/Shanghai'), NULL, FALSE),
            (1, 1, (select NOW() at time zone 'Asia/Shanghai'), NULL, TRUE)",
        )
        .execute(&pool)
        .await.unwrap();
        pool
    }
    #[rstest]
    #[case(1)]
    // #[case(3)]
    #[actix_rt::test]
    async fn test_get_all_records_by_user_id(#[case] id:i32) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0), db: pool });
        // let req = test::TestRequest::get().to_http_request();
        let resp = get_all_records_by_user_id(app_state,web::Path::from(id)).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    
    #[actix_rt::test]
    async fn test_get_all_records() -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState {health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0), db: pool });
        let resp = get_all_records(app_state).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case(1,2)]
    // #[case(3,1)]
    #[actix_rt::test]
    async fn test_return_book(#[case] record_id: i32, #[case] book_id: i32) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        let book = ReturnBook {
            record_id: record_id,
            book_id: book_id,
        };
        /*let req = test::TestRequest::post()
            .uri("/1/edit_book/")
            .set_json(&new_book)
            .to_http_request();*/
        let resp = return_book(app_state,web::Json(book)).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }

}

