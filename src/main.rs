#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::alloc;
use core::alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};

struct KernelAllocator { address: usize };

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
	
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    }
}

#[global_allocator]
static A: KernelAllocator = KernelAllocator { address: 0x9000 };

#[unsafe(no_mangle)]
pub extern "C" fn _kernel_start() -> ! {
	const VALUE: *mut u8 = 0x8050 as *mut u8;
	
	let my_vec: Vec<u8> = 10;

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

/*unsafe fn kernel_print(msg: String) {
	todo!();
}*/


#[panic_handler]
fn panic<'b>(_: &PanicInfo::<'b>) -> ! {
	loop {}
}
