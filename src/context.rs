use std::path::PathBuf;

pub struct HandlerContext {
    base_dir: PathBuf,
}
impl HandlerContext {
    pub fn new(base_dir: PathBuf) -> Self {
        HandlerContext { base_dir }
    }

    pub fn base_dir(&self) -> PathBuf {
        self.base_dir.clone()
    }
}
