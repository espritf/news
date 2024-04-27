use std::sync::Arc;
use crate::news::handlers::NewsRepository;
use crate::transfomer::Model;

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn NewsRepository>,
    pub model: Arc<Model>,
}