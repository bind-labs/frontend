#[derive(Debug)]
pub enum SecureStorageError {}

type Result<T> = std::result::Result<T, SecureStorageError>;

pub fn secure_store(key: &str, value: &str) -> Result<()> {
    todo!("implement this for ios")
}

pub fn secure_retrieve(key: &str) -> Result<Option<String>> {
    todo!("implement this for ios")
}
