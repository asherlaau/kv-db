// data file that stores the data
pub struct DataFile{
    file_id: Arc<RwLock<u32>>, // file's id
    write_off: Arc<RwLock<u64>>, // to mark the current writing postion of the file
    io_manager: Box<dyn fio::IOManager>, // io interface, it have to be dyn as trait size is unknown
}



pub const DATA_FILE_NAME_SUFFIX: &str = ".data";

// question mark ? 
impl DataFile{

    pub fn new(dir_path: PathBuf, file_id: u32) -> Result<DataFile>{
        todo!()
    }
    pub fn get_write_off(&self) -> u64{
        let read_guard = self.write_off.read();
        *read_guard 
    }

    pub fn sync(&self) -> Result<()> {
        todo!()
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        todo!()
    }

    pub read_log_record(&self, offset: u64) -> Result<ReadLogRecord>{
        todo!()
    }

    pub fn set_write_off(&self, offset:u64){
        let mut write_guard = self.write_off.write();
        *write_guard = offset;
    }
    pub fn get_file_id(&self) -> u32{
        let read_guard = self.file_id.read();
        *read_guard
    }
}