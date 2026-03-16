use alloc::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcessState {
    Ready,
    Running,
    Sleeping,
    Zombie,
}

#[derive(Clone, Debug)]
pub struct Process {
    pub pid: u32,
    pub parent: Option<u32>,
    pub priority: u8,
    pub state: ProcessState,
    pub ticks_used: u64,
    pub name: [u8; 24],
    pub name_len: usize,
}

impl Process {
    pub fn new(pid: u32, parent: Option<u32>, name: &str, priority: u8) -> Self {
        let mut name_buf = [0u8; 24];
        let mut name_len = 0usize;
        for b in name.bytes().take(name_buf.len()) {
            name_buf[name_len] = b;
            name_len += 1;
        }

        Self {
            pid,
            parent,
            priority,
            state: ProcessState::Ready,
            ticks_used: 0,
            name: name_buf,
            name_len,
        }
    }

    pub fn name(&self) -> &str {
        core::str::from_utf8(&self.name[..self.name_len]).unwrap_or("<invalid>")
    }
}

#[derive(Default)]
pub struct ProcessTable {
    next_pid: u32,
    procs: VecDeque<Process>,
}

impl ProcessTable {
    pub fn new() -> Self {
        Self {
            next_pid: 1,
            procs: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, parent: Option<u32>, name: &str, priority: u8) -> u32 {
        let pid = self.next_pid;
        self.next_pid += 1;
        self.procs
            .push_back(Process::new(pid, parent, name, priority));
        pid
    }

    pub fn schedule_next(&mut self) -> Option<u32> {
        let mut best_idx = None;
        let mut best_prio = 0u8;
        for (idx, p) in self.procs.iter().enumerate() {
            if p.state == ProcessState::Ready && (best_idx.is_none() || p.priority > best_prio) {
                best_idx = Some(idx);
                best_prio = p.priority;
            }
        }

        let idx = best_idx?;
        if let Some(proc_) = self.procs.get_mut(idx) {
            proc_.state = ProcessState::Running;
            proc_.ticks_used = proc_.ticks_used.saturating_add(1);
            let pid = proc_.pid;
            proc_.state = ProcessState::Ready;
            Some(pid)
        } else {
            None
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &Process> {
        self.procs.iter()
    }
}
