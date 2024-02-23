use std::os::unix::fs::MetadataExt;

use crate::database::Database;
use anyhow::Result;
use async_recursion::async_recursion;
use tokio::fs;
use video_walker::video::Video;

#[async_recursion]
pub async fn scan_folder(
    path: String,
    collection_id: u8,
    folder_id: u64,
    db: Database,
) -> Result<()> {
    println!("searching in folder:{}",path);
    let mut entries = fs::read_dir(path).await?;
    let mut futs = Vec::new();
    while let Some(entry) = entries.next_entry().await? {
        let meta = entry.metadata().await?;
        if meta.is_dir() {
            let new_path = format!("{}", entry.path().to_str().unwrap());
            futs.push(scan_folder(
                new_path,
                collection_id,
                entry.ino(),
                db.clone(),
            ));
        } else {
            
            
            let mut video = Video::default();
            video.folder_id = folder_id;

            video.video_id = entry.ino();
            video.title = entry.file_name().to_str().unwrap().to_string();
            video.filename = entry.file_name().to_str().unwrap().to_string();
            video.size = Some(meta.size());
            db.insert_video(video).await?;
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
