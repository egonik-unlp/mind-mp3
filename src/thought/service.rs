use crate::thought::{dto::ThoughtDto, repository::ThougtRepository};

#[derive(Debug, Clone)]
pub struct ThoughtService {
    pub repository: ThougtRepository,
}
impl ThoughtService {
    pub fn new(repository: ThougtRepository) -> Self {
        Self { repository }
    }

    pub async fn create_thought(&self, input: ThoughtDto) -> anyhow::Result<()> {
        self.repository.create_thought(input).await?;
        Ok(())
    }
    pub async fn get_all_thoughts(&self) -> anyhow::Result<Vec<ThoughtDto>> {
        let res = self.repository.get_all_thoughts().await?;
        Ok(res)
    }
}
