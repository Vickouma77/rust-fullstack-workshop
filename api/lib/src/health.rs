use actix_web::get;
#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}
#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;
    result.unwrap_or_else(|e| format!("Error: {:?}", e))
}