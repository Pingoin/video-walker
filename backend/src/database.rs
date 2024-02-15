use actix_web::{error, web, Error};
use r2d2_sqlite::SqliteConnectionManager;
use anyhow::Result;

use video_walker::video::Video;

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new() -> Self {
        // connect to SQLite DB
        let manager = SqliteConnectionManager::file("videowalker.db");
        let pool = Pool::new(manager).unwrap();
        Database { pool: pool }
    }

    pub async fn init(&self) -> Result<()> {

        let pool = self.pool.clone();

        let conn = web::block(move || pool.get())
            .await??;
        web::block(move || {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS videos (
            video_id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            folder_id INTEGER NOT NULL,
            description TEXT,
            year INTEGER,
            collection_id INTEGER,
            filename TEXT NOT NULL,
            size INTEGER
        )",
                (), // empty list of parameters.
            )
        })
        .await??;
        Ok(())
    }

    pub async fn get_all_videos(&self) -> Result<Vec<Video>> {
        let mut result = Vec::new();

        let pool = self.pool.clone();

        let conn = web::block(move || pool.get())
            .await??;
        web::block(move || {
            let mut stmt = conn.prepare("SELECT * FROM videos")?;
            let person_iter = stmt
                .query_map([], |row| {
                    Ok({
                        let mut vid = Video::default();
                        vid.video_id = row.get(0)?;
                        vid.title = row.get(1)?;
                        vid.folder_id = row.get(2)?;
                        vid.description = row.get(3)?;
                        vid.year = row.get(4)?;
                        vid.collection_id = row.get(5)?;
                        vid.filename = row.get(6)?;
                        vid.size = row.get(7)?;
                        vid
                    })
                })?;

            for person in person_iter {
                if let Ok(pers) = person {
                    result.push(pers)
                }
            }

            Ok(result)
        })
        .await?
    }

    pub async fn insert_video(&self, video: Video)->Result<()> {
        let conn = self.pool.get()?;
        web::block(move || {
            conn.execute(
                "
        INSERT INTO videos (
            video_id, 
            title, 
            folder_id ,
            description,
            year,
            collection_id,
            filename,
            size
        ) VALUES (?1, ?2,?3,?4,?5,?6,?7,?8)",
                (
                    video.video_id,
                    video.title,
                    video.folder_id,
                    video.description,
                    video.year,
                    video.collection_id,
                    video.filename,
                    video.size,
                ),
            )
        })
        .await??;
    Ok(())
    }
}
