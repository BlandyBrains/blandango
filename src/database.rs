use serde::{Serialize, Deserialize};

use crate::{
    Result, 
    Client, Router, 
    DatabaseRouter, NewCollection, 
    Properties, CollectionQueryParams, CollectionRouter, Collection
};


#[derive(Deserialize)]
pub struct DatabaseResponse<T>
where T: Serialize {
    pub error: bool, 
    pub code: u16,
    pub result: T
}


///
/// Db represents an ArangoDB instance.
///
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Db {
    // the name of the current database
    pub name: String,

    // the id of the current database
    pub id: String,

    // path: the filesystem path of the current database
    pub path: String,

    // isSystem: whether or not the current database is the _system database
    pub is_system: bool,

    // sharding: the default sharding method for collections created in this database
    pub sharding: Option<String>,

    // replicationFactor: the default replication factor for collections in this database
    pub replication_factor: Option<String>,

    // writeConcern: the default write concern for collections in this database
    pub write_concern: Option<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User{
    pub username: String,

    #[serde(rename="passwd")]
    pub password: String,

    pub active: bool

    //  A JSON object with extra user information. It is used by the web interface
    //  to store graph viewer settings and saved queries. Should not be set or
    //  modified by end users, as custom attributes will not be preserved.
    //  NOT SUPPORTED
    //  pub extra: E
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DbOptions {

    /// The sharding method to use for new collections in this database. Valid values
    /// are: "", "flexible", or "single". The first two are equivalent. (cluster only)
    pub sharding: Option<String>, 

    /// Default replication factor for new collections created in this database.
    /// Special values include "satellite", which will replicate the collection to
    /// every DB-Server (Enterprise Edition only), and 1, which disables replication.
    /// (cluster only)
    pub replication_factor: Option<String>,

    /// Default write concern for new collections created in this database.
    /// It determines how many copies of each shard are required to be
    /// in sync on the different DB-Servers. If there are less than these many copies
    /// in the cluster, a shard refuses to write. Writes to shards with enough
    /// up-to-date copies succeed at the same time, however. The value of
    /// writeConcern cannot be greater than replicationFactor.
    /// For SatelliteCollections, the writeConcern is automatically controlled to
    /// equal the number of DB-Servers and has a value of 0. (cluster only)
    pub write_concern: Option<u8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewDatabase {
    /// Has to contain a valid database name. The name must conform to the selected
    /// naming convention for databases. If the name contains Unicode characters, the
    /// name must be NFC-normalized.
    /// Non-normalized names will be rejected by arangod.
    pub name: String,

    pub options: Option<DbOptions>,

    /// An array of user objects. The users will be granted Administrate permissions
    /// for the new database. Users that do not exist yet will be created.
    /// If users is not specified or does not contain any users, the default user
    /// root will be used to ensure that the new database will be accessible after it
    /// is created. The root user is created with an empty password should it not exist.
    pub users: Option<Vec<User>>,
}

impl NewDatabase {
    pub fn new(name: String) -> Self{
        Self {
            name,
            options: None,
            users: None,
        }
    }
}

pub struct Database {
    pub client: Client
}
impl Database {
    /// List databases
    pub async fn list(&self) -> Result<Vec<String>> {
        let response: DatabaseResponse<Vec<String>> = self.client.get(<Router as DatabaseRouter>::base()).await?;
        Ok(response.result)
    }
    
    /// Create a database
    /// Must be in _system database.
    pub async fn create(&self, db: &NewDatabase) -> Result<bool> {
        let response: DatabaseResponse<bool> = self.client.post(<Router as DatabaseRouter>::base(), &db).await?;
        Ok(response.result)
    }

    /// Information of the database
    pub async fn current(&self) -> Result<Db> {
        let response: DatabaseResponse<Db> = self.client.get(Router::current()).await?;
        Ok(response.result)
    }

    /// List of accessible databases for user
    pub async fn user(&self) -> Result<Vec<String>> {
        let response: DatabaseResponse<Vec<String>> = self.client.get(Router::user()).await?;
        Ok(response.result)
    }
    
    /// Drop a database
    pub async fn drop(&self, database: &str) -> Result<bool> {
        let endpoint: String = format!("{}/{}", <Router as DatabaseRouter>::base(), database);
        let response: DatabaseResponse<bool> = self.client.delete(endpoint).await?;
        Ok(response.result)
    }

    /// Create a new collection
    /// ```
    /// Request
    /// {
    ///     "name": String,
    ///     "waitForSync": bool,
    ///     "isSystem": bool,
    ///     "schema": {
    ///         rule: { 
    ///           properties: { nums: { type: "array", items: { type: "number", maximum: 6 } } }, 
    ///           additionalProperties: { type: "string" },
    ///           required: ["nums"]
    ///         },
    ///         level: "moderate",
    ///         message: "The document does not contain an array of numbers in attribute 'nums', or one of the numbers is greater than 6."
    ///       },
    ///     "computedValues": [],
    ///     "keyOptions": {
    ///         "type": String,
    ///         "allowUserKeys": bool,
    ///         "increment": String,
    ///         "offset": String
    ///     },
    ///     "type": int,
    ///     "cachedEnabled": bool,
    ///     "numberOfShards": int,
    ///     "shardKeys": [String],
    ///     "replicationFactor": int,
    ///     "writeConcern": int,
    ///     "shardingStrategy": String,
    ///     "distributeShardLike": String,
    ///     "isSmart": bool,
    ///     "isDisjoint": bool,
    ///     "smartGraphAttribute": String,
    ///     "smartJoinAttribute": String
    /// }
    /// ```
    pub async fn new_collection(&self, new_collection: &NewCollection) -> Result<Properties> {
        let endpoint: String = <Router as CollectionRouter>::base_with_params(CollectionQueryParams::default())?;
        Ok(self.client.post(endpoint, &new_collection).await?)
    }

    pub fn collection(&self, name: &str) -> Collection {
        return Collection::new(self.client.clone(), name.to_string())
    }
}

