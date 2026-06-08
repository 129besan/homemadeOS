use crate::handoff::PixelFormat;
use crate::uefi::{Guid, LocateProtocolFn, SystemTable};
use core::ffi::c_void;

const EFI_SUCCESS: usize = 0;
const GRAPHICS_OUTPUT_PROTOCOL_GUID: Guid = Guid {
    data1: 0x9042_a9de,
    data2: 0x23dc,
    data3: 0x4a38,
    data4: [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
};

#[repr(C)]
pub struct Framebuffer {
    pub base: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: PixelFormat,
}

#[repr(C)]
struct GraphicsOutputProtocol {
    query_mode: usize,
    set_mode: usize,
    blt: usize,
    mode: *mut GraphicsOutputProtocolMode,
}

#[repr(C)]
struct GraphicsOutputProtocolMode {
    max_mode: u32,
    mode: u32,
    info: *mut GraphicsOutputModeInformation,
    size_of_info: usize,
    frame_buffer_base: u64,
    frame_buffer_size: usize,
}

#[repr(C)]
struct GraphicsOutputModeInformation {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: u32,
    pixel_information: PixelBitmask,
    pixels_per_scan_line: u32,
}

#[repr(C)]
struct PixelBitmask {
    red: u32,
    green: u32,
    blue: u32,
    reserved: u32,
}

pub fn detect_framebuffer(system_table: &SystemTable) -> Option<Framebuffer> {
    let boot_services = unsafe { system_table.boot_services.as_ref()? };
    let locate_protocol: LocateProtocolFn = unsafe { core::mem::transmute(boot_services.locate_protocol) };
    let mut interface: *mut c_void = core::ptr::null_mut();
    let status = locate_protocol(
        &GRAPHICS_OUTPUT_PROTOCOL_GUID,
        core::ptr::null_mut(),
        &mut interface,
    );
    if status != EFI_SUCCESS || interface.is_null() {
        return None;
    }

    let gop = unsafe { &*(interface as *const GraphicsOutputProtocol) };
    let mode = unsafe { gop.mode.as_ref()? };
    let info = unsafe { mode.info.as_ref()? };
    let format = pixel_format_from_gop(info.pixel_format)?;

    Some(Framebuffer {
        base: mode.frame_buffer_base,
        width: info.horizontal_resolution,
        height: info.vertical_resolution,
        stride: info.pixels_per_scan_line,
        format,
    })
}

fn pixel_format_from_gop(format: u32) -> Option<PixelFormat> {
    match format {
        0 => Some(PixelFormat::RedGreenBlue),
        1 => Some(PixelFormat::BlueGreenRed),
        2 => Some(PixelFormat::PixelBitMask),
        3 => Some(PixelFormat::PixelBltOnly),
        4 => Some(PixelFormat::PixelFormatMax),
        _ => None,
    }
}
