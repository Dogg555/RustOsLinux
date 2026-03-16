#[derive(Clone, Copy, Debug)]
pub struct EthernetHeader {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub ethertype: u16,
}

impl EthernetHeader {
    pub const LEN: usize = 14;

    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::LEN {
            return None;
        }

        let mut dst = [0u8; 6];
        let mut src = [0u8; 6];
        dst.copy_from_slice(&buf[0..6]);
        src.copy_from_slice(&buf[6..12]);
        let ethertype = u16::from_be_bytes([buf[12], buf[13]]);

        Some(Self {
            dst,
            src,
            ethertype,
        })
    }
}
