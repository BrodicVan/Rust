use actix_web::web;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug,Clone)]
pub struct User{
    pub id:i32,
    pub name:String,
    pub password:String,
    pub is_mana:bool,
}

#[derive(Deserialize,Debug,Clone)]
pub struct CreateUser{
    pub name:String,
    pub password:String,
    pub is_mana:bool,
}

#[derive(Deserialize,Debug,Clone)]
pub struct UpdateUser{
    pub name:Option<String>,
    pub password:Option<String>,
    pub is_mana:Option<bool>,
}

#[derive(Deserialize,Debug,Clone)]
pub struct RegUser{
    pub name:String,
    pub password:String,
}

#[derive(Deserialize,Debug,Clone)]
pub struct LoginUser{
    pub id:i32,
    pub password:String,
}

impl From<web::Json<CreateUser>> for CreateUser{
    fn from(new_user: web::Json<CreateUser>) -> Self {
        CreateUser { 
            name: new_user.name.clone(), 
            password: new_user.password.clone(), 
            is_mana: new_user.is_mana.clone(), 
        }        
    }
}

impl From<web::Json<UpdateUser>> for UpdateUser{
    fn from(update_user: web::Json<UpdateUser>) -> Self {
        UpdateUser { 
            name: update_user.name.clone(), 
            password: update_user.password.clone(), 
            is_mana: update_user.is_mana.clone(), 
        }        
    }
}

impl From<web::Json<LoginUser>> for LoginUser{
    fn from(login_user: web::Json<LoginUser>) -> Self {
        LoginUser { 
            id: login_user.id.clone(), 
            password: login_user.password.clone(),
        }        
    }
}

impl From<web::Json<RegUser>> for RegUser{
    fn from(reg_user: web::Json<RegUser>) -> Self {
        RegUser { 
            name: reg_user.name.clone(), 
            password: reg_user.password.clone(),
        }        
    }
}