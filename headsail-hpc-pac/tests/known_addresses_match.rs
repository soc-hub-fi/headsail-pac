// This warning is incorrect in our case; formatting is not actually fully
// supported in panic
#![allow(non_fmt_panics)]

use headsail_hpc_pac as pac;

fn verify_ptr<T>(ptr: *const T, expected_addr: usize) {
    let addr = ptr as usize;

    if addr != expected_addr {
        panic!(format!(
            "incorrect address in memory map:\nexpected: 0x{expected_addr:x}\n  actual: 0x{addr:x}"
        ))
    }
}

#[test]
fn sysctrl() {
    verify_ptr(pac::SYSCTRL::ptr(), 0x1_FF90_0000);
}
