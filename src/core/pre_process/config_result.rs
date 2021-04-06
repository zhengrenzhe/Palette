use std::sync::Arc;

#[derive(Debug)]
pub struct ConfigResult {
    pub images: Arc<Vec<String>>,
}
