use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod database;
mod mutex_box;
mod walker;

use database::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/api/videos/all")]
async fn persdsf(db: web::Data<Database>) -> impl Responder {
    let pers = db.get_all_videos().await.unwrap();
    HttpResponse::Ok().json(pers)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Database::new().await;
    data.init().await.unwrap();
    walker::scan_folder(".".to_string(), 6, 6,data.clone()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(hello)
            .service(echo)
            .service(persdsf)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
