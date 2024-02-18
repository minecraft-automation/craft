use sha2::Sha256;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ModStoreEntry<P = PathBuf> {
    pub hash: Sha256,

    pub metadata: P,
    pub file: P,
}
