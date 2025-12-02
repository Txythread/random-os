#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // Simple non-overlap implementation (good enough for kernel use)
    unsafe {
        let mut i = 0usize;
        while i < n {
            ptr::write(dest.add(i), ptr::read(src.add(i)));
            i += 1;
        }
        dest
    }
}

#[alloc_error_handler]
fn handle_alloc_error(layout: Layout) -> ! {
    unsafe { halt(10) }
}


#[unsafe(no_mangle)]
pub extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    unsafe {
        let mut i = 0usize;
        while i < n {
            ptr::write(s.add(i), c as u8);
            i += 1;
        }
        s
    }
}


struct KernelAllocator {  }

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
	let size = layout.size();
	let align = layout.align();

	let address: usize = core::ptr::read_volatile(VALUE);

	// Overshoot relative to the last possible address
	let overshoot: usize = address % align;

	// Undershoot relative to the next possible address
	let undershoot: usize = align  - overshoot;

	let effective_address = address + undershoot;

	core::ptr::write_volatile(VALUE, effective_address + 1);

	
	return effective_address as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    }
}

#[global_allocator]
static A: KernelAllocator = KernelAllocator {  };

const VALUE: *mut usize = 0x10_00_00 as *mut usize;

#[unsafe(no_mangle)]
pub extern "C" fn _kernel_start() -> ! {
	let my_vec: alloc::vec::Vec<u8> = alloc::vec![10];

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
