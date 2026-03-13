use crate::syscall::table::SYSCALL_TABLE;
use crate::arch::x86_64::gdt::KERNEL_CS;

core::arch::global_asm!(
    ".globl syscall_entry",
    ".globl syscall_entry_end",
    "syscall_entry:",
    "    swapgs",
    "    mov gs:[0], rsp",
    "    mov rsp, gs:[8]",
    "    push r11",
    "    push rcx",
    "    sti",
    "    push rdi",
    "    push rsi",
    "    push rdx",
    "    push r10",
    "    push r8",
    "    push r9",
    "    mov rdi, rax",
    "    mov rsi, rdx",
    "    mov rdx, r10",
    "    mov rcx, r8",
    "    mov r8, r9",
    "    call dispatch_syscall",
    "    pop r9",
    "    pop r8",
    "    pop r10",
    "    pop rdx",
    "    pop rsi",
    "    pop rdi",
    "    cli",
    "    pop rcx",
    "    pop r11",
    "    swapgs",
    "    sysretq",
    "syscall_entry_end:",
);

extern "C" {
    pub fn syscall_entry();
}
