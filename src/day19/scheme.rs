use sqlx::PgPool;

pub async fn create_tables(pool: &PgPool) {
    let sql = include_str!("scheme.sql");
    sqlx::query(sql).execute(pool).await.unwrap();
}
