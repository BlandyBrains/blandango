use blandango::*;
use serde::{Serialize, Deserialize};

mod model;

use crate::model::{test_config, teardown, setup_collection};


#[derive(Serialize, Deserialize, Debug)]
pub struct SampleData {
    pub body: String
}

#[tokio::test]
async fn test_index(){
    let config: &Config = &test_config();
    let index: Index = Arango::new(config);

    let _: Properties = setup_collection(&name::<SampleData>()).await;

    // View Indexes
    let indices: Vec<Idx> = index.read(&name::<SampleData>()).await.unwrap();
    println!("Indexes: {:#?}", indices);

    // Get Index
    let primary: IndexResponse = index.get(&indices.first().unwrap().id).await.unwrap();
    println!("Primary: {:#?}", primary);

    // Create Index
    let mut new_index: NewIdx = NewIdx::default();
    new_index.unique = Some(true);
    new_index.name = "unique_body".to_owned();
    new_index.fields = vec!["body".to_owned()];
    new_index.collection = name::<SampleData>();
    // new_index.stored_values = Some(vec![]);

    let new_idx: IndexResponse = index.create(&new_index).await.unwrap();
    println!("New Index: {:#?}", new_idx);

    // Delete Index
    let del: IdResponse = index.delete(&new_idx.id).await.unwrap();
    println!("Del Index: {:#?}", del);

    teardown(&name::<SampleData>()).await;
}