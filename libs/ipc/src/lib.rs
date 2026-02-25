#![no_std]

pub const IPC_PAYLOAD_MAX: usize = 256;

#[repr(C)]
pub struct Message {
    pub sender_pid: u32,
    pub channel: u32,
    pub len: u16,
    pub payload: [u8; IPC_PAYLOAD_MAX],
}
