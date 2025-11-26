#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
	const GPIO_PIN_0: *mut u32 = 0x1F000D0000 as *mut u32;
	loop {
		unsafe {
			core::ptr::write_volatile(GPIO_PIN_0, 0x1);
		}
	}
}


#[panic_handler]
fn panic<'b>(_: &PanicInfo::<'b>) -> ! {
	loop {}
}
