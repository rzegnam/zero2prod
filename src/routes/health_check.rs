use actix_web::{HttpResponse};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
