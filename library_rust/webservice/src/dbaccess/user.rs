use crate::error::MyError;
use crate::models::user::{User, CreateUser, UpdateUser, LoginUser, RegUser};
use sqlx::postgres::PgPool;

//查询所有用户
pub async fn get_all_users_db(pool: &PgPool)->Result<Vec<User>,MyError>
{
    let rows = sqlx::query!("SELECT id,name,password,is_mana FROM user1")
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
pub async fn get_user_byId_db(pool: &PgPool,id:i32)->Result<User,MyError>
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
    Ok(format!("DeletedI{:?}record", user_row))
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