use std::{fmt, error, time::Duration};

use base64::{Engine as _, engine::general_purpose};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use hyper::{
    client::HttpConnector, Response, Body, body::Buf, 
    http::{self, request::Builder, uri::InvalidUri}, 
    Request, Method, HeaderMap
};
use log::info;
use crate::{api::{ApiError, self, API}, DocumentQueryParams};



#[derive(Debug)]
pub enum ClientError {
    Client(hyper::Error),
    Connection(http::Error),
    Api(ApiError),
    Uri(InvalidUri),
    Params(serde_url_params::Error),
    JsonError(serde_json::Error),
    Insertion(String),
    Validation(String)
}

pub type Result<T> = std::result::Result<T, ClientError>;

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "client error {:#?}", self)
    }
}

impl error::Error for ClientError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::Uri(ref e) => Some(e),
            Self::Client(ref e) => Some(e),
            Self::Connection(ref e) => Some(e),
            Self::Api(ref e) => Some(e),
            Self::Params(ref e) => Some(e),
            Self::JsonError(ref e) => Some(e),
            // Self::Insertion(ref e) => Some(e),
            // Self::Validation(ref e) => Some(e),
            _ => None,
        }
    }
}

impl ClientError {
    pub fn validation_error(message: String) -> Self {
        Self::Validation(message)
    }

    pub fn insertion_error(message: String) -> Self {
        Self::Insertion(message)
    }
}

impl From<hyper::Error> for ClientError{
    fn from(value: hyper::Error) -> ClientError {
        return ClientError::Client(value);
    }
}

impl From<serde_json::Error> for ClientError{
    fn from(value: serde_json::Error) -> ClientError {
        return ClientError::JsonError(value);
    }
}

impl From<http::uri::InvalidUri> for ClientError{
    fn from(value: http::uri::InvalidUri) -> Self {
        return ClientError::Uri(value);
    }
}

impl From<ApiError> for ClientError{
    fn from(value: ApiError) -> ClientError {
        return ClientError::Api(value);
    }
}

impl From<http::Error> for ClientError {
    fn from(value: http::Error) -> ClientError {
        return ClientError::Connection(value);
    }
}

impl From<serde_url_params::Error> for ClientError {
    fn from(value: serde_url_params::Error) -> ClientError {
        return ClientError::Params(value);
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename(deserialize = "database"))]
pub struct Config {
    pub host: String,
    pub database: String,
    pub user: String,
    pub password: String,
}

impl Config {
    fn to_base(&self) -> String {
        return format!("{0}/_db/{1}/", self.host, self.database);
    }
}

pub (crate) trait CollectionRouter {
    fn base() -> String {
        return String::from(api::API::Collection(api::Collection::Base));
    }
    fn base_collection(collection_name: &str) -> String {
        return format!("{}/{}", String::from(api::API::Collection(api::Collection::Base)), collection_name);
    }
    fn base_with_params<P: Serialize>(params: P) -> Result<String> {
        let encoded_params: String = serde_url_params::to_string(&params)?;
        Ok(format!("{}?{}", Self::base(), encoded_params))
    }
    fn extension(collection_name: &str, extension: API) -> String {
        return format!("{}/{}", Self::base_collection(collection_name), String::from(extension));
    }
    fn extension_with_params<P: Serialize>(collection_name: &str, endpoint: API, params: P) -> Result<String> {
        let encoded_params: String = serde_url_params::to_string(&params)?;
        Ok(format!("{}/{}?{}", 
            Self::base_collection(collection_name),
            String::from(endpoint),
            encoded_params
        ))
    }
    fn index(collection_name: &str) -> String {
        return format!("{}?collection={}", String::from(api::API::Index(api::Index::Base)), collection_name);
    }

}

pub (crate) trait DatabaseRouter {
    fn base() -> String {
        return String::from(api::API::Database(api::Database::Base));
    }
    fn current() -> String {
        return format!("{}/{}", 
            String::from(api::API::Database(api::Database::Base)), 
            String::from(api::API::Database(api::Database::Current)));
    }
    fn user() -> String {
        return format!("{}/{}", 
            String::from(api::API::Database(api::Database::Base)), 
            String::from(api::API::Database(api::Database::User)));
    }    
}

