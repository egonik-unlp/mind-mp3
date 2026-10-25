use qdrant_client::Payload;
use serde::{de::DeserializeOwned, Serialize};

pub trait VectorDatabaseItem: DeserializeOwned + Serialize {
    fn into_description(&self) -> String;
    fn into_payload(&self) -> anyhow::Result<Payload> {
        let payload: Payload = serde_json::to_value(self)
            .map_err(|err| anyhow::anyhow!(err))?
            .try_into()?;
        Ok(payload)
    }
}
