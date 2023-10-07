use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};
use uuid::Uuid;
use log::info;

async fn root() -> impl Responder {
    info!("Serving root...");
    HttpResponse::Ok().body(r#"<a href="/start">Start the Infinite Loop</a>"#)
}

async fn start() -> impl Responder {
    let new_uuid = Uuid::new_v4();
    info!("Serving start with UUID: {}", new_uuid);
    HttpResponse::Ok().body(format!(r#"<a href="/loop/{}">Go to {}</a>"#, new_uuid, new_uuid))
}

async fn infinite_loop(_: web::Path<Uuid>) -> impl Responder {
    let new_uuid = Uuid::new_v4();
    info!("Generating new UUID link: {}", new_uuid);
    HttpResponse::Ok().body(format!(r#"<a href="/loop/{}">Go to {}</a>"#, new_uuid, new_uuid))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // Add logger middleware to print request info
            .route("/", web::get().to(root))
            .route("/start", web::get().to(start))
            .route("/loop/{uuid}", web::get().to(infinite_loop))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
