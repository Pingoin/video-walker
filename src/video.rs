use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Default,Clone,TS)]
#[ts(export)]
pub struct Video {
    pub video_id: u64,
    pub title: String,
    pub folder_id: u64,
    pub description: String,
    pub year: Option<u16>,
    pub collection_id:u8,
    pub filename: String,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Default,Clone,TS)]
#[ts(export)]
pub struct Folder {
    pub folder_id: u64,
    pub folder_name: String,
    pub super_folder_id: u64,
}