use super::Indexer;
use crate::data::log_record::LogRecordPos;
use parking_lot::RwLock;
use std::{collections::BTreeMap, sync::Arc};
/// encapslate the BTreeMap
pub struct BTree {
    // key can be any size
    tree: Arc<RwLock<BTreeMap<Vec<u8>, LogRecordPos>>>,
}
impl BTree {
    pub fn new() -> Self {
        Self {
            tree: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }
}
impl Indexer for BTree {
    fn put(&self, key: Vec<u8>, pos: LogRecordPos) -> bool {
        let mut write_guard = self.tree.write();
        write_guard.insert(key, pos);
        true
    }

    fn get(&self, key: Vec<u8>) -> Option<LogRecordPos> {
        let read_guard = self.tree.read();
        read_guard.get(&key).copied()
    }

    fn delete(&self, key: Vec<u8>) -> bool {
        let mut write_guard = self.tree.write();
        let res = write_guard.remove(&key);
        res.is_some()
    }
}

// unit test
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_btree_put() {
        let bt = BTree::new();
        let res1 = bt.put(
            "".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 1,
                offset: 10,
            },
        );
        assert_eq!(res1, true);

        let res2 = bt.put(
            "aa".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 10,
                offset: 20,
            },
        );
        assert_eq!(res2, true);
    }

    #[test]
    fn test_btree_get() {
        let bt = BTree::new();
        let res1 = bt.put(
            "".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 1,
                offset: 10,
            },
        );
        assert_eq!(res1, true);

        let res2 = bt.put(
            "aa".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 10,
                offset: 20,
            },
        );
        assert_eq!(res2, true);

        let pos1 = bt.get("".as_bytes().to_vec());
        println!("POS = {:?}", pos1);
        assert!(pos1.is_some());
        assert_eq!(pos1.unwrap().file_id, 1);
        assert_eq!(pos1.unwrap().offset, 10);
        let pos2 = bt.get("aa".as_bytes().to_vec());
        println!("POS = {:?}", pos2);
        assert!(pos2.is_some());
        assert_eq!(pos2.unwrap().file_id, 10);
        assert_eq!(pos2.unwrap().offset, 20);
    }

    #[test]
    fn test_btree_delete() {
        let bt = BTree::new();
        let res1 = bt.put(
            "".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 1,
                offset: 10,
            },
        );
        assert_eq!(res1, true);

        let res2 = bt.put(
            "aa".as_bytes().to_vec(),
            LogRecordPos {
                file_id: 10,
                offset: 20,
            },
        );
        assert_eq!(res2, true);

        let d1 = bt.delete("".as_bytes().to_vec());
        assert!(d1);
        let d1 = bt.delete("aa".as_bytes().to_vec());
        assert!(d1);
        let d1 = bt.delete("not exist".as_bytes().to_vec());
        assert!(!d1);
    }
}
