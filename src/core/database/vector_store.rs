use lvv::jobs::JobBuilder;
use qdrant_client::Qdrant;

pub type VectorDbPool = Qdrant;

pub async fn get_vector_db_connection(qdrant_url: String, qdrant_api_key: String) -> VectorDbPool {
    Qdrant::from_url(&qdrant_url)
        .api_key(qdrant_api_key)
        .build()
        .unwrap()
}
