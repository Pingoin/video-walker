use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
mod database;
mod mutex_box;

use database::Database;
use video_walker::video::Video;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Hoden!")
}

#[get("/api/videos/all")]
async fn persdsf(db: web::Data<Database>) -> impl Responder {
    let pers = db.get_all_videos().await;

    let string = format!("{:?}", pers);

    HttpResponse::Ok().body(string)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let data = Database::new().await;
    data.init().await.unwrap();
    let mut testvid=Video::default();

    testvid.video_id=rng.gen();
    testvid.title="TestTitle".to_string();

    data.insert_video(testvid).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(data.clone()))
            .service(hello)
            .service(echo)
            .service(persdsf)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
