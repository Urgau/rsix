#if !defined(__LP64__)
#error "x32 is not yet supported"
#endif

// x86-64 syscall argument register ordering is the same as the x86-64
// userspace argument register ordering except that a3 is passed in r10
// instead of rcx, and the syscall number (nr) is passed in eax.
//
// outline.rs takes care of reordering the nr argument to the end for us,
// so we only need to move nr into eax and move rcx into r10 as needed.

    .file       "x86_64.S"
    .intel_syntax noprefix

    .section    .text.rsix_reordered_syscall0,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall0
    .hidden     rsix_reordered_syscall0
    .type       rsix_reordered_syscall0, @function
rsix_reordered_syscall0:
    .cfi_startproc
    mov    eax,edi
    syscall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall0, .-rsix_reordered_syscall0

    .section    .text.rsix_reordered_syscall1,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall1
    .hidden     rsix_reordered_syscall1
    .type       rsix_reordered_syscall1, @function
rsix_reordered_syscall1:
    .cfi_startproc
    mov    eax,esi
    syscall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall1, .-rsix_reordered_syscall1

    .section    .text.rsix_reordered_syscall1_noreturn,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall1_noreturn
    .hidden     rsix_reordered_syscall1_noreturn
    .type       rsix_reordered_syscall1_noreturn, @function
rsix_reordered_syscall1_noreturn:
    .cfi_startproc
    mov    eax,esi
    syscall
    ud2
    .cfi_endproc
    .size rsix_reordered_syscall1_noreturn, .-rsix_reordered_syscall1_noreturn

    .section    .text.rsix_reordered_syscall2,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall2
    .hidden     rsix_reordered_syscall2
    .type       rsix_reordered_syscall2, @function
rsix_reordered_syscall2:
    .cfi_startproc
    mov    eax,edx
    syscall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall2, .-rsix_reordered_syscall2

    .section    .text.rsix_reordered_syscall3,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall3
    .hidden     rsix_reordered_syscall3
    .type       rsix_reordered_syscall3, @function
rsix_reordered_syscall3:
    .cfi_startproc
    mov    eax,ecx
    syscall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall3, .-rsix_reordered_syscall3

    .section    .text.rsix_reordered_syscall4,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall4
    .hidden     rsix_reordered_syscall4
    .type       rsix_reordered_syscall4, @function
rsix_reordered_syscall4:
    .cfi_startproc
    mov    eax,r8d
    mov    r10,rcx
    syscall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall4, .-rsix_reordered_syscall4

    .section    .text.rsix_reordered_syscall5,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall5
    .hidden     rsix_reordered_syscall5
    .type       rsix_reordered_syscall5, @function
rsix_reordered_syscall5:
    .cfi_startproc
    mov    eax,r9d
    mov    r10,rcx
    syscall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall5, .-rsix_reordered_syscall5

    .section    .text.rsix_reordered_syscall6,"ax",@progbits
    .p2align    4
    .globl      rsix_reordered_syscall6
    .hidden     rsix_reordered_syscall6
    .type       rsix_reordered_syscall6, @function
rsix_reordered_syscall6:
    .cfi_startproc
    mov    eax,DWORD PTR [rsp+0x8]
    mov    r10,rcx
    syscall
    ret
    .cfi_endproc
    .size rsix_reordered_syscall6, .-rsix_reordered_syscall6

    .section .note.GNU-stack,"",@progbits
