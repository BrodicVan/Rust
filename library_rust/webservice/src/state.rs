use sqlx::PgPool;
use std::sync::Mutex;


pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub db: PgPool,
}
/* 
impl AppState {
    pub async fn new() -> Result<Self, MyError> {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = PgPoolOptions::new()
            .max_connections(1)
            .connect(&db_url)
            .await?;
        Ok(Self {
            health_check_response: "OK".into(),
            visit_count: Mutex::new(0),
            db,
        })
    }
}
*/