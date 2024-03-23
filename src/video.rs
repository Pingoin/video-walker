use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Default,Clone,TS)]
#[ts(export)]
pub struct Video {
    pub video_id: String,
    pub title: String,
    pub folder_id: String,
    pub description: String,
    pub year: Option<u16>,
    pub collection_id:u8,
    pub filename: String,
    pub size: Option<u64>,
}

impl FolderMember for Video{
    fn get_super_folder_id(&self)->String {
        self.folder_id.clone()
    }
    
    fn get_name(&self)->String {
        self.filename.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Default,Clone,TS)]
#[ts(export)]
pub struct Folder {
    pub folder_id: String,
    pub folder_name: String,
    pub collection_id:u8,
    pub super_folder_id: String,
}
impl FolderMember for Folder {
    fn get_super_folder_id(&self)->String {
        self.super_folder_id.clone()
    }
    
    fn get_name(&self)->String {
        self.folder_name.clone()
    }
}

pub trait FolderMember {
    fn get_super_folder_id(&self)->String;
    fn get_name(&self)->String;

}