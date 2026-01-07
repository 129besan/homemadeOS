use core::ffi::c_void;

pub type Handle = *mut c_void;

#[repr(C)]
pub struct TableHeader {
    pub signature: u64,
    pub revision: u32,
    pub size: u32,
    pub crc32: u32,
    pub reserved: u32,
}

#[repr(C)]
pub struct BootServices {
    pub header: TableHeader,
    pub raise_tpl: usize,
    pub restore_tpl: usize,
    pub allocate_pages: usize,
    pub free_pages: usize,
    pub get_memory_map: usize,
    pub allocate_pool: usize,
    pub free_pool: usize,
    pub create_event: usize,
    pub set_timer: usize,
    pub wait_for_event: usize,
    pub signal_event: usize,
    pub close_event: usize,
    pub check_event: usize,
    pub install_protocol_interface: usize,
    pub reinstall_protocol_interface: usize,
    pub uninstall_protocol_interface: usize,
    pub handle_protocol: usize,
    pub reserved: usize,
    pub register_protocol_notify: usize,
    pub locate_handle: usize,
    pub locate_device_path: usize,
    pub install_configuration_table: usize,
    pub load_image: usize,
    pub start_image: usize,
    pub exit: usize,
    pub unload_image: usize,
    pub exit_boot_services: usize,
    pub get_next_monotonic_count: usize,
    pub stall: usize,
    pub set_watchdog_timer: usize,
    pub connect_controller: usize,
    pub disconnect_controller: usize,
    pub open_protocol: usize,
    pub close_protocol: usize,
    pub open_protocol_information: usize,
    pub protocols_per_handle: usize,
    pub locate_handle_buffer: usize,
    pub locate_protocol: usize,
    pub install_multiple_protocol_interfaces: usize,
    pub uninstall_multiple_protocol_interfaces: usize,
    pub calculate_crc32: usize,
    pub copy_mem: usize,
    pub set_mem: usize,
    pub create_event_ex: usize,
}

#[repr(C)]
pub struct SimpleTextOutput {
    pub reset: usize,
    pub output_string: usize,
    pub test_string: usize,
    pub query_mode: usize,
    pub set_mode: usize,
    pub set_attribute: usize,
    pub clear_screen: usize,
    pub set_cursor_position: usize,
    pub enable_cursor: usize,
    pub mode: *mut c_void,
}

#[repr(C)]
pub struct SystemTable {
    pub header: TableHeader,
    pub firmware_vendor: *mut u16,
    pub firmware_revision: u32,
    pub console_in_handle: Handle,
    pub con_in: *mut c_void,
    pub console_out_handle: Handle,
    pub con_out: *mut SimpleTextOutput,
    pub standard_error_handle: Handle,
    pub std_err: *mut SimpleTextOutput,
    pub runtime_services: *mut c_void,
    pub boot_services: *mut BootServices,
    pub number_of_table_entries: usize,
    pub configuration_table: *mut c_void,
}

pub fn print(system_table: &SystemTable, msg: &[u16]) {
    if let Some(con_out) = unsafe { system_table.con_out.as_mut() } {
        let func: extern "efiapi" fn(*mut SimpleTextOutput, *mut u16) -> usize =
            unsafe { core::mem::transmute(con_out.output_string) };
        func(con_out as *mut _, msg.as_ptr() as *mut _);
    }
}
