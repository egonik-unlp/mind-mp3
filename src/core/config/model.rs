use serde::{Deserialize, Serialize};

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // pub qdrant_url: String,
    // pub qdrant_api_key: String,
    // pub qdrant_collection_name: String,
    pub spotify_endpoint: String,
    pub database_url: String,
    pub spotify_client_secret: String,
    pub spotify_client_id: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().expect("Cannot read .env file. Panicking");
        // let qdrant_url =
        //     env::var("QDRANT_CLUSTER_ENDPOINT").expect("QDRANT_CLUSTER_ENDPOINT missing");
        // let qdrant_api_key = env::var("QDRANT_API_KEY").expect("QDRANT_API_KEY missing");
        // let qdrant_collection_name =
        //     env::var("QDRANT_COLLECTION_NAME").expect("QDRANT_COLLECTION_NAME missing");
        let spotify_endpoint = env::var("SPOTIFY_ENDPOINT").expect("SPOTIFY_ENDPOINT missing");
        let spotify_client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET missing");
        let spotify_client_id = env::var("CLIENT_ID").expect("CLIENT_ID_missing");
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing");
        Self {
            // qdrant_url,
            // qdrant_api_key,
            // qdrant_collection_name,
            spotify_endpoint,
            spotify_client_secret,
            spotify_client_id,
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
