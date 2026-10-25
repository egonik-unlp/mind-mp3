use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use std::env;

#[derive()]
pub struct Config {
    // pub qdrant_url: String,
    // pub qdrant_api_key: String,
    // pub qdrant_collection_name: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().expect("Cannot read .env file. Panicking");
        // let qdrant_url =
        //     env::var("QDRANT_CLUSTER_ENDPOINT").expect("QDRANT_CLUSTER_ENDPOINT missing");
        // let qdrant_api_key = env::var("QDRANT_API_KEY").expect("QDRANT_API_KEY missing");
        // let qdrant_collection_name =
        //     env::var("QDRANT_COLLECTION_NAME").expect("QDRANT_COLLECTION_NAME missing");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
        Self {
            // qdrant_url,
            // qdrant_api_key,
            // qdrant_collection_name,
            database_url,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InformationConfigFile {}

impl InformationConfigFile {
    pub fn new() -> Self {
        Self {}
    }
}
