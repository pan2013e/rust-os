#![allow(unused)]

use core::arch::asm;

#[allow(dead_code)]
pub const SBI_SET_TIMER: i32 = 0x0;
pub const SBI_PUTCHAR  : i32 = 0x1;
pub const SBI_SHUTDOWN : i32 = 0x8;

#[allow(dead_code)]
pub struct SbiRet {
    error : i64,
    value : i64
}

#[inline(always)]
pub fn sbi_ecall(ext : i32, fid: i32,
    arg0 : u64, arg1 : u64, arg2 : u64,
    arg3 : u64, arg4 : u64, arg5 : u64) -> SbiRet {
    let mut err : i64;
    let mut val : i64;
    unsafe {
        asm!(
            "mv a0, {arg0}",
            "mv a1, {arg1}",
            "mv a2, {arg2}",
            "mv a3, {arg3}",
            "mv a4, {arg4}",
            "mv a5, {arg5}",
            "mv a6, {fid}",
            "mv a7, {ext}",
            "ecall",
            "mv {err}, a0",
            "mv {val}, a1",
            arg0 = in(reg) arg0,
            arg1 = in(reg) arg1,
            arg2 = in(reg) arg2,
            arg3 = in(reg) arg3,
            arg4 = in(reg) arg4,
            arg5 = in(reg) arg5,
            fid = in(reg) fid,
            ext = in(reg) ext,
            err = out(reg) err,
            val = out(reg) val
        );
    }
    return SbiRet { error : err, value : val};
}