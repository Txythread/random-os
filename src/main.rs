#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

use alloc::vec;
use core::panic::PanicInfo;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use alloc::format;
use crate::print::*;
use log::info;
use uefi::prelude::*;
use uefi::boot::*;
use uefi::allocator::Allocator;

mod print;
mod memory;


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

#[entry]
fn main() -> Status {
	uefi::helpers::init().unwrap();

	println!("Was geht ab in Rumänien?");

	let was_geht = vec![10, 11];

	println!("Alles geht in Rumänien");

	println!("was geht {}", 10);
	let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let map = unsafe { exit_boot_services(None) };

	unsafe { halt(0); }
	println!("UEFI Services terminated.");



	loop {}



    	Status::SUCCESS
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
static A: Allocator = Allocator;

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
