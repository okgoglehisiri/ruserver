use std::sync::Arc;

use adapter::{database::ConnectionPool, repository::health::HealthCheckRepositoryImpl};
use kernel::repository::health::HealthCheckRepository;

// DI container
#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        // resolve dependencies
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        Self {
            health_check_repository,
        }
    }

    // return dependencies resolved instance
    pub fn health_check_repository(&self) -> Arc<dyn HealthCheckRepository> {
        self.health_check_repository.clone()
    }
}
