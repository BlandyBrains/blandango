use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NameResponse {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct IdResponse {
    pub id: String,
    pub error: bool,
    pub code: u8
}

#[derive(Deserialize)]
pub struct Response<T> {
    pub error: bool,
    pub code: u8,
    pub result: T
}

#[derive(Deserialize)]
pub struct FlatResponse<T> {
    pub error: bool,
    pub code: u8,
    
    #[serde(flatten)]
    pub result: T
}