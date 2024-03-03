use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use byteorder::{BigEndian, ReadBytesExt};

mod database;
mod walker;

use database::Database;
use video_walker::{setup_data::CollectionSetup, video::Folder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[get("/api/videos/all")]
async fn all_videos(db: web::Data<Database>) -> impl Responder {
    let pers = db.get_all_videos().await.unwrap();
    HttpResponse::Ok().json(pers)
}

#[get("/api/videos/path/{id}")]
async fn video_path(req: HttpRequest,db: web::Data<Database>) -> impl Responder {
    if let Some(id_str) = req.match_info().get("id") {
        if let Ok( mut id_vec)=URL_SAFE.decode(id_str.as_bytes()){
            if id_vec.len()>=8{
                id_vec.truncate(8);
                let mut id_slc=id_vec.as_slice();
                let id=id_slc.read_u64::<BigEndian>().unwrap();
                let vid=db.get_video(id).await.unwrap().unwrap();
                let path= db.get_path(vid).await.unwrap().display().to_string();
                return HttpResponse::Ok().body(path);
            }
        }
        
    }
    

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
    let start_foler=Folder{folder_id:1,collection_id:0,folder_name:"root".to_string(),super_folder_id:0};
    data.insert_folder(start_foler.clone(), start_foler.folder_id.clone()).await.unwrap();
    let collection=CollectionSetup{path:".".to_string(),file_extentions:Vec::new(),id:1};
    walker::scan_folder(".".to_string(), collection, start_foler,data.clone()).await.unwrap();
    let testvideo=data.get_video(13267142).await.unwrap().unwrap();
    let path=data.get_path(testvideo).await.unwrap();
    println!("{:?}",path);
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
