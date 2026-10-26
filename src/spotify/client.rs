#[derive(Debug, Clone)]
pub struct SpotifyClient {
    client_id: String,
    client_secret: String,
    endpoint: String,
}

impl SpotifyClient {
    pub fn new(client_id: String, client_secret: String, endpoint: String) -> Self {
        Self {
            client_id,
            client_secret,
            endpoint,
        }
    }
}
