use std::fs::File;
use std::os::unix::io::AsRawFd;
use wayland_client::{
    protocol::{wl_buffer::WlBuffer, wl_shm, wl_shm::WlShm},
    Display, GlobalManager,
};
use wayland_protocols::wlr::screencopy::v1::client::zwlr_screencopy_manager_v1::ZwlrScreencopyManagerV1;

/// Struct to represent the screenshot buffer (equivalent to `struct grim_buffer` in C)
pub struct GrimBuffer {
    pub wl_buffer: WlBuffer,
    pub data: *mut u8, // Pointer to raw pixel data
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub size: usize,
    pub format: wl_shm::Format,
}

impl GrimBuffer {
    /// Creates a new shared memory buffer for Wayland (mimics `create_buffer()` in C)
    pub fn create(display: &Display, shm: &WlShm, width: i32, height: i32) -> Self {
        let stride = width * 4; // Assuming ARGB8888 (4 bytes per pixel)
        let size = (stride * height) as usize;

        // Create shared memory file (equivalent to shm_open in C)
        let mut file = tempfile::tempfile().expect("Failed to create temporary file");
        file.set_len(size as u64).expect("Failed to resize file");

        // Create Wayland shared memory pool
        let pool = shm.create_pool(file.as_raw_fd(), size as i32);
        let wl_buffer = pool.create_buffer(0, width, height, stride, wl_shm::Format::Argb8888);

        Self {
            wl_buffer,
            data: std::ptr::null_mut(), // Will be mapped later
            width,
            height,
            stride,
            size,
            format: wl_shm::Format::Argb8888,
        }
    }

    /// Destroys the buffer (equivalent to `destroy_buffer()` in C)
    pub fn destroy(self) {
        self.wl_buffer.destroy();
    }
}

/// Captures the screen by requesting a frame from `wlr_screencopy`
pub fn capture_screen(
    display: &Display,
    screencopy_manager: &ZwlrScreencopyManagerV1,
    shm: &WlShm,
) {
    let width = 1920; // Replace with actual screen width
    let height = 1080; // Replace with actual screen height

    // Create a Wayland buffer
    let buffer = GrimBuffer::create(display, shm, width, height);

    // Request a screen capture and attach the buffer
    let _frame = screencopy_manager.capture_output(0, &buffer.wl_buffer);

    println!("✅ Requested screenshot capture with buffer");
}

