use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
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
