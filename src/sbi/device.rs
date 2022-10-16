use crate::sbi::ecall::{sbi_ecall, SBI_SHUTDOWN};

pub fn shutdown() {
    sbi_ecall(SBI_SHUTDOWN, 0, 
        0, 0, 0, 0, 0, 0);
}