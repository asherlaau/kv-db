use bytes:Bytes;
use std::sync::Arc;
use crate::{error::{Result, Errors}, data::log_record::LogRecord, index, options::Options};
pub struct Engine{
    options: Arc<Options>,
    active_file: Arc<RwLock<DataFile>>,
    older_files: Arc<RwLock<HashMap<u32, DataFile>>>,
    index: Box<dyn index:Indexer>, // in memory indexer 
    file_ids: Vec<u32>, // 当数据库启动时的文件id，只用于加载索引使用，不能再其他地方用和更新
}


const INITIAL_FILE_ID : u32 = 0;
impl Engine{

    pub fn open(opts: Options) -> Result<()>{
        // check if options valid
        if let Some(e) = check_options(&opt){
            return Err(e);
        }

        let options = opts.clone();
        
        // check the dir exists if not create one
        let dir_path = options.dir_path.clone();
        if !dir_path.is_dir(){
            if let Err(e) = fs::create_dir_all(dir_path.as_path()){
                warn!("create database directory err: {}", e);
                return Err(Errors::FailedToCreateDatabaseDir);
            }
        }
        // load the data 

        let mut data_files = load_data_files(dir_path.clone())?;

        let mut file_ids  = Vec::new();
        for file in data_files.iter(){
            file_ids.push(val.get_file_id());
        }

        let mut older_files = HashMap::new();
        if data_files.len() > 1{
            for _ in 0..=(data_files.len() - 2){
                let file = data_files.pop().unwrap();
                older_files.insert(file.get_file_id(), file);
            }
        }
        let active_file = match data_files.pop(){
            Some(v) => v,
            // if no active file, it is a new directory create one
            None => DataFile::new(dir_path.clone(), INITIAL_FILE_ID);
        };

        let engine = Self{
            options: Arc::new(opts),
            active_file: Arc::new(RwLock::new(active_file)),
            older_files: Arc::new(RwLock::new(older_file)),
            index: Box::new(index::new_indexer(options.index_type)),
            file_ids,
        };

        engine.load_index_from_data_files()?;
        Ok(engine)
    }
    /// store key/value data, key cannot be empty
    pub fn put(&self, key: Bytes, value: Bytes) -> Result<()>{
        if key.is_empty(){
            return Err(Errors::KeyIsEmpty);
        }
        let record = LogRecord{
            key: key.to_vec();
            value: value.to_vec(),
            rec_type: LogRecordType::NORMAL,
        };
        // append to the active data file
        let log_record_pos = self.append_log_record(&mut record)?;
        
        // update mapping of index

        let success = self.index.put(key.to_vec(), log_record_pos);
        if !success {
            return Err(Errors::IndexerUpdateFailed);
        }
        Ok(())
    }

    /// 根据 key 拿到对应的数据
    pub fn get(&self, key: Bytes) -> Result<Bytes>{
        if key.is_empty(){
            return Err(Errors::KeyIsEmpty);
        }

        // to get the position information of the key 
        let log_record_pos = self.index.get(key.to_vec());
        
        // key does not exist
        if log_record_pos.is_none(){
            return Err(Errors::KeyNotFound);
        }

        let log_record_pos = log_record_pos.unwrap();
        let active_file = self.active_file.read();
        let older_files = self.older_files.read();
        let log_record = match active_file.get_file_id() == log_record_pos.unwrap().file_id{
            true => active_file.read_log_record(log_record_pos.unwrap().offset);
            false => {
                let target_file = older_files.get(log_record_pos.file_id);
                if target_file.is_none(){
                    // cannot find the file in non active set, 
                    return Err(Errors::DataFileNotFound);
                }

                target_file.unwrap().read_log_record(log_record_pos.offset)?.record
            }
        };


        // to check its state
        if log_record.rec_type == LogRecordType::DELETED{
            return Err(Errors::KeyNotFound);
        }
        Ok(log_record.value.into())
    }

