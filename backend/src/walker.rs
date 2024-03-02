use std::os::unix::fs::MetadataExt;

use crate::database::Database;
use anyhow::Result;
use async_recursion::async_recursion;
use tokio::fs;
use video_walker::video::{Video,Folder};

#[async_recursion]
pub async fn scan_folder(
    path: String,
    collection_id: u8,
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
                folder_id:entry.ino(),
                folder_name:entry.file_name().to_str().unwrap().to_string(),
                super_folder_id:folder.folder_id,
            };
            db.insert_folder(new_folder.clone(), new_folder.folder_id.clone()).await?;
            futs.push(scan_folder(
                new_path,
                collection_id,
                new_folder,
                db.clone(),
            ));
        } else {
            
            
            let mut video = Video::default();
            video.folder_id = folder.folder_id;

            video.video_id = entry.ino();
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
