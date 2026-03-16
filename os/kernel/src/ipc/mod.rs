use alloc::collections::VecDeque;
use spin::Mutex;

#[derive(Clone, Copy)]
pub struct Message {
    pub from: u32,
    pub to: u32,
    pub value: u64,
}

static QUEUE: Mutex<VecDeque<Message>> = Mutex::new(VecDeque::new());

pub fn init() {}

pub fn send(msg: Message) {
    QUEUE.lock().push_back(msg);
}

pub fn recv_for(pid: u32) -> Option<Message> {
    let mut queue = QUEUE.lock();
    let idx = queue.iter().position(|m| m.to == pid)?;
    queue.remove(idx)
}
