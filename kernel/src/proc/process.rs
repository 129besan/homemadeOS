use crate::sched::task::Pid;
use crate::mm::paging::address_space::AddressSpace;
use alloc::collections::BTreeMap;

pub enum ProcessState {
    Alive,
    Zombie,
}

pub struct Process {
    pub pid: Pid,
    pub state: ProcessState,
    pub address_space: Option<AddressSpace>,
}

pub struct ProcessTable {
    pub processes: BTreeMap<Pid, Process>,
    pub next_pid: u64,
}

impl ProcessTable {
    pub fn new() -> Self {
        ProcessTable {
            processes: BTreeMap::new(),
            next_pid: 1,
        }
    }

    pub fn allocate_pid(&mut self) -> Pid {
        let pid = Pid(self.next_pid);
        self.next_pid += 1;
        pid
    }

    pub fn insert(&mut self, process: Process) {
        let pid = process.pid;
        self.processes.insert(pid, process);
    }

    pub fn get(&self, pid: Pid) -> Option<&Process> {
        self.processes.get(&pid)
    }

    pub fn get_mut(&mut self, pid: Pid) -> Option<&mut Process> {
        self.processes.get_mut(&pid)
    }
}

pub fn kill_user_task(pid: Pid) {
    crate::log_info!("killing user task pid={}", pid.0);
}
