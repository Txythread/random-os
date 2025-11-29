#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _kernel_start() -> ! {
	const VALUE: *mut u8 = 0x8050 as *mut u8;
	loop {
		unsafe {
			core::ptr::write_volatile(VALUE, *VALUE+1);
			halt(0);
		}
	}
}

unsafe fn halt(code: u64) -> ! {
	// Arm64 halt
	// Ask qemu to stop
	#[cfg(target_arch="aarch64")]
	unsafe {
		core::arch::asm!(
			"mov x0, {0}",
			"mov x1, 0x18", // EXIT
			"hlt 0xF000", // semi-hosting call
			in(reg) code,
			options(noreturn)
		);
	}

	#[cfg(target_arch="x86_64")]
	unsafe {
		core::arch::asm!(
			"hlt",
			options(noreturn)
		);
	}
}


#[panic_handler]
fn panic<'b>(_: &PanicInfo::<'b>) -> ! {
	loop {}
}
