use crate::thought::dto::{ThoughtDto, TrackDto};
use leptos::prelude::*;

#[server]
pub async fn create_thought(input: ThoughtDto) -> Result<(), ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let context = expect_context::<AppState>();
    context
        .thought_service
        .create_thought(input)
        .await
        .map_err(ServerFnError::new)
}

#[server]
pub async fn get_all_thoughts() -> Result<Vec<ThoughtDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    let context = expect_context::<AppState>();
    context
        .thought_service
        .get_all_thoughts()
        .await
        .map_err(ServerFnError::new)
}

mod test {
    use super::*;
    /// TODO: Use a mocked state with database test connections
    #[test]
    fn create_a_thought() -> anyhow::Result<(), ServerFnError> {
        assert_eq!(0, 0);
        Ok(())
    }
}