pub (crate) trait DocumentRouter {
    fn base(collection_name: &str) -> String {
        return format!("{}/{}", String::from(api::API::Document(api::Document::Base)), collection_name);
    }
    fn base_with_params(collection_name: &str, params: DocumentQueryParams) -> Result<String> {
        let encoded_params: String = serde_url_params::to_string(&params)?;
        Ok(format!("{}?{}", 
            Self::base(collection_name),
            encoded_params
        ))
    }
    fn key(collection_name: &str, key: &str) -> String {
        return format!("{}/{}", Self::base(collection_name), key);
    }
    fn key_with_params(collection_name: &str, key: &str, params: DocumentQueryParams) -> Result<String> {
        let encoded_params: String = serde_url_params::to_string(&params)?;
        Ok(format!("{}/{}?{}", Self::base(collection_name), key, encoded_params))
    } 
}


pub (crate) trait QueryRouter {
    fn cursor(id: &Option<String>) -> String {
        return match id {
            Some(batch_id) => {
                format!("{}/{}", String::from(API::Query(crate::api::Query::Cursor)), batch_id)
            },
            None => {
                String::from(API::Query(crate::api::Query::Cursor))
            }
        }        
    }

    fn cache() -> String {
        return String::from(API::Query(crate::api::Query::Cache));
    }
    fn cache_entries() -> String {
        return format!("{}/{}", String::from(API::Query(crate::api::Query::Cache)), String::from(API::Query(crate::api::Query::Entries)));
    }
    fn cache_properties() -> String {
        return format!("{}/{}", String::from(API::Query(crate::api::Query::Cache)), String::from(API::Query(crate::api::Query::Properties)));
    }
    fn explain() -> String {
        return String::from(API::Query(crate::api::Query::Explain));
    }
    fn query() -> String {
        return String::from(API::Query(crate::api::Query::Query));
    }    
    fn running() -> String {
        return format!("{}/{}", String::from(API::Query(crate::api::Query::Query)), String::from(API::Query(crate::api::Query::Current)));
    }
    fn kill(query_id: &str) -> String {
        return format!("{}/{}", String::from(API::Query(crate::api::Query::Query)), query_id);
    }
    fn slow() -> String {
        return format!("{}/{}", String::from(API::Query(crate::api::Query::Query)), String::from(API::Query(crate::api::Query::Slow)));
    }   
}


pub (crate) trait IndexRouter {
    fn base_as_query(collection_name: &str) -> String {
        return format!("{}?collection={}", String::from(api::API::Index(crate::api::Index::Base)), collection_name);
    }

    fn base_as_path(collection_name: &str) -> String {
        return format!("{}/{}", String::from(api::API::Index(crate::api::Index::Base)), collection_name);
    }
}

#[derive(Clone)]
pub (crate) struct Router {}

impl CollectionRouter for Router{}
impl DatabaseRouter for Router{}
impl DocumentRouter for Router{}
impl QueryRouter for Router{}
impl IndexRouter for Router{}

#[derive(Clone)]
pub struct Client {
    host: String,
    database: String,
    secret: String,
    client: hyper::Client<HttpConnector>
}

impl Client {    
    pub fn new(config: &Config) -> Self {
        let client: hyper::client::Client<HttpConnector> = hyper::Client::builder()
            .set_host(true)
            // adjusted here when troubleshooting a production issue with ubuntu 22.04
            // when set to None the system will not recycle the connection.
            .pool_idle_timeout(Duration::from_millis(100))
            .http2_keep_alive_timeout(Duration::from_secs(0))
            .build_http();

        let up: String = format!("{}:{}", config.user.clone(), config.password.clone());
        let b64: String = general_purpose::STANDARD.encode(up.as_bytes());

        println!("secret: {:#?}", b64);

        Self{
            host: config.host.clone(),
            database: config.database.clone(),
            secret: b64,
            client
        }
    }

    fn to_base(&self) -> String {
        return format!("{0}/_db/{1}/", self.host, self.database);
    }

    fn apply_authentication(&self, headers: &mut HeaderMap) {
        headers.insert("Authorization", format!("Basic {}", self.secret).parse().unwrap());
    }

    pub async fn head<T: DeserializeOwned>(&self, endpoint: String) -> Result<T> {
        let url: String = format!("{}{}", self.to_base(), endpoint);

        let mut builder: Builder = Request::head(url);
        {   
            match builder.headers_mut() {
                Some(h) => self.apply_authentication(h),
                _ => {}
            }
        }
        let request: Request<Body> = builder.body(Body::from(""))?;
        let res: Response<Body> = self.client.request(request).await?;
        self.handler(res).await
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: String) -> Result<T> {
        let url: String = format!("{}{}", self.to_base(), endpoint);

        info!("REQUEST: {:#?}", url);

        println!("GET {:#?}", url);

        let mut builder: Builder = Request::get(url);
        {   
            match builder.headers_mut() {
                Some(h) => self.apply_authentication(h),
                _ => {}
            }
        }
        let request: Request<Body> = builder.body(hyper::Body::default())?;
        let res: Response<Body> = self.client.request(request).await?;
        self.handler(res).await
    }

