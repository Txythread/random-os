#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start_kernel() -> ! {
	let GPIO_PIN_0: *mut u8 = 0x8050 as *mut u8;
	let mut value = 10;
	loop {
		value += 1;
		unsafe {
			core::ptr::write_volatile(GPIO_PIN_0, 0x1);
		}
	}
}


#[panic_handler]
fn panic<'b>(_: &PanicInfo::<'b>) -> ! {
	loop {}
}
