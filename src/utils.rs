use core::arch::asm;
use core::panic::PanicInfo;
use crate::error;

static mut GDB_ATTACHED: bool = false;

pub fn wait_for_debugger() {
    unsafe {
        while !GDB_ATTACHED {
            asm!("pause");
        }
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "[-] Panic in {} at ({}, {}):",
            location.file(),
            location.line(),
            location.column()
        );

        if let Some(message) = info.message() {
            error!("[-] Panic message: {}", message);
        }
        else {
            error!("[-] Panic occured");
        }
    }

    loop {
        unsafe { asm!("cli; hlt") };
    }
}