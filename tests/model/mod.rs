use blandango::{Collection, Arango, Config, NewCollection, Properties, Database};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct EdgeData {
    pub body: String
}


#[derive(Serialize, Deserialize, Debug)]
pub struct QueryBody {
    #[serde(rename="@collection")]
    pub collection: String,
    pub body: String
}

pub fn test_config() -> Config {
    Config { 
        host: "http://localhost:8529".to_owned(),            
        database: "_system".to_owned(),
        user: "blandango".to_owned(),
        password: "blandango".to_owned(),
    }
}

pub async fn setup_collection(name: &str) -> Properties {
    teardown(&name).await;

    let db: Database = Database::new(&test_config());

    let mut new_collection: NewCollection = NewCollection::default_document_collection(name.to_owned());
    new_collection.cache_enabled = true;

    let props: Properties = db.new_collection(&new_collection)
        .await
        .expect(&format!("error creating collection {}", new_collection.name));

    return props;
}

pub async fn setup_edge_collection(name: &str) -> Properties {
    teardown(&name).await;

    let db: Database = Database::new(&test_config());
    
    let mut new_collection: NewCollection = NewCollection::default_document_collection(name.to_owned());
    new_collection.cache_enabled = true;
    new_collection.r#type = 3;

    let props: Properties = db.new_collection(&new_collection)
        .await
        .expect(&format!("error creating collection {}", new_collection.name));

    return props;
}

pub async fn teardown(name: &str) {
    let db: Database = Database::new(&test_config());

    let collection: Collection = db.collection(name);

    _ = collection.drop().await;
}