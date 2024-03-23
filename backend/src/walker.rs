use std::os::unix::fs::MetadataExt;

use crate::database::Database;
use anyhow::Result;
use async_recursion::async_recursion;
use tokio::fs;
use video_walker::{setup_data::CollectionSetup, video::{Folder, Video}};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};

#[async_recursion]
pub async fn scan_folder(
    path: String,
    collection:CollectionSetup,
    folder: Folder,
    db: Database,
) -> Result<()> {
    println!("searching in folder:{}",path);
    let mut entries = fs::read_dir(path).await?;
    let mut futs = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let meta = entry.metadata().await?;
        if meta.is_dir() {
            let new_path = format!("{}", entry.path().to_str().unwrap());
            let new_folder=Folder{
                folder_id:to_base64(entry.ino()),
                collection_id:collection.id,
                folder_name:entry.file_name().to_str().unwrap().to_string(),
                super_folder_id:folder.folder_id.clone(),
            };
            db.insert_folder(new_folder.clone(), new_folder.folder_id.clone()).await?;
            futs.push(scan_folder(
                new_path,
                collection.clone(),
                new_folder,
                db.clone(),
            ));
        } else {
            
            
            let mut video = Video::default();
            video.folder_id = folder.folder_id.clone();
            video.collection_id=collection.id.clone();
            video.video_id =to_base64(entry.ino());
            video.title = entry.file_name().to_str().unwrap().to_string();
            video.filename = entry.file_name().to_str().unwrap().to_string();
            video.size = Some(meta.size());
            db.insert_video(video.clone(),video.video_id).await?;
        }
        println!(
            "{:?}/{:?}: {}",
            entry.path(),
            entry.file_name(),
            entry.ino()
        );
    }
    let mut handles = Vec::with_capacity(futs.len());
    for fut in futs {
        handles.push(tokio::spawn(fut));
    }

    for handle in handles {
        handle.await.unwrap()?;
    }

    Ok(())
}

fn to_base64(input:u64)->String
{
    URL_SAFE.encode(input.to_be_bytes())
}