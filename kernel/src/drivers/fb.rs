use core::ptr;
use lazy_static::lazy_static;
use limine::request::FramebufferRequest;
use spin::Mutex;

#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

lazy_static! {
    static ref FRAMEBUFFER: Mutex<Framebuffer> = Mutex::new(Framebuffer::default());
}

#[derive(Default)]
struct Framebuffer {
    address: *mut u8,
    width: u64,
    height: u64,
    pitch: u64,
    bpp: u16,
}

impl Framebuffer {
    fn put_pixel(&mut self, x: u64, y: u64, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let offset = y * self.pitch + x * (self.bpp as u64 / 8);
        unsafe {
            ptr::write_volatile(self.address.add(offset as usize) as *mut u32, color);
        }
    }
}

unsafe impl Send for Framebuffer {}
unsafe impl Sync for Framebuffer {}

pub fn init() {
    let response = FRAMEBUFFER_REQUEST.get_response().unwrap();
    let fb = response.framebuffers().next().unwrap();

    let mut framebuffer = FRAMEBUFFER.lock();
    framebuffer.address = fb.addr();
    framebuffer.width = fb.width();
    framebuffer.height = fb.height();
    framebuffer.pitch = fb.pitch();
    framebuffer.bpp = fb.bpp();
}

pub fn clear(color: u32) {
    let mut fb = FRAMEBUFFER.lock();

    for y in 0..fb.height {
        for x in 0..fb.width {
            fb.put_pixel(x, y, color);
        }
    }
}

pub fn draw_rect(x: u64, y: u64, w: u64, h: u64, color: u32) {
    let mut fb = FRAMEBUFFER.lock();

    for dx in 0..w {
        fb.put_pixel(x + dx, y, color);
    }
    for dx in 0..w {
        fb.put_pixel(x + dx, y + h - 1, color);
    }
    for dy in 0..h {
        fb.put_pixel(x, y + dy, color);
    }

    for dy in 0..h {
        fb.put_pixel(x + w - 1, y + dy, color);
    }
}

pub fn fill_rect(x: u64, y: u64, w: u64, h: u64, color: u32) {
    let mut fb = FRAMEBUFFER.lock();

    for dx in x..x + w {
        for dy in y..y + h {
            fb.put_pixel(dx, dy, color);
        }
    }
}
