use crate::sched::task::Pid;

pub enum ProcessState {
    Alive,
    Zombie,
}

pub struct Process {
    pub pid: Pid,
    pub state: ProcessState,
}

pub fn kill_user_task(pid: Pid) {
    crate::log_info!("killing user task pid={}", pid.0);
}
