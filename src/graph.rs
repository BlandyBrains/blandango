use std::any::type_name;

use convert_case::{Case, Casing};
use serde::{Serialize, de::DeserializeOwned, Deserialize};

use crate::{
    DocumentQueryParams, DocumentResponse, 
    Result, Client, Query, CollectionRouter,
    QueryResponse, QueryBound, 
    api::{API, self}, EmptyResult, Properties, CollectionRequest, 
    CollectionQueryParams, IdResponse, Index, NameResponse, Router, ArangoEdgeKeys
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge<R> {
    #[serde(flatten)]    
    pub keys: ArangoKey,

    #[serde(flatten)]
    pub record: R,

    pub modified_on: Option<i64>,
    pub created_on: Option<i64>
}

impl<R> Edge<R>
where 
    R: Serialize + DeserializeOwned + Sized + Sync + Send {

    pub fn collection_name() -> String {
        let mut full_name: String = type_name::<R>().to_owned();
        full_name = full_name.split("<").next().unwrap().to_string();
        return full_name.split("::").last().to_owned().unwrap().to_owned().to_case(Case::Snake);
    }

    // Graph Operations
    pub async fn create_graph(client: &Client) -> Result<Properties> {
        let request: CollectionRequest = CollectionRequest::default_document_collection(Self::graph_name());
        let endpoint: String = Router::base_with_params(CollectionQueryParams::default())?;
        Ok(client.post(endpoint, &request).await?)
    }

    pub async fn create_graph_with_options(client: &Client, request: &CollectionRequest) -> Result<Properties> {
        let endpoint: String = Router::base_with_params(CollectionQueryParams::default())?;
        Ok(client.post(endpoint, &request).await?)
    }

    pub async fn get(client: &Client, key: &str) -> Result<Self>{
        Ok(client.get(Router::document_key(&Self::graph_name(), key)).await?)
    }

    pub async fn create(client: &Client, model: &R) -> Result<Self>{
        let response: DocumentResponse<Self> = client.post(Router::document_with_params(&Self::collection_name(), DocumentQueryParams::default()).unwrap(), &model).await?;
        Ok(response.new.unwrap())
    }

    pub async fn delete(client: &Client, key: &str) -> Result<Self> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.return_old = Some(true);
        params.return_new = None;

        let response: DocumentResponse<Self> = client.delete(Router::document_key_with_params(&Self::collection_name(), &key, params).unwrap()).await?;
        Ok(response.old.unwrap())
    }
    
    /*
        update assumes we have an instance of the document record.
        since this requires a round trip request to fetch the require before mutation
        it is less likely to be used. In most instances, it's best to use the Provider trait 
        and modify a document through it's parent.
    */
    pub async fn update(client: &Client, document: &Self) -> Result<bool> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.silent = Some(true);
        params.return_new = None;

        let _: EmptyResult = client.patch(Router::document_key_with_params(&Self::collection_name(), &document.keys.key, params).unwrap(), &document.record).await?;
        Ok(true)
    }
    
    /*
        replace assumes we have an instance of the document record.
        since this requires a round trip request to fetch the require before mutation
        it is less likely to be used. In most instances, it's best to use the Provider trait 
        and modify a document through it's parent.
    */    
    pub async fn replace(client: &Client, document: &Self) -> Result<bool> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.silent = Some(true);
        params.return_new = None;

        let _: EmptyResult = client.put(Router::document_key_with_params(&Self::collection_name(), &document.keys.key, params).unwrap(), &document.record).await?;
        Ok(true)
    }
    
    pub async fn query(client: &Client, query: &Query) -> Result<QueryResponse<Vec<Self>>> {
        Ok(client.post(String::from(API::Cursor), query).await?)
    }
    
    /// Executes a Bound AQL query
    /// 
    /// ```
    /// let query: QueryBound<QueryBody> = QueryBound{
    ///   id: None, 
    ///   query:r#"
    ///     FOR r in @@collection
    ///     FILTER r.body == @body
    ///     RETURN r
    ///   "#.to_owned(), 
    ///   batch_size: 100, 
    ///   count: true,
    ///   bind: QueryBody{ 
    ///     collection: Document::<SampleData>::collection_name(),
    ///     body: message.record.body.clone() 
    ///   },
    /// };
    /// 
    /// let results: QueryResponse<Vec<Document<SampleData>>> = Document::bounded_query(client, &query).await.unwrap();
    /// ```
    pub async fn bound_query<Q: Serialize + Sync>(client: &Client, query: &QueryBound<Q>) -> Result<QueryResponse<Vec<Self>>> {
        Ok(client.post(String::from(API::Cursor), query).await?)
    }

    pub async fn properties(client: &Client) -> Result<Properties>{
        Ok(client.get(Router::extension(&Self::collection_name(), API::Collection(api::Collection::Properties))).await?)
    }
    
    pub async fn delete_collection(client: &Client) -> Result<IdResponse> {
        let endpoint: String = Router::base_collection(&Self::collection_name());
        Ok(client.delete(endpoint).await?)
    }
    
    pub async fn create_index(client: &Client, index: &mut Index) -> Result<NameResponse> {
        index.name = Self::collection_name();
        let endpoint: String = Router::index(&index.collection);
        let res: NameResponse = client.post(endpoint, index).await?;
        Ok(res)
    }
    
}

#[cfg(test)]
mod test {
    use serde::{Serialize, Deserialize};
    use crate::Document;

    #[test]
    fn test_collection_name_scheme() {
        #[derive(Serialize, Deserialize, Debug)]
        struct MyTestData {}

        type MyCollection = Document<MyTestData>;

        assert_eq!(Document::<MyTestData>::collection_name(), "my_test_data");
        assert_eq!(MyCollection::collection_name(), "my_test_data");
    }
}