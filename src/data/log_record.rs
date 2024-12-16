#[derive(PartialEq)]
pub enum LogRecordType {
    NORMAL = 1,
    // to label the deleted data
    DELETED = 2,
}

/// It is called Log cause it only append data, just like logging
/// LogRecord is a record of what you writed to the data file
pub struct LogRecord {
    pub(crate) key: Vec<u8>,
    pub(crate) value: Vec<u8>,
    pub(crate) rec_type: LogRecordType,
}

/// information of data position and index, decribe where the data store
#[derive(Clone, Copy, Debug)]
pub struct LogRecordPos {
    pub(crate) file_id: u32,
    pub(crate) offset: u64,
}

/// the information read from datafile
pub struct ReadLogRecord{
    pub(crate) record: LogRecord,
    pub(crate) size: u64,
}
impl LogRecord {
    pub fn encode() -> Vec<u8> {
        todo!()
    }
}
