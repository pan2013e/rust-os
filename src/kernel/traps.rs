use crate::println;

use crate::sbi::timer::clock_set_next_event;
use crate::sbi::console::*;

struct Args {
    scause: u64,
    sepc: u64
}

const EXC : usize = 0x0;
const INT : usize = 0x1;

const S_TIMER_INT : usize = 0x5;
const S_EXTRN_INT : usize = 0x9;

const TRAP_ENTRY: [[fn(Args); 16]; 2] = {
    let mut array : [[fn(Args); 16]; 2] = [[not_implemented; 16]; 2];
    array[INT][S_TIMER_INT] = s_timer_int;
    array[INT][S_EXTRN_INT] = s_external_int;
    array
};


#[no_mangle]
pub extern "C" fn trap_handler(scause : u64, sepc : u64) {
    TRAP_ENTRY[int_or_exc(scause)][(scause & 0xF) as usize](Args{scause, sepc});
}

fn int_or_exc(scause : u64) -> usize {
    if (scause as i64) < 0 {
        INT
    } else {
        EXC
    }
}

fn s_timer_int(args : Args) {
    // payload here
    // println!("test");
    clock_set_next_event();
}

fn s_external_int(args: Args) {
    let ch = console_getchar();
    console_echo(ch);
    unsafe {
        BUF[BUF_PTR] = ch;
        BUF_PTR += 1;
    }
}

fn not_implemented(args : Args) {
    panic!("{}", "Not implemented trap.\nsepc: {args.sepc}, scause: {args.scause}");
}