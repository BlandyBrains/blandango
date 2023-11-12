use blandango::*;
mod model;

use crate::model::test_config;


#[tokio::test]
async fn test_database() {
    let config: &Config = &test_config();
    let database: Database = Arango::new(config);

    // create 
    let db: NewDatabase = NewDatabase::new("integration_db".to_owned());
    let created = database.create(&db).await.unwrap();
    assert_eq!(true, created);

    // list
    let dbs: Vec<String> = database.list().await.unwrap();
    println!("databases: {:?}", dbs);

    // info
    let info: Db = database.current().await.unwrap();
    println!("info: {:#?}", info);

    // user
    let user_dbs: Vec<String> = database.user().await.unwrap();
    println!("user: {:#?}", user_dbs);

    // drop
    let drop = database.drop(&db.name).await.unwrap();
    println!("drop: {:#?}", drop);
}