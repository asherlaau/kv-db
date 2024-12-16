pub mod btree;
use crate::{data::log_record::LogRecordPos, options::IndexType};

pub trait Indexer: Sync + Send {
    // needs to be thread safe

    /// store key, value information to indexer
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool;

    /// to get the LogRecordPos of the key
    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos>;

    /// to deletet the key from Indexer
    fn delete(&self, key: Vec<u8>) -> bool;
}

pub fn new_indexer(index_type: IndexType) -> impl Indexer{
    match index_type{
        IndexType::BTree => btree::BTree::new(),
        IndexType::SkipList => todo!(),
        _ => panic!(" unknown index type ")
    }
}