    pub async fn post<T: DeserializeOwned, K: Serialize>(&self, endpoint: String, data: &K) -> Result<T>{
        let url: String = format!("{}{}", self.to_base(), endpoint);

        let mut builder: Builder = Request::builder()
            .method(Method::POST)
            .uri(url);
        {   
            match builder.headers_mut() {
                Some(h) => self.apply_authentication(h),
                _ => {}
            }
        }
        let request: Request<Body> = builder.body(Body::from(serde_json::to_string(&data)?))?;
        let res: Response<Body> = self.client.request(request).await?;
        self.handler(res).await
    }

    pub async fn patch<T: DeserializeOwned, K: Serialize>(&self, endpoint: String, data: &K) -> Result<T>{
        let url: String = format!("{}{}", self.to_base(), endpoint);

        let mut builder: Builder = Request::builder()
            .method(Method::PATCH)
            .uri(url);
        {
            match builder.headers_mut() {
                Some(h) => self.apply_authentication(h),
                _ => {}
            }
        }
        let request: Request<Body> = builder.body(Body::from(serde_json::to_string(&data)?))?;
        let res: Response<Body> = self.client.request(request).await?;
        self.handler(res).await
    }

    pub async fn put<T: DeserializeOwned, K: Serialize>(&self, endpoint: String, data: &K) -> Result<T>{
        let url: String = format!("{}{}", self.to_base(), endpoint);

        let mut builder: Builder = Request::builder()
            .method(Method::PUT)
            .uri(url);
        {
            match builder.headers_mut() {
                Some(h) => self.apply_authentication(h),
                _ => {}
            }
        }
        let request: Request<Body> = builder.body(Body::from(serde_json::to_string(&data)?))?;
        let res: Response<Body> = self.client.request(request).await?;
        self.handler(res).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, endpoint: String) -> Result<T> {
        let url: String = format!("{}{}", self.to_base(), endpoint);

        let mut builder: Builder = Request::builder()
            .method(Method::DELETE)
            .uri(url);
        {
            match builder.headers_mut() {
                Some(h) => self.apply_authentication(h),
                _ => {}
            }
        }
        let request: Request<Body> = builder.body(Body::from(""))?;
            // .body(Body::from(serde_json::to_string(&data)?))?;
        let res: Response<Body> = self.client.request(request).await?;
        self.handler(res).await
    }

    pub async fn delete_many<T: DeserializeOwned, K: Serialize>(&self, endpoint: String, data: &K) -> Result<T> {
        let url: String = format!("{}{}", self.to_base(), endpoint);

        let mut builder: Builder = Request::builder()
            .method(Method::DELETE)
            .uri(url);
        {
            match builder.headers_mut() {
                Some(h) => self.apply_authentication(h),
                _ => {}
            }
        }
        let request: Request<Body> = builder.body(Body::from(serde_json::to_string(&data)?))?;
        let res: Response<Body> = self.client.request(request).await?;
        self.handler(res).await
    }

    async fn handler<T: DeserializeOwned>(&self, res: Response<Body>) -> Result<T> {
        if res.status().is_success() {
            let body = hyper::body::aggregate(res).await?;
            let data: T = serde_json::from_reader(body.reader())?;
            return Ok(data);
        }
        let body = hyper::body::aggregate(res).await?;
        let err: ApiError = serde_json::from_reader(body.reader())?;
        Err(err.into())
    }
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use crate::Client;

    use super::Config;
    
    #[derive(Deserialize)]
    pub struct Fake;

    #[tokio::test]
    async fn test_invalid_uri() {
        let client: Client = Client::new(&Config{
            host: "fake".to_owned(), 
            database: "fake".to_owned(),
            user: String::new(),
            password: String::new(),
        });

        match client.get("fake".to_owned()).await {
            Ok(fake) => fake,
            Err(_) => {}
        };
    }

    #[tokio::test]
    async fn test_bad_connection() {
        let client: Client = Client::new(&Config{
            host: "http://fake.domain.test".to_owned(), 
            database: "fake".to_owned(),
            user: String::new(),
            password: String::new(),
        });

        match client.get("fake".to_owned()).await {
            Ok(fake) => fake,
            Err(_) => {}
        };
    }

    #[tokio::test]
    async fn test_404() {
        let client: Client = Client::new(&Config{
            host: "http://localhost:8529".to_owned(), 
            database: "blandromeda".to_owned(),
            user: String::new(),
            password: String::new(),
        });

        match client.get("fake".to_owned()).await {
            Ok(()) => (),
            Err(_) => {}
        }
    }
}