use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Default,Clone,TS)]
#[ts(export)]
pub struct SetupData{
    collections:Vec<CollectionSetup>
}

#[derive(Debug, Serialize, Deserialize, Default,Clone,TS)]
#[ts(export)]
pub struct CollectionSetup{
    pub path:String,
    pub file_extentions:Vec<String>,
    pub id:u8,
}