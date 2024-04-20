use std::sync::Arc;
use crate::news::handlers::NewsRepository;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn NewsRepository>,
}