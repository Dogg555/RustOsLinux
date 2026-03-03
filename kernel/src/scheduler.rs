use core::sync::atomic::{AtomicU64, Ordering};

pub const MAX_TASKS: usize = 16;

static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisterState {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

impl RegisterState {
    pub const fn empty() -> Self {
        Self {
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: 0,
            rsp: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rip: 0,
            rflags: 0x202,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub stack_pointer: u64,
    pub registers: RegisterState,
}

impl Task {
    pub fn new(entry_ip: u64, stack_pointer: u64) -> Self {
        let id = NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed);
        let mut regs = RegisterState::empty();
        regs.rip = entry_ip;
        regs.rsp = stack_pointer;

        Self {
            id,
            stack_pointer,
            registers: regs,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContextSwitch {
    pub previous_task: u64,
    pub next_task: u64,
}

pub struct RoundRobinScheduler {
    tasks: [Option<Task>; MAX_TASKS],
    run_queue: [usize; MAX_TASKS],
    queue_len: usize,
    current_pos: usize,
}

impl RoundRobinScheduler {
    pub const fn new() -> Self {
        Self {
            tasks: [None; MAX_TASKS],
            run_queue: [0; MAX_TASKS],
            queue_len: 0,
            current_pos: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) -> Result<u64, &'static str> {
        if self.queue_len >= MAX_TASKS {
            return Err("run queue full");
        }

        let slot = self.find_free_slot().ok_or("task table full")?;
        self.tasks[slot] = Some(task);
        self.run_queue[self.queue_len] = slot;
        self.queue_len += 1;
        Ok(task.id)
    }

    pub fn on_timer_tick(&mut self) -> Option<ContextSwitch> {
        if self.queue_len <= 1 {
            return None;
        }

        let prev_slot = self.run_queue[self.current_pos];
        self.current_pos = (self.current_pos + 1) % self.queue_len;
        let next_slot = self.run_queue[self.current_pos];

        let prev = self.tasks[prev_slot].expect("run queue must point at valid tasks");
        let next = self.tasks[next_slot].expect("run queue must point at valid tasks");

        Some(ContextSwitch {
            previous_task: prev.id,
            next_task: next.id,
        })
    }

    pub fn save_current_registers(&mut self, regs: RegisterState) {
        if self.queue_len == 0 {
            return;
        }

        let slot = self.run_queue[self.current_pos];
        if let Some(task) = &mut self.tasks[slot] {
            task.registers = regs;
            task.stack_pointer = regs.rsp;
        }
    }

    pub fn current_task(&self) -> Option<Task> {
        if self.queue_len == 0 {
            return None;
        }

        self.tasks[self.run_queue[self.current_pos]]
    }

    pub fn load_next_registers(&self) -> Option<RegisterState> {
        self.current_task().map(|t| t.registers)
    }

    fn find_free_slot(&self) -> Option<usize> {
        let mut i = 0;
        while i < MAX_TASKS {
            if self.tasks[i].is_none() {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_robin_rotates_tasks() {
        let mut scheduler = RoundRobinScheduler::new();
        let t1 = Task::new(0x1000, 0x8000);
        let t2 = Task::new(0x2000, 0x9000);
        let id1 = scheduler.add_task(t1).unwrap();
        let id2 = scheduler.add_task(t2).unwrap();

        assert_eq!(scheduler.current_task().unwrap().id, id1);
        let sw = scheduler.on_timer_tick().unwrap();
        assert_eq!(sw.previous_task, id1);
        assert_eq!(sw.next_task, id2);
        assert_eq!(scheduler.current_task().unwrap().id, id2);
    }

    #[test]
    fn context_save_and_restore_tracks_registers() {
        let mut scheduler = RoundRobinScheduler::new();
        let _ = scheduler.add_task(Task::new(0x3000, 0xA000)).unwrap();

        let mut regs = RegisterState::empty();
        regs.rax = 42;
        regs.rsp = 0xBEEF;
        scheduler.save_current_registers(regs);

        let loaded = scheduler.load_next_registers().unwrap();
        assert_eq!(loaded.rax, 42);
        assert_eq!(loaded.rsp, 0xBEEF);
    }
}
