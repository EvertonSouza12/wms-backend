use actix_web::{web, App, HttpServer};
use std::env;

mod db;
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://wms_user:wms_password@db:3306/wms_db".to_string());

    // Inicializa o banco de dados e cria as tabelas via db.rs
    let pool = db::init_db(&database_url)
        .await
        .expect("Falha ao inicializar o banco de dados MariaDB");

    println!("Servidor rodando na porta 8080...");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::get_products)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}