use actix_web::web;
use anyhow::Result;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use video_walker::video::{Folder, FolderMember, Video};

#[derive(Clone)]
pub struct Database {
    videos: Arc<Mutex<BTreeMap<String, Video>>>,
    folders: Arc<Mutex<BTreeMap<String, Folder>>>,
}

impl Database {
    pub async fn new() -> Self {
        Database {
            videos: Arc::new(Mutex::new(BTreeMap::new())),
            folders: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub async fn init(&self) -> Result<()> {
        Ok(())
    }

    pub async fn get_all_videos(&self) -> Result<Vec<Video>> {
        let videos = Arc::clone(&self.videos);
        web::block(move || {
            let videos = videos.lock().unwrap();
            let vals = Vec::from_iter(videos.values().cloned());
            Ok(vals)
        })
        .await?
    }

    pub async fn insert_video(&self, video: Video, id: String) -> Result<()> {
        let videos = Arc::clone(&self.videos);
        web::block(move || {
            let mut videos = videos.lock().unwrap();
            (*videos).insert(id, video);
            Ok(())
        })
        .await?
    }

    pub async fn get_all_folders(&self) -> Result<Vec<Folder>> {
        let folders = Arc::clone(&self.folders);
        web::block(move || {
            let folders = folders.lock().unwrap();
            let vals = Vec::from_iter(folders.values().cloned());
            Ok(vals)
        })
        .await?
    }

    pub async fn insert_folder(&self, folder: Folder, id: String) -> Result<()> {
        let folders = Arc::clone(&self.folders);
        web::block(move || {
            let mut folders = folders.lock().unwrap();
            (*folders).insert(id, folder);
            Ok(())
        })
        .await?
    }

    pub async fn get_video(&self, video_id:String)->Result<Option<Video>>{
        let videos = Arc::clone(&self.videos);
        web::block(move || {
            let videos = videos.lock().unwrap();
            let vals = videos.get(&video_id);
            Ok(if let Some(v) =vals  {
                Some(v.clone())
            }else{None})
        })
        .await?
    }

    pub async fn get_path<T>(&self, item: T) -> Result<PathBuf>
    where
        T: FolderMember + Send,
    {
        let folders = Arc::clone(&self.folders);
        let mut path_items = Vec::new();
        let mut seach_folder_id = item.get_super_folder_id();
        path_items.push(item.get_name());
        web::block(move || {
            while seach_folder_id.len()>0{
                let folders = folders.lock().unwrap();
                if let Some(f) = (*folders).get(&seach_folder_id)  {
                    path_items.push(f.folder_name.clone());
                    seach_folder_id=f.super_folder_id.clone();
                }
            }
            path_items.reverse();
            let path: PathBuf = path_items.iter().collect();
            Ok(path)
        })
        .await?
    }
}
