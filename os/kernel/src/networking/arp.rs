#[derive(Clone, Copy, Debug)]
pub struct ArpPacket {
    pub oper: u16,
    pub sender_mac: [u8; 6],
    pub sender_ip: [u8; 4],
    pub target_mac: [u8; 6],
    pub target_ip: [u8; 4],
}

impl ArpPacket {
    pub const LEN: usize = 28;

    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < Self::LEN {
            return None;
        }

        let oper = u16::from_be_bytes([buf[6], buf[7]]);
        let mut sender_mac = [0u8; 6];
        sender_mac.copy_from_slice(&buf[8..14]);

        let mut sender_ip = [0u8; 4];
        sender_ip.copy_from_slice(&buf[14..18]);

        let mut target_mac = [0u8; 6];
        target_mac.copy_from_slice(&buf[18..24]);

        let mut target_ip = [0u8; 4];
        target_ip.copy_from_slice(&buf[24..28]);

        Some(Self {
            oper,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }
}
