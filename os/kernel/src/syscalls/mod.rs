use spin::Mutex;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SyscallNumber {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Fork = 4,
    Exec = 5,
    Wait = 6,
    Exit = 7,
    Sleep = 8,
    Socket = 9,
}

type Handler = fn(arg0: usize, arg1: usize, arg2: usize) -> isize;

fn unimplemented_handler(_: usize, _: usize, _: usize) -> isize {
    -38
}

struct SyscallTable {
    handlers: [Handler; 64],
}

impl SyscallTable {
    const fn new() -> Self {
        Self {
            handlers: [unimplemented_handler; 64],
        }
    }

    fn register(&mut self, num: SyscallNumber, handler: Handler) {
        self.handlers[num as usize] = handler;
    }

    fn dispatch(&self, num: usize, a: usize, b: usize, c: usize) -> isize {
        if let Some(handler) = self.handlers.get(num) {
            handler(a, b, c)
        } else {
            -38
        }
    }
}

static TABLE: Mutex<SyscallTable> = Mutex::new(SyscallTable::new());

pub fn init() {
    let mut table = TABLE.lock();
    table.register(SyscallNumber::Write, sys_write);
    table.register(SyscallNumber::Sleep, sys_sleep);
}

pub fn dispatch(number: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    TABLE.lock().dispatch(number, arg0, arg1, arg2)
}

fn sys_write(ptr: usize, len: usize, _fd: usize) -> isize {
    let msg = unsafe { core::slice::from_raw_parts(ptr as *const u8, len) };
    if let Ok(text) = core::str::from_utf8(msg) {
        print!("{}", text);
        len as isize
    } else {
        -22
    }
}

fn sys_sleep(ticks: usize, _unused: usize, _unused2: usize) -> isize {
    println!("sys_sleep({ticks}) called");
    0
}
