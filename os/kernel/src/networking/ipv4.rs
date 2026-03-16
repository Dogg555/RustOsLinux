#[derive(Clone, Copy, Debug)]
pub struct Ipv4Header {
    pub ihl: u8,
    pub protocol: u8,
    pub src: [u8; 4],
    pub dst: [u8; 4],
}

impl Ipv4Header {
    pub const MIN_LEN: usize = 20;

    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::MIN_LEN {
            return None;
        }

        let version_ihl = buf[0];
        let version = version_ihl >> 4;
        if version != 4 {
            return None;
        }

        let ihl = version_ihl & 0x0f;
        let header_len = (ihl as usize) * 4;
        if buf.len() < header_len {
            return None;
        }

        let protocol = buf[9];

        let mut src = [0u8; 4];
        src.copy_from_slice(&buf[12..16]);

        let mut dst = [0u8; 4];
        dst.copy_from_slice(&buf[16..20]);

        Some(Self {
            ihl,
            protocol,
            src,
            dst,
        })
    }
}
