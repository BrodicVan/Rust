use actix_web::{web, HttpResponse};

use crate::{state::AppState, error::MyError, db_access::book::{get_all_books_db, borrow_book_db, add_book_db, edit_book_db, delete_book_db}, models::book::{BorrowBook, CreateBook, UpdateBook}};

pub async fn get_all_books(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    // let teacher_id = i32::try_from(params.0).unwrap();
    
    get_all_books_db(&app_state.db)
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn borrow_book(
    app_state: web::Data<AppState>,
    borrow_book: web::Json<BorrowBook>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let user_id = params.into_inner();
    borrow_book_db(&app_state.db, user_id, BorrowBook::from(borrow_book),)
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn add_book(
    new_book: web::Json<CreateBook>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    add_book_db(&app_state.db, CreateBook::from(new_book))
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn edit_book(
    app_state: web::Data<AppState>,
    update_book: web::Json<UpdateBook>,
) -> Result<HttpResponse, MyError> {
    edit_book_db(&app_state.db,  update_book.into())
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn delete_book(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
)->Result<HttpResponse,MyError>{
    let (_, book_id) = params.into_inner();
    delete_book_db(&app_state.db, book_id)
        .await
        .map(|book| HttpResponse::Ok().json(book))
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
        sqlx::query(
            "INSERT INTO book (name, writer, press, is_borrowed)
            VALUES ('Book1', 'Writer1', 'Press1', FALSE),
                   ('Book2', 'Writer2', 'Press2', TRUE)",
        )
        .execute(&pool)
        .await.unwrap();
        pool
    }

    #[actix_rt::test]
    async fn test_get_all_books() -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0), db: pool });
        // let req = test::TestRequest::get().to_http_request();
        let resp = get_all_books(app_state).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case(1)]
    // #[case(2)]
    // #[case(3)]
    #[actix_rt::test]
    async fn test_borrow_book(#[case] id: i32) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState {health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0), db: pool });
        let book = BorrowBook {
            id: 1,
        };
        /*let req = test::TestRequest::post()
            .uri("/1/borrow_book/")
            .set_json(&book)
            .to_http_request();*/
        let resp = borrow_book(app_state, web::Json(book), web::Path::from(id)).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case("Book3".to_string(), "Writer3".to_string(), "Press3".to_string())]
    // #[case("".to_string(), "".to_string(), "".to_string())]
    // #[case("锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈锈".to_string(), "Writer3".to_string(), "Press3".to_string())]
    #[actix_rt::test]
    async fn test_add_book(#[case] name: String, #[case] writer: String,#[case] press: String) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        let new_book = CreateBook {
            name: name,
            writer: writer,
            press: press,
        };
        /*let req = test::TestRequest::post()
            .uri("/1/edit_book/")
            .set_json(&new_book)
            .to_http_request();*/
        let resp = add_book(web::Json(new_book),app_state).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case(1,"NewName".to_string(), "NewWriter".to_string(), "NewPress".to_string())]
    // #[case(1,"".to_string(), "".to_string(), "".to_string())]
    // #[case(66,"NewName".to_string(), "NewWriter".to_string(), "NewPress".to_string())]
    #[actix_rt::test]
    async fn test_edit_book(#[case] id: i32,#[case] name: String, #[case] writer: String,#[case] press: String) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        let update_book = UpdateBook {
            id: id,
            name: Some(name),
            writer: Some(writer),
            press: Some(press),
        };
        /*let req = test::TestRequest::put()
            .uri("/edit_book")
            .set_json(&update_book)
            .to_http_request();*/
        let resp = edit_book(app_state, web::Json(update_book)).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case(1)]
    // #[case(66)]
    #[actix_rt::test]
    async fn test_delete_book(#[case] id:i32) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        // let req = test::TestRequest::delete().uri("/delete_book/1/1").to_http_request();
        let resp = delete_book(app_state, web::Path::from((1, 1))).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
}
