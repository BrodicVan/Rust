use crate::db_access::user::*;
use crate::error::MyError;
use crate::models::user::{CreateUser, UpdateUser, LoginUser, RegUser};
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_users(app_state: web::Data<AppState>)-> Result<HttpResponse, MyError>{
    
    get_all_users_db(&app_state.db)
        .await
        .map(|users| HttpResponse::Ok().json(users))
}

pub async fn get_user_by_id(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    // let teacher_id = i32::try_from(params.0).unwrap();
    let id = params.into_inner();
    get_user_by_id_db(&app_state.db, id)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn post_new_user(
    new_user: web::Json<CreateUser>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_user_db(&app_state.db, CreateUser::from(new_user))
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn update_user_details(
    app_state: web::Data<AppState>,
    update_user: web::Json<UpdateUser>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let id = params.into_inner();
    update_user_details_db(&app_state.db, id, UpdateUser::from(update_user),)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
)->Result<HttpResponse,MyError>{
    let id = params.into_inner();
    delete_user_db(&app_state.db, id)
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn post_login_request(
    login_user: web::Json<LoginUser>,
    app_state: web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    post_login_request_db(&app_state.db, LoginUser::from(login_user))
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

pub async fn post_reg_request(
    reg_user: web::Json<RegUser>,
    app_state: web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    post_reg_request_db(&app_state.db, RegUser::from(reg_user))
        .await
        .map(|user| HttpResponse::Ok().json(user))
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
        sqlx::query("TRUNCATE TABLE user1 RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await.unwrap();
        sqlx::query("TRUNCATE TABLE book RESTART IDENTITY CASCADE")
            .execute(&pool)
            .await.unwrap();
        sqlx::query("TRUNCATE TABLE record RESTART IDENTITY CASCADE")
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

    #[actix_rt::test]
    async fn test_get_all_users() -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0), db: pool });
        // let req = test::TestRequest::get().to_http_request();
        let resp = get_all_users(app_state).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case(1)]
    // #[case(4)]
    #[actix_rt::test]
    async fn test_get_user_by_id(#[case] id:i32) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0), db: pool });
        // let req = test::TestRequest::get().to_http_request();
        let resp = get_user_by_id(app_state,web::Path::from(id)).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case("Bob".to_string(), "123456".to_string(), false)]
    // #[case("".to_string(), "".to_string(), false)]
    #[actix_rt::test]
    async fn test_post_new_user(#[case] name:String,#[case] password:String,#[case] is_mana:bool) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        let new_user = CreateUser {
            name: name,
            password: password,
            is_mana: is_mana,
        };
        let resp = post_new_user(web::Json(new_user),app_state).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case(1,"NewName".to_string(), "111111".to_string(), false)]
    // #[case(1,"".to_string(), "".to_string(), false)]
    // #[case(66,"cscs".to_string(), "123456".to_string(), false)]
    #[actix_rt::test]
    async fn test_update_user_details(#[case] id:i32,#[case] name:String,#[case] password:String,#[case] is_mana:bool) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        let update_user = UpdateUser {
            name: Some(name),
            password: Some(password),
            is_mana: Some(is_mana),
        };

        let resp = update_user_details(app_state, web::Json(update_user),web::Path::from(id)).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case(1)]
    // #[case(66)]
    #[actix_rt::test]
    async fn test_delete_user(#[case] id:i32) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        let resp = delete_user(app_state, web::Path::from(id)).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case(1,"123456".to_string())]
    // #[case(1,"111111".to_string())]
    // #[case(66,"111111".to_string())]
    #[actix_rt::test]
    async fn test_post_login_request(#[case] id:i32,#[case] password:String) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        let login_user = LoginUser {
            id: id,
            password: password,
        };
        let resp = post_login_request(web::Json(login_user),app_state,).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    #[rstest]
    #[case("Bob".to_string(), "123456".to_string())]
    // #[case("".to_string(), "".to_string())]
    #[actix_rt::test]
    async fn test_post_reg_request(#[case] name:String,#[case] password:String) -> Result<(), MyError> {
        let pool = create_test_pool().await;
        let app_state = web::Data::new(AppState { health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),db: pool });
        let reg_user = RegUser {
            name: name,
            password: password,
        };
        let resp = post_reg_request(web::Json(reg_user),app_state,).await?;
        assert_eq!(resp.status(), StatusCode::OK);
        Ok(())
    }
    
}

