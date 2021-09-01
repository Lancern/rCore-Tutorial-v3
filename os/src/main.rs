#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

extern crate log;

#[macro_use]
mod console;
mod lang_items;
mod sbi;

global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();

    console::init_log();

    log::error!("Hello, world!");
    log::warn!("Hello, world!");
    log::info!("Hello, world!");
    log::debug!("Hello, world!");
    log::trace!("Hello, world!");

    log::debug!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    log::debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    log::debug!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    log::debug!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    log::debug!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
}
