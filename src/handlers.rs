use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;
use crate::models::Product;

#[get("/api/products")]
pub async fn get_products(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Product>(
        "SELECT id, sku, name, description, barcode, category, unit, min_stock FROM products"
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => {
            eprintln!("Erro ao buscar produtos: {:?}", err);
            HttpResponse::InternalServerError().body("Erro ao buscar lista de produtos")
        }
    }
}