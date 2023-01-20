use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct UserRegisterForm{
    pub name:String,
    pub password:String,
    // pub is_mana:bool,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct UserResponse{
    pub id:i32,
    pub name:String,
    pub password:String,
    pub is_mana:bool,
}