#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    File,
    Directory,
}

#[derive(Clone, Debug)]
pub struct Inode {
    pub id: usize,
    pub parent: Option<usize>,
    pub node_type: NodeType,
    pub name: [u8; 32],
    pub name_len: usize,
    pub data_len: usize,
}

impl Inode {
    pub fn new(id: usize, parent: Option<usize>, node_type: NodeType, name: &str) -> Self {
        let mut buf = [0u8; 32];
        let mut len = 0usize;
        for b in name.bytes().take(buf.len()) {
            buf[len] = b;
            len += 1;
        }

        Self {
            id,
            parent,
            node_type,
            name: buf,
            name_len: len,
            data_len: 0,
        }
    }

    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.name[..self.name_len]).unwrap_or("?")
    }
}
