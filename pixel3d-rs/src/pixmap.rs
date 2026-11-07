use std::path::Path;

pub trait PortablePixmap {
    fn save(&self, filepath: impl AsRef<Path>) -> std::io::Result<()>;
}