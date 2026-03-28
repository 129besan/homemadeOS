use crate::proc::process::ProcessTable;
use crate::proc::elf::{validate_elf, validate_program_headers, map_loadable_segments, build_user_stack, create_main_thread};

pub fn spawn_elf(path: &str, argv: &[&str]) -> Result<u64, ()> {
    crate::log_info!("spawn_elf: path={}", path);
    Err(())
}
