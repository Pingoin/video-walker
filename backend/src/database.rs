use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use actix_web::web;
use anyhow::Result;

use video_walker::video::{Folder, Video};

#[derive(Clone)]
pub struct Database {
    videos: Arc<Mutex<BTreeMap<u64,Video>>>,
    folders: Arc<Mutex<BTreeMap<u64,Folder>>>,
}

impl Database {
    pub async fn new() -> Self {
        Database { videos:Arc::new(Mutex::new(BTreeMap::new() )),folders:Arc::new(Mutex::new(BTreeMap::new() ))}
    }

    pub async fn init(&self) -> Result<()> {
        Ok(())
    } 

    pub async fn get_all_videos(&self) -> Result<Vec<Video>> {
            let videos=Arc::clone(&self.videos);
            web::block(move||{
                let videos=videos.lock().unwrap();
                let vals= Vec::from_iter(videos.values().cloned());
                Ok(vals)
            }).await?
    }

    pub async fn insert_video(&self, video: Video,id:u64)->Result<()> {
        let videos=Arc::clone(&self.videos);
        web::block(move ||{
            let mut videos=videos.lock().unwrap();
            (*videos).insert(id, video);
            Ok(())
        }).await?
    }

    pub async fn get_all_folders(&self) -> Result<Vec<Folder>> {
        let folders=Arc::clone(&self.folders);
        web::block(move||{
            let folders =folders.lock().unwrap();
            let vals= Vec::from_iter(folders.values().cloned());
            Ok(vals)
        }).await?
}

    pub async fn insert_folder(&self, folder: Folder,id:u64)->Result<()> {
        let folders=Arc::clone(&self.folders);
        web::block(move ||{
            let mut folders =folders.lock().unwrap();
            (*folders).insert(id, folder);
            Ok(())
        }).await?
    }
}
