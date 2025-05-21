#![no_std]
#![no_main]

use limine::BaseRevision;

mod drivers;
mod panic_handler;
mod writer;

#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    drivers::fb::init();
    drivers::fb::clear(0xFF00FF00);
    drivers::fb::fill_rect(50, 50, 100, 100, 0xFF000000);

    loop {
        core::hint::spin_loop();
    }
}
