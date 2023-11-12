use serde::{Deserialize, Serialize};
use crate::ClientError;

pub trait Validator: Serialize + Send {
    fn validate(&self) -> Result<(), ClientError>;
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Empty{}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ArangoKeys {
    #[serde(rename="_id")]
    pub id: String,
    
    #[serde(rename="_key")]
    pub key: String,           

    #[serde(rename="_rev")]
    pub rev: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ArangoEdgeKeys {
    #[serde(rename="_to")]
    pub to: String,             
    #[serde(rename="_from")]
    pub from: String,           
}
