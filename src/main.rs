#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let result = core::hint::black_box(fib(30)) as u64;
    unsafe {
        asm!(
            "add a0, zero, {0}",
            "li a7, 93",
            "ecall",
            in(reg) result,
            options(noreturn)
        )

    }

}


fn fib(n: usize) -> usize {
    if n <= 1 { n }
    else { fib(n-1) + fib(n-2) }
}


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}

}


// needed for debug builds cause the darwin target has an std.
// https://users.rust-lang.org/t/unexpected-undefined-reference-to-rust-eh-personality-when-compiling-with-c-panic-abort-for-no-std-library/120311
#[unsafe(no_mangle)]
pub extern "C" fn rust_eh_personality() {}



#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 { unsafe {
    for i in 0..size {
        dst.add(i).write(src.add(i).read());
    }
    return dst;
}}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 { unsafe {
	if dst as *const u8 == src { return dst }

	//if ((uintptr_t)s-(uintptr_t)d-n <= -2*n) return memcpy(d, s, n);

	if (dst as *const u8) < src {
        for i in 0..size {
            dst.add(i).write(src.add(i).read());
        }
	}
    else {
        for i in (0..size).rev() {
            dst.add(i).write(src.add(i).read());
        }
	}

	return dst;
}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dst: *mut u8, value: i32, size: usize) -> *mut u8 { unsafe {
    let value = value as u8;

    for i in 0..size {
        dst.add(i).write(value);
    }

    return dst;
}}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn bzero(dst: *mut u8, size: usize) { unsafe {
    memset(dst, core::hint::black_box(0), size);
}}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(lhs: *const u8, rhs: *const u8, size: usize) -> i32 { unsafe {
    for i in 0..size {
        let l = lhs.add(i).read();
        let r = rhs.add(i).read();
        if l != r {
            return l.wrapping_sub(r) as i32;
        }
    }
    return 0;
}}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn strlen(ptr: *const u8) -> usize { unsafe {
    let mut len = 0;
    while ptr.add(len).read() != 0 {
        len += 1;
    }
    return len;
}}


