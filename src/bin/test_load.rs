use core::config::model::Config;
use mind_mp3::core::database::connection::get_connection_pool;
use mind_mp3::{
    core,
    thought::{
        dto::{ThoughtDto, TrackDto},
        repository::ThougtRepository,
        service::ThoughtService,
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env();
    let conn = get_connection_pool(config.database_url);
    let repository = ThougtRepository::new(conn);
    let service = ThoughtService::new(repository);
    let thoughts = vec![
        ThoughtDto {
            track: TrackDto {
                track_artist: "Raxon".into(),
                track_name: "Never stops".into(),
            },
        },
        ThoughtDto {
            track: TrackDto {
                track_artist: "Underworld".into(),
                track_name: "Skyscraper I Love You".into(),
            },
        },
    ];
    for thought in thoughts {
        service.create_thought(thought).await?;
    }
    Ok(())
}
