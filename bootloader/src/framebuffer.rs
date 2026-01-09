use crate::handoff::PixelFormat;

#[repr(C)]
pub struct Framebuffer {
    pub base: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: PixelFormat,
}

pub fn detect_framebuffer() -> Option<Framebuffer> {
    None
}
