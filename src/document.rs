use serde::{Serialize, de::DeserializeOwned, Deserialize};

use crate::{
    DocumentQueryParams, 
    Result, Client, DocumentRouter,
    Empty, Router, ArangoKeys, name, ArangoEdgeKeys
};

#[derive(Deserialize, Debug)]
pub struct DocumentResponse<D> {
    // #[serde(rename="new")]
    pub new: Option<D>,

    // #[serde(rename="old")]
    pub old: Option<D>,
}

/*
    Generic Document struct 
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Doc<R> 
where 
    R: Sized + Sync + Send {

    #[serde(flatten)]    
    pub keys: ArangoKeys,

    #[serde(flatten)]
    pub record: R,

    pub modified_on: u64,
    pub created_on: u64
}

impl<R> Doc<R>
where 
    R: Serialize + DeserializeOwned + Sync + Send {
    /// helper function to get the naming scheme for record <R>.
    pub fn name(&self) -> String {
        return name::<R>();
    }
}


pub struct Document{
    pub client: Client
}

impl Document {

    /// Insert document into the collection
    pub async fn insert<R: Serialize + DeserializeOwned + Sync + Send>(&self, model: &R) -> Result<Doc<R>>{
        let response: DocumentResponse<Doc<R>> = self.client.post(Router::base_with_params(&name::<R>(), DocumentQueryParams::default()).unwrap(), &model).await?;
        Ok(response.new.unwrap())
    }

    /// Insert many documents into the collection
    pub async fn insert_many<R: Serialize + DeserializeOwned + Sync + Send>(&self, models: &Vec<R>) -> Result<Vec<ArangoKeys>>{
        let route: String = format!("{}#multiple", Router::base(&name::<R>()));
        println!("ROUTE: {:#?}", route);
        let response: Vec<ArangoKeys> = self.client.post(route, &models).await?;
        Ok(response)
    }

    /// Read a document by key
    pub async fn read<R: Serialize + DeserializeOwned + Sync + Send>(&self, key: &str) -> Result<Doc<R>>{
        Ok(self.client.get(Router::key(&name::<R>(), key)).await?)
    }

    /// Read multiple documents by key
    pub async fn read_many<S: Serialize + Sized + Sync + Send, R: Serialize + DeserializeOwned + Sync + Send>(&self, keys: &Vec<S>) -> Result<Vec<Doc<R>>> {
        let response: Vec<Doc<R>> = self.client.put(Router::base(&name::<R>()),keys).await?;
        Ok(response)
    }

    /// Read a document header by key
    /// Use this method to review the status of a document.
    /// Returns 200 empty response if document exists
    /// Returns 404 if document does not exist
    /// The ArangoDB endpoint allows further customization using headers which is not currently supported.
    pub async fn read_header<R: Serialize + DeserializeOwned + Sync + Send>(&self, key: &str) -> Result<()>{
        let _ : Empty = self.client.head(Router::key(&name::<R>(), key)).await?;
        Ok(())
    }

    /// Delete a document by key
    /// It's recommended to use the instance method Document::delete instead.
    pub async fn delete<R: Serialize + DeserializeOwned + Sync + Send>(&self, key: &str) -> Result<Doc<R>> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.return_old = Some(true);
        params.return_new = None;

        let response: DocumentResponse<Doc<R>> = self.client.delete(Router::key_with_params(&name::<R>(), &key, params).unwrap()).await?;
        Ok(response.old.unwrap())
    }

    /// Update a document
    pub async fn update<R: Serialize + DeserializeOwned + Sync + Send>(&self, doc: &Doc<R>) -> Result<()> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.silent = Some(true);
        params.return_new = None;

        let _: Empty = self.client.patch(Router::key_with_params(&doc.name(), &doc.keys.key, params).unwrap(), &doc.record).await?;
        Ok(())
    }
    
    /// Replace a document
    pub async fn replace<R: Serialize + DeserializeOwned + Sync + Send>(&self, doc: &Doc<R>) -> Result<()> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.silent = Some(true);
        params.return_new = None;

        let _: Empty = self.client.put(Router::key_with_params(&doc.name(), &doc.keys.key, params).unwrap(), &doc.record).await?;
        Ok(())
    }

    /// Destroy a document
    pub async fn destroy<R: Serialize + DeserializeOwned + Sync + Send>(&self, doc: &Doc<R>) -> Result<()> {
        let _: DocumentResponse<Doc<R>> = self.client.delete(
            Router::key_with_params(&name::<R>(), &doc.keys.key, DocumentQueryParams::default()).unwrap()).await?;
        Ok(())
    }    

    /// Delete many documents by key
    /// This endpoint will perform a bulk delete for many documents.
    /// If any error occurs during deletion it will be reflected in the header 'X-Arango-Error-Codes'
    /// as ERROR_CODE:QUANTITY,ERROR_CODE:QUANTITY,...
    /// Treat this endpoint as experimental.
    async fn delete_many<S: Serialize + Sized + Sync + Send, R: Serialize + DeserializeOwned + Sync + Send>(&self, keys: &Vec<S>) -> Result<()> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.return_old = None;
        params.return_new = None;

        // todo - this endpoint can return an array with mixed object structures
        // result = ["id", {"error": true, "errorNum": 1234, "errorMessage": "the reason for error" }]
        // Due to this mixed response it will require a special Serializer
        // Ignoring the response for now.
        let _: Empty = self.client.delete_many(Router::base_with_params(&name::<R>(), params).unwrap(), keys).await?;
        Ok(())
    }    

    /// Update many documents
    async fn update_many<R: Serialize + DeserializeOwned + Sync + Send>(&self, documents: &Vec<Doc<R>>) -> Result<()> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.silent = Some(true);
        params.return_new = None;

        let _: Empty = self.client.patch(Router::base_with_params(&name::<R>(), params).unwrap(), &documents).await?;
        Ok(())
    }

    /// Replace many documents
    async fn replace_many<R: Serialize + DeserializeOwned + Sync + Send>(client: &Client, documents: &Vec<Doc<R>>) -> Result<()> {
        let mut params: DocumentQueryParams = DocumentQueryParams::default();
        params.silent = Some(true);
        params.return_new = None;

        let _: Empty = client.put(Router::base_with_params(&name::<R>(), params).unwrap(), &documents).await?;
        Ok(())
    } 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edge<R>
where 
    R: Sized + Sync + Send {

    #[serde(flatten)]
    pub link: ArangoEdgeKeys,
    
    #[serde(flatten)]
    pub record: R
}

impl<R> Edge<R> 
where 
R: Sized + Sync + Send{
    pub fn new(from: String, to: String, record: R) -> Self {
        Self {
            link: ArangoEdgeKeys { to, from },
            record
        }
    }

}

#[cfg(test)]
mod test {
    use serde::Deserialize;

    use crate::{name, Edge, Doc};

    #[test]
    fn test_name_scheme() {
        struct MyTestData {}
        assert_eq!(name::<MyTestData>(), "my_test_data");

        type MyTestWrapper = Doc<MyTestData>;
        assert_eq!(name::<MyTestWrapper>(), "my_test_data");
    }

    #[test]
    fn test_edge_deserialize() {
        
        #[derive(Deserialize, Debug)]
        struct MyTestData {
            pub data: String
        }

        assert_eq!(name::<MyTestData>(), "my_test_data");

        let data: &str = r#"
        {
            "_id": "collection/1",
            "_key": "1",
            "_rev": "1234",
            "_to": "collection_a/2",
            "_from": "collection_b/3",
            "data": "SOME_DATA"
        }
        "#;

        let test_data: Edge<MyTestData> = serde_json::from_str(data).unwrap();

        // assert_eq!(test_data.keys.id, "collection/1");
        // assert_eq!(test_data.keys.key, "1");
        // assert_eq!(test_data.keys.rev, "1234");
        assert_eq!(test_data.link.to, "collection_a/2");
        assert_eq!(test_data.link.from, "collection_b/3");
        assert_eq!(test_data.record.data, "SOME_DATA");

        // println!("DATA: {:#?}", test_data);
    }
}