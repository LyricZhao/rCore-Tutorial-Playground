use riscv::register::{sstatus::Sstatus, scause::Scause};

#[repr(C)]
pub struct TrapFrame {
    pub x: [usize; 32],     // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize,
    pub scause: Scause,
    pub stval: usize,
}