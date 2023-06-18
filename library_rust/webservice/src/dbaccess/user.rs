use crate::error::MyError;
use crate::models::user::{User, CreateUser, UpdateUser, LoginUser, RegUser};
use sqlx::postgres::PgPool;

//查询所有用户
pub async fn get_all_users_db(pool: &PgPool)->Result<Vec<User>,MyError>
{
    let rows = sqlx::query!("SELECT id,name,password,is_mana FROM user1 order by id")
        .fetch_all(pool)
        .await?;
    let users: Vec<User> = rows
        .iter()
        .map(|r| User{
            id: r.id,
            name: r.name.clone(),
            password: r.password.clone(),
            is_mana: r.is_mana,
        })
        .collect();
    match users.len(){
        0 => Err(MyError::NotFound("No users found".into())),
        _ => Ok(users),
    }
}

//查询指定用户
pub async fn get_user_by_id_db(pool: &PgPool,id:i32)->Result<User,MyError>
{
    let row = sqlx::query!("SELECT id,name,password,is_mana FROM user1 where id=$1",id)
        .fetch_one(pool)
        .await
        .map(|r| User{
            id:r.id,
            name:r.name,
            password:r.password,
            is_mana:r.is_mana,
        })
        .map_err(|_err| MyError::NotFound("User Id not found".into()))?;
        Ok(row)
}

//插入用户
pub async fn post_new_user_db(
    pool: &PgPool,
    new_user: CreateUser,
) -> Result<User, MyError> {
    let row = sqlx::query_as!(
        User,
        r#"INSERT INTO user1 (name,password,is_mana)
        VALUES ($1, $2, $3)
        RETURNING id, name, password, is_mana"#,
        new_user.name,new_user.password,new_user.is_mana
        )
    .fetch_one(pool)
    .await?;

    Ok(row)//Ok(User?
}

//更新用户
pub async fn update_user_details_db(
    pool: &PgPool,
    id: i32,
    update_user: UpdateUser,
) -> Result<User, MyError> {
    let current_user_row = sqlx::query_as!(
        User,
        "SELECT * FROM user1 where id=$1",
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("User Id not found".into()))?;

    let name: String = if let Some(name) = update_user.name {
        name
    } else {
        current_user_row.name
    };
    let password: String = if let Some(password) = update_user.password {
        password
    } else {
        current_user_row.password
    };
    let is_mana: bool = if let Some(is_mana) = update_user.is_mana {
        is_mana
    } else {
        current_user_row.is_mana
    };

    let user_row = sqlx::query_as!(
        User,
        "UPDATE user1 SET name = $1, password = $2, is_mana = $3
            where id = $4
            RETURNING id, name, password, is_mana",
        name,
        password,
        is_mana,
        id
    )
    .fetch_one(pool)
    .await;
    if let Ok(user) = user_row {
        Ok(user)
    } else {
        Err(MyError::NotFound("User id not found".into()))
    }
}

//删除用户
pub async fn delete_user_db(pool: &PgPool, id: i32) -> Result<String, MyError> {
    let user_row = sqlx::query!(
        "DELETE FROM user1 where id=$1",
        id,
    )
    .execute(pool)
    .await?;
    Ok(format!("DeletedI{:?}record", user_row.rows_affected()))
}

//判断登录
pub async fn post_login_request_db(pool: &PgPool, login_user:LoginUser)->Result<User,MyError>{
    let row = sqlx::query!("SELECT id,name,password,is_mana FROM user1 where id=$1",login_user.id)
        .fetch_one(pool)
        .await
        .map(|r| User{
            id:r.id,
            name:r.name,
            password:r.password,
            is_mana:r.is_mana,
        })
        .map_err(|_err| MyError::NotFound("User Id not found".into()))?;
    if row.password==login_user.password
    {
        Ok(row)
    }
    else{
        Err(MyError::InvalidInput("Id or password wrong".into()))
    }
    
}