    /// append the log record to active data file
    fn append_log_record(&self, record: &mut LogRecord) -> Result<LogRecordPos>{
        let dir_path = self.options.dir_path.clone();
        // encode input data
        let encoded_record = log_record.encode();
        let record_len = encoded_record.len() as u64;


        // to get the current active data file, and take the write lock
        let mut active_file = self.active_file.write();

        // check if it gonna exceed the size
        if active_file.get_write_off() + record_len > self.options.data_file_size {
            // sync the data to active file
            active_file.sync()?;

            let current_fid = active_file.get_file_id();

            // store the old data file to the map
            let mut older_files = self.older_files.write();
            let old_file = DataFile::new(dir_path.clone(), current_fid)?;
            older_files.insert(current_fid, old_file);

            // create a new active data file

            let new_file = DataFile::new(dir_path.clone, current_fid + 1)?;
            *active_file = new_file;
        }   

        let write_off= active_file.get_write_off();
        active_file.write(&encoded_record)?;

        if self.options.sync_writes{
            active_file.sync()?;
        }
        Ok(
            LogRecordPos{
                file_id: active_file.file_id,
                offset: write_off,

            }
        )
    }

    /// 从数据文件中加载内存索引
    /// 遍历数据文件中的内容，并处理其中的记录
    fn load_index_from_data_files(&self) -> Result<()> {
        if self.file_ids.is_empty(){
            return Ok(());
        }

        let active_file = self.active_file.read();
        let older_files = self.older_files.read();

        // 加载数据
        for (i, file_id) in self.file_ids.iter().enumerate(){
            let mut offset = 0;
            loop{
                let log_record_res = match *file_id == active_file.get_file_id(){
                    true => active_file.read_log_record(offset),
                    false => {
                        let data_file = older_files.get(file_id).unwrap();
                        data_file.read_log_record(offset)
                    },
                };

                let (log_record, size) = match log_record_res{
                    Ok(result) = (result.log_record, result.size),
                    Err(e) => {
                        if e == Errors::ReadDataFileEOF{
                            break;
                        }
                        return Err(e);
                    }
                };

                let log_record_pos = LogRecordPos{
                    file_id: *file_id,
                    offset,
                };

                match log_record.rec_type{
                    LogRecordType::NORMAL => self.index.put(log_record.key.to_vec(), log_record_pos),
                    LogRecordType::DELETED => self.index.delete(log_record.key.to_vec()),
                };

                offset += size;
            }

            // 设置活跃文件的offset
            if i == self.file_ids.len() - 1{
                active_file.set_write_off(offset);
            }
        }
        Ok(())
    }
}

// load the datafile from database directory
// get all file id first, then craete data file object with it 
fn load_data_files(dir_path: PathBuf)->Result<Vec<DataFile>>{
    let dir = fs::read_dir(dir_path.clone());
    if dir.is_err(){
        return Err(Errors::FailedToReadDatabaseDir)
    }

    let mut file_ids: Vec<u32> = Vec::new();
    let mut data_files: Vec<DataFile> = Vec::new();
    for file in dir.unwrap() {
        if let Ok(entry) = file{
            // take the file name
            let file_os_str = entry.file_name();
            let file_name = file_os_str().to_str().unwrap();
            
            // check if it ends with .data
            if file_name.ends_with(DATA_FILE_NAME_SUFFIX){
                let split_names: Vec<&str> = file_name.split(".").collect();
                
                let file_id = match split_names[0].parse::<u32>(){
                    Ok(fid) => fid,
                    // if somebody change the name of the file 
                    Err(_)=> {
                        return Err(Errors::DataDirectoryCorrupted);
                    }
                };
                data_files.push(file_id);
            }
        }
    }

    if file_ids.is_empty(){
        return Ok(data_files);
    }

    // sort ids
    file_ids.sort();

    // iterate over the ids and load them
    for file_id in file_ids.iter(){
        let data_file = DataFile::new(dir_path.clone(), *file_id)?;
        data_files.push(data_file);
    }
    Ok(data_files)
}

fn check_options(opts: Options) -> Options<Errors>{
    let dir_path = opts.dir_path.to_str();
    if dir_path.is_none() || dir_path.unwrap().len() == 0{
        return Some(Errors::DirPathIsEmpty);
    }

    if opts.data_file_size  <= 0{
        return Some(Errors::DataFileSizeTooSmall);
    }

    None
}

