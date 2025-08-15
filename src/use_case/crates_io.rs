#[derive(Debug, Clone)]
pub struct CratesIoUseCase {
    pub crates_io_repository:
        std::sync::Arc<dyn crate::repository::crates_io::CratesIoRepository + Send + Sync>,
}

impl CratesIoUseCase {
    pub async fn search_crate(
        &self,
        keyword: &str,
    ) -> Result<Vec<crate::entity::crates_io::CrateSummaryEntity>, crate::error::Error> {
        let crates = self.crates_io_repository.search_crate(keyword).await?;
        let entities = crates
            .into_iter()
            .map(|c| crate::entity::crates_io::CrateSummaryEntity {
                name: c.name,
                description: c.description,
                latest_stable_version: c.latest_stable_version,
                latest_version: c.latest_version,
                downloads: c.downloads,
                created_at: c.created_at,
                updated_at: c.updated_at,
            })
            .collect::<Vec<crate::entity::crates_io::CrateSummaryEntity>>();

        Ok(entities)
    }
}
