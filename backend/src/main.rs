use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod database;
mod walker;

use database::Database;
use video_walker::video::Folder;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/api/videos/all")]
async fn all_videos(db: web::Data<Database>) -> impl Responder {
    let pers = db.get_all_videos().await.unwrap();
    HttpResponse::Ok().json(pers)
}

#[get("/api/folders/all")]
async fn all_folders(db: web::Data<Database>) -> impl Responder {
    let pers = db.get_all_folders().await.unwrap();
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
    let start_foler=Folder{folder_id:0,folder_name:"root".to_string(),super_folder_id:0};
    data.insert_folder(start_foler.clone(), start_foler.folder_id.clone()).await.unwrap();
    walker::scan_folder(".".to_string(), 6, start_foler,data.clone()).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(hello)
            .service(echo)
            .service(all_videos)
            .service(all_folders)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
