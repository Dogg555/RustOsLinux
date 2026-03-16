use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

use super::inode::{Inode, NodeType};

const MAX_DATA_BYTES: usize = 1024;

#[derive(Clone)]
struct FileData {
    inode: usize,
    bytes: [u8; MAX_DATA_BYTES],
    len: usize,
}

impl FileData {
    const fn empty() -> Self {
        Self {
            inode: usize::MAX,
            bytes: [0; MAX_DATA_BYTES],
            len: 0,
        }
    }
}

pub struct RamFs {
    inodes: Vec<Inode>,
    files: Vec<FileData>,
}

impl RamFs {
    pub fn new() -> Self {
        Self {
            inodes: vec![Inode::new(0, None, NodeType::Directory, "/")],
            files: vec![FileData::empty()],
        }
    }

    pub fn mkdir(&mut self, parent: usize, name: &str) -> usize {
        let id = self.inodes.len();
        self.inodes
            .push(Inode::new(id, Some(parent), NodeType::Directory, name));
        id
    }

    pub fn create_file(&mut self, parent: usize, name: &str) -> usize {
        let id = self.inodes.len();
        self.inodes
            .push(Inode::new(id, Some(parent), NodeType::File, name));
        self.files.push(FileData {
            inode: id,
            ..FileData::empty()
        });
        id
    }

    pub fn write_file(&mut self, inode: usize, data: &[u8]) -> usize {
        if let Some(file) = self.files.iter_mut().find(|f| f.inode == inode) {
            let count = core::cmp::min(data.len(), file.bytes.len());
            file.bytes[..count].copy_from_slice(&data[..count]);
            file.len = count;
            if let Some(node) = self.inodes.get_mut(inode) {
                node.data_len = count;
            }
            return count;
        }
        0
    }

    pub fn read_file(&self, inode: usize, out: &mut [u8]) -> usize {
        if let Some(file) = self.files.iter().find(|f| f.inode == inode) {
            let count = core::cmp::min(out.len(), file.len);
            out[..count].copy_from_slice(&file.bytes[..count]);
            return count;
        }
        0
    }

    pub fn children(&self, parent: usize) -> Vec<&Inode> {
        self.inodes
            .iter()
            .filter(|i| i.parent == Some(parent))
            .collect()
    }
}

static FS: Mutex<Option<RamFs>> = Mutex::new(None);

pub fn initialize() {
    *FS.lock() = Some(RamFs::new());
}

pub fn with_fs<R>(f: impl FnOnce(&mut RamFs) -> R) -> Option<R> {
    let mut guard = FS.lock();
    guard.as_mut().map(f)
}
