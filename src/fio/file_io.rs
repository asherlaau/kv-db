use std::{
    fs,
    fs::{File, OpenOptions},
    io::Write,
    os::unix::prelude::FileExt,
    path::PathBuf,
    sync::Arc,
};

use crate::errors::{Errors, Result};
use log::error;
use parking_lot::RwLock;

use super::IOManager;

// FileIo standard system file IO from linux
pub struct FileIO {
    fd: Arc<RwLock<File>>, // 系统文件描述符
}

impl FileIO {
    pub fn new(file_name: PathBuf) -> Result<Self> {
        match OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(file_name)
        {
            Ok(file) => {
                return Ok(FileIO {
                    fd: Arc::new(RwLock::new(file)),
                });
            }
            Err(e) => {
                error!("open data file err: {}", e);
                return Err(Errors::FailedOpenDataFile);
            }
        }
    }
}
impl IOManager for FileIO {
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize> {
        let read_guard = self.fd.read();
        match read_guard.read_at(buf, offset) {
            Ok(n) => return Ok(n),
            Err(e) => {
                error!("read from data err: {}", e);
                return Err(Errors::FailedToReadFromDataFile);
            }
        };
    }

    fn write(&self, buf: &[u8]) -> Result<usize> {
        let mut write_guard = self.fd.write();
        match write_guard.write(buf) {
            Ok(n) => return Ok(n),
            Err(e) => {
                error!("write to data file err: {}", e);
                return Err(Errors::FailedWriteToDataFile);
            }
        };
    }

    // the reason of adding a sync when having a write
    // write doesnt mean writing to the disk, it is often written to
    // in-memory buffer first and then writing to disk
    // sync_all() ensure all the in-memory buffer are sync to the disk
    fn sync(&self) -> Result<()> {
        let read_guard = self.fd.read();
        if let Err(e) = read_guard.sync_all() {
            error!("failed to sync data file: {}", e);
            return Err(Errors::FailedSyncDataFile);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_io_write() {
        let path = PathBuf::from("/tmp/a.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());
        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("key-a".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(5, res1.ok().unwrap());

        let res1 = fio.write("key-b".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(5, res1.ok().unwrap());

        let res1 = fs::remove_file(path.clone());
        assert!(res1.is_ok());
    }

    #[test]
    fn test_file_io_read() {
        let path = PathBuf::from("/tmp/b.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());
        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("key-a".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(5, res1.ok().unwrap());

        let res2 = fio.write("key-b".as_bytes());
        assert!(res2.is_ok());
        assert_eq!(5, res2.ok().unwrap());

        let mut buf = [0u8; 5];
        let read_res1 = fio.read(&mut buf, 0);
        assert!(read_res1.is_ok());
        assert_eq!(5, read_res1.ok().unwrap());

        let mut buf2 = [0u8; 5];
        let read_res2 = fio.read(&mut buf2, 5);
        assert!(read_res2.is_ok());
        assert_eq!(5, read_res2.ok().unwrap());

        let res3 = fs::remove_file(path.clone());
        assert!(res3.is_ok());
    }

    #[test]
    fn test_file_io_sync() {
        let path = PathBuf::from("/tmp/c.data");
        let fio_res = FileIO::new(path.clone());
        assert!(fio_res.is_ok());
        let fio = fio_res.ok().unwrap();

        let res1 = fio.write("key-a".as_bytes());
        assert!(res1.is_ok());
        assert_eq!(5, res1.ok().unwrap());

        let res2 = fio.write("key-b".as_bytes());
        assert!(res2.is_ok());
        assert_eq!(5, res2.ok().unwrap());

        let sync_res = fio.sync();
        assert!(sync_res.is_ok());

        let res3 = fs::remove_file(path.clone());
        assert!(res3.is_ok());
    }
}
