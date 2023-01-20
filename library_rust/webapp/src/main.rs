use serde_json::json;


fn main() {
    println!("Hello, world!");
}
pub async fn tmp(){
    
    let new_user = json!({
            
        "is_mana":false,
        "name":"nihao",
        "password":"nihao",
    });
    let awc_client = awc::Client::default();

    let res = awc_client
        .post("http://localhost:3000/users/")
        .send_json(&new_user)
        .await
        .unwrap()
        .body()
        .await;
}