use leptos::prelude::*;

use crate::spotify::dto::SpotifyResultDto;

#[server]
pub async fn search(query: String) -> Result<SpotifyResultDto, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let context = expect_context::<AppState>();
    context
        .spotify_service
        .search(query)
        .map_err(ServerFnError::new)
}
