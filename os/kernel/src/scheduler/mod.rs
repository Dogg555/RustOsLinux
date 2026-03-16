use spin::Mutex;

use crate::process::ProcessTable;

static TABLE: Mutex<Option<ProcessTable>> = Mutex::new(None);

pub fn init() {
    let mut table = ProcessTable::new();
    let _ = table.spawn(None, "init", 10);
    let _ = table.spawn(Some(1), "kworker/0", 5);
    let _ = table.spawn(Some(1), "netd", 6);

    *TABLE.lock() = Some(table);
}

pub fn tick() -> Option<u32> {
    let mut guard = TABLE.lock();
    guard.as_mut().and_then(ProcessTable::schedule_next)
}

pub fn dump() {
    let guard = TABLE.lock();
    if let Some(table) = guard.as_ref() {
        for proc_ in table.list() {
            println!(
                "proc pid={} name={} prio={} ticks={}",
                proc_.pid,
                proc_.name(),
                proc_.priority,
                proc_.ticks_used
            );
        }
    }
}
