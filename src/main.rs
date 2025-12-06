#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use core::panic::PanicInfo;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

pub unsafe fn uart_putc(c: u8) {
    unsafe {
        core::ptr::write_volatile(0x0900_0000 as *mut u8, c);
    }
}

pub unsafe fn uart_print(s: &str) {
	unsafe {
		for c in s.chars() {
			let mut bytes: [u8; 4] = [0; 4];
			c.encode_utf8(&mut bytes);

			for i in 0..bytes.len() {
				let byte = bytes[i];
				
				if byte != 0 {
					uart_putc(byte);
				}
			}
		}
	}
}

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
fn handle_alloc_error(_layout: Layout) -> ! {
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

#[unsafe(export_name = "efi_main")]
pub extern "C" fn efi_main(_h: *mut core::ffi::c_void, _st: *mut core::ffi::c_void) -> usize {
	
	unsafe { 
		uart_print("was geht ab in rumänien?"); 
		halt(0);
	}

	loop {}
    	0
}

struct KernelAllocator {  }

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
	let size = layout.size();
	let align = layout.align();

	let address: usize = unsafe { core::ptr::read_volatile(VALUE) };

	// Overshoot relative to the last possible address
	let overshoot: usize = address % align;

	// Undershoot relative to the next possible address
	let undershoot: usize = align  - overshoot;

	let effective_address = address + undershoot;

	unsafe {
		core::ptr::write_volatile(VALUE, effective_address + size + 1);
	}
	
	return effective_address as *mut u8
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
    }
}

#[global_allocator]
static A: KernelAllocator = KernelAllocator {  };

const VALUE: *mut usize = 0x10_00_00 as *mut usize;

unsafe fn halt(code: u64) -> ! {
	// Arm64 halt
	// Ask qemu to stop
	#[cfg(target_arch="aarch64")]
	unsafe {
		core::arch::asm!(
			"mov x1, {0}",
			"mov x0, 0x18", // EXIT
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
