use std::path::PathBuf;
pub struct Options{
    // 数据库的文件位置
    pub dir_path: PathBuf,

    // data file size
    pub data_file_size: u64,

    // sync after write
    pub sync_writes: bool,

    pub index_type: IndexType,
}

pub enum IndexType{
    BTree,


    SkipList,
}