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
