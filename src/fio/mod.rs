use crate::errors::Result;
pub mod file_io;

pub trait IOManager: Sync + Send {
    // IOmanager will pass between threads, got to be thread safe
    /// read the  data on given position (offset)
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize>;

    /// write the bytes to the file
    fn write(&self, buf: &[u8]) -> Result<usize>;

    /// sync all the data in memory to the disk
    fn sync(&self) -> Result<()>;
}
