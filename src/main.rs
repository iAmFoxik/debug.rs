#![no_std]
#![no_main]
#![feature(panic_info_message)]

use uefi::prelude::*;

mod logger;
mod utils;
mod port;

#[entry]
fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let boot_services = system_table.boot_services();
    #[cfg(debug_assertions)]
    {
        utils::wait_for_debugger();
    }
    core::panic!("Error");
    
    Status::SUCCESS
}
