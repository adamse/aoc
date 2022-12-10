bits 64

; sysv calling convention
; ~~~~~~~~~~~~~~~~~~~~~~~
; Functions preserve the registers rbx, rsp, rbp, r12, r13, r14, and r15
; while rax, rdi, rsi, rdx, rcx, r8, r9, r10, r11 are scratch registers

; syscall interface
; ~~~~~~~~~~~~~~~~~
; rax = syscall no
; args in rdi, rsi, rdx, r10, r8, r9
; return in rax, rdx

; syscall numbers
; https://filippo.io/linux-syscall-table/
%define SYS_READ 0
%define SYS_WRITE 1
%define SYS_EXIT 60

; argument in rdi
; clobbers rax, but never returns anyway
exit:
    mov rax, SYS_EXIT
    syscall
    ud2

; fd in rdi, string in rsi
write:
    mov rax, SYS_WRITE
    syscall
    ret

%define STDOUT 1
%define STDIN 0

; arguments
; no return
puts:
    mov rdi, STDOUT
    lea rsi, [rel output_buffer]
    mov rdx, qword [rel output_buffer_used]
    mov rax, SYS_WRITE
    syscall
    ret

hexdigits:
    db '0123456789abcdef'

; turn a number into a string
;
; puts '<number>\n' into output_buffer
;
; arguments
; rdi = number to display
;
; return
; nothing
; 1408487760
format_dec:
    lea rsi, [rel scratch_buffer]

    xor r8d, r8d

    mov rax, rdi
    mov rdi, 10
    .digits:
        ; divide rax by 10, quotient in rax, remainder in rdx
        xor rdx, rdx ; not so large my numbers
        div rdi
        mov r8b, byte [rel hexdigits + rdx]
        mov [rsi], byte r8b
        inc rsi
        cmp rax, 0
        jne .digits
    .end_digits:

    lea rdx, [rel output_buffer]
    lea rcx, [rel scratch_buffer]
    .copy_backwards:
        dec rsi
        ; copy from scratch_buffer(rsi) to output_buffer(rdx)
        mov r8b, byte [rsi]
        mov [rdx], byte r8b
        inc rdx
        cmp rsi, rcx
        jne .copy_backwards
    .end_copy_backwards:

    ; end the line
    mov [rdx], byte 10 ; \n
    add rdx, 1
    lea rcx, [rel output_buffer]
    sub rdx, rcx
    mov [rel output_buffer_used], rdx

    ret


part1:
    ; rcx = pointer to current puzzle input
    lea rcx, [rel puzzle_input]
    ; r10 = puzzle end
    lea r10, [rel puzzle_input_end]

    ; r8 = forward position
    xor r8d, r8d

    ; r9 = depth
    xor r9d, r9d

    .next_ins:
        ; check if we've run out of input
        cmp rcx, r10
        je .end

        ; load instruction
        mov esi, [rcx]
        ; load operand
        mov edi, [rcx + 4]

        ; advance to the next instruction
        lea rcx, [rcx + 2*4]

        ; handle the instruction
        jmp [.jumptab + rsi * 8]

    ; operand is in rdi
    .forward:
        add r8, rdi
        jmp .next_ins
    .down:
        add r9, rdi
        jmp .next_ins
    .up:
        sub r9, rdi
        jmp .next_ins

    .end:

    ; construct and print answer
    mov rdi, r8
    imul rdi, r9
    call format_dec
    call puts

    ret

    .jumptab:
        dq .forward, .down, .up

part2:
    ; rcx = pointer to current puzzle input
    lea rcx, [rel puzzle_input]
    ; r11 = puzzle end
    lea r11, [rel puzzle_input_end]

    ; r8 = forward position
    xor r8d, r8d

    ; r9 = aim
    xor r9d, r9d

    ; r10 = depth
    xor r10d, r10d

    .next_ins:
        ; check if we've run out of input
        cmp rcx, r11
        je .end

        ; load instruction
        mov esi, [rcx]
        ; load operand
        mov edi, [rcx + 4]

        ; advance to the next instruction
        lea rcx, [rcx + 2*4]

        ; handle the instruction
        jmp [.jumptab + rsi * 8]

    ; operand is in rdi
    .forward:
        ; go forward
        add r8, rdi
        ; adjust depth according to aim
        imul rdi, r9
        add r10, rdi
        jmp .next_ins
    .down:
        add r9, rdi
        jmp .next_ins
    .up:
        sub r9, rdi
        jmp .next_ins

    .end:

    ; construct and print answer
    mov rdi, r8
    imul rdi, r10
    call format_dec
    call puts
    ret

    .jumptab:
        dq .forward, .down, .up

global _start
_start:
    call part1
    call part2
    call exit

section .data

scratch_buffer:
    db 400 dup 0

; buffer to put stuff in
output_buffer_used:
    dq 0
output_buffer:
    db 400 dup 0

; puzzle input, input file
align 32
puzzle_input:
%include "d2_proc.txt"
puzzle_input_end: equ $
