use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FsHierarchy<P = PathBuf> {
    pub config_store: P,
    pub mods_store: P,
}
