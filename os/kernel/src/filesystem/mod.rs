pub mod inode;
pub mod ramfs;
pub mod vfs;

pub fn init() {
    vfs::init();
}
