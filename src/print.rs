use alloc::string::String;

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

pub unsafe fn uart_print_string(s: String) {
	uart_print(s.as_str());
}

#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ({
		unsafe { $crate::print::uart_print_string(format!($($arg)*)) }
	})
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