//注册
pub async fn post_reg_request_db(
    pool: &PgPool,
    reg_user: RegUser,
) -> Result<User, MyError> {
    let row = sqlx::query_as!(
        User,
        r#"INSERT INTO user1 (name,password,is_mana)
        VALUES ($1, $2, FALSE)
        RETURNING id, name, password, is_mana"#,
        reg_user.name,reg_user.password
        )
    .fetch_one(pool)
    .await?;

    Ok(row)//Ok(User?
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

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

    #[actix_rt::test]
    async fn test_get_all_users_db() -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let users = get_all_users_db(&pool).await?;
        assert_eq!(users.len(),3);
        Ok(())
    }

    #[rstest]
    #[case(1)]
    #[case(4)]
    #[actix_rt::test]
    async fn test_get_user_by_id_db(#[case] id:i32) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let user = get_user_by_id_db(&pool, id).await;
        if id==1
        {
            assert_eq!(user?.name, "李应东");
        }
        else {
            assert!(user.is_err());
        }
        Ok(())
    }
    #[rstest]
    #[case("Bob".to_string(), "123456".to_string(), false)]
    #[case("".to_string(), "".to_string(), false)]
    #[actix_rt::test]
    async fn test_post_new_user_db(#[case] name:String,#[case] password:String,#[case] is_mana:bool) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let name_ref = &name;
        let new_user = CreateUser {
            name: name_ref.to_string(),
            password: password,
            is_mana: is_mana,
        };
        let user = post_new_user_db(&pool, new_user).await;
        if name_ref.len()==0
        {
            assert!(user.is_err());
        }
        else {
            assert_eq!(user?.name, name_ref.to_string());
        }
        Ok(())
    }

    #[rstest]
    #[case(1,"NewName".to_string(), "111111".to_string(), false)]
    #[case(1,"".to_string(), "".to_string(), false)]
    #[case(66,"cscs".to_string(), "123456".to_string(), false)]
    #[actix_rt::test]
    async fn test_update_user_details_db(#[case] id:i32,#[case] name:String,#[case] password:String,#[case] is_mana:bool) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let name_ref=&name;
        let update_user = UpdateUser {
            name: Some(name_ref.to_string()),
            password: Some(password),
            is_mana: Some(is_mana),
        };
        let user = update_user_details_db(&pool, id, update_user).await;
        if id==1 {
            if name_ref.len()==0 {
                assert!(user.is_err());
            }
            else {
                assert_eq!(user?.name,"NewName");
            }
        }
        else {
            assert!(user.is_err());
        }
        Ok(())
    }

    #[rstest]
    #[case(1)]
    #[case(66)]
    #[actix_rt::test]
    async fn test_delete_user_db(#[case] id:i32) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let result = delete_user_db(&pool, id).await?;
        let expected_message;
        if id==1 {
            expected_message = format!("DeletedI{:?}record", 1);
        }
        else {
            expected_message = format!("DeletedI{:?}record", 0);
        }
        assert_eq!(result, expected_message);
        Ok(())
    }

    #[rstest]
    #[case(1,"123456".to_string())]
    #[case(1,"111111".to_string())]
    #[case(66,"111111".to_string())]
    #[actix_rt::test]
    async fn test_post_login_request_db(#[case] id:i32,#[case] password:String) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let password_ref=&password;
        let login_user = LoginUser {
            id: id,
            password: password_ref.to_string(),
        };
        let user = post_login_request_db(&pool, login_user).await;
        if id==1
        {
            if password_ref=="123456"
            {
                assert_eq!(user?.name,"李应东");
            }
            else {
                assert!(user.is_err());
            }
        }
        else {
            assert!(user.is_err());
        }
        Ok(())
    }

    #[rstest]
    #[case("Bob".to_string(), "123456".to_string())]
    #[case("".to_string(), "".to_string())]
    #[actix_rt::test]
    async fn test_post_reg_request_db(#[case] name:String,#[case] password:String) -> Result<(), MyError> {
        let pool = get_test_db_pool().await?;
        let name_ref = &name;
        let reg_user = RegUser {
            name: name_ref.to_string(),
            password: password,
        };
        let user = post_reg_request_db(&pool, reg_user).await;
        if name_ref.len()==0
        {
            assert!(user.is_err());
        }
        else {
            assert_eq!(user?.name, name_ref.to_string());
        }
        Ok(())
    }
}
