use crate::error::MyError;
use crate::models::{UserRegisterForm,UserResponse};
use actix_web::http::header::ContentType;
use actix_web::{web,Error,HttpResponse,Result};
use serde_json::json;

pub async fn get_all_users(tmpl: web::Data<tera::Tera>)
->Result<HttpResponse,Error>{
    let awc_client = awc::Client::default();

    let res = awc_client
        .get("http://localhost:3000/users/")
        .send()
        .await
        .unwrap()
        .json::<Vec<UserResponse>>()
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("error","");
    ctx.insert("users",&res);

    let s = tmpl
        .render("users.html",&ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_form(tmpl: web::Data<tera::Tera>)
->Result<HttpResponse,Error>{
    let mut ctx = tera::Context::new();
    ctx.insert("error","");
    ctx.insert("current_name","");
    ctx.insert("current_password","");
    // ctx.insert("current_is_mana","");
    let s = tmpl
        .render("register.html",&ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))

}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<UserRegisterForm>,
)->Result<HttpResponse,Error>{
    
    let mut ctx = tera::Context::new();
    let s;
    if params.name == ""||params.password==""
    {
        ctx.insert("error", "用户名或密码不能为空");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_password", &params.password);
        ctx.insert("current_is_mana","false");
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template error".into()))?;
        // s = String::from("value");
    }
    else
    {
        println!("...........");
        let new_user = json!({
            
            "is_mana":false,
            "name":&params.name,
            "password":&params.password,
        });
        let awc_client = awc::Client::default();

        let res = awc_client
            .post("http://localhost:3000/users/")
            .send_json(&new_user)
            .await
            .unwrap()
            .body()
            .await?;
        
        let user_response:UserResponse = serde_json::from_str(&std::str::from_utf8(&res)?)?;
        
        s = format!("注册成功,你的id是:{},请牢记",user_response.id);
    }
    Ok(HttpResponse::Ok().content_type(ContentType::json()).body(s))
}