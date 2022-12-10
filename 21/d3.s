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

; 12 byte chars to 12 word ints
align 32
bytes_to_words:
    db 0, -1
    db 1, -1
    db 2, -1
    db 3, -1
    db 4, -1
    db 5, -1
    db 6, -1
    db 7, -1
    db 8, -1
    ; starting from 0 again since the upper bytes in the shuffle mask only
    ; address the upper bytes in the source
    db 9, -1
    db 10, -1
    db 11, -1
    db (32 - 12) dup -1

align 32
ascii_0:
    db 32 dup '0'

align 32
five_hundred:
    dw 16 dup 500

part1:
    ; rcx = pointer to current puzzle input
    lea rcx, [rel puzzle_input]
    ; r10 = puzzle end
    lea r10, [rel puzzle_input_end]

    vpxor ymm0, ymm0, ymm0
    vpxor ymm1, ymm1, ymm1
    vpxor ymm2, ymm2, ymm2

    .loop:
        cmp rcx, r10
        je .end
        ; load the line
        vmovups ymm1, [rcx]
        ; advance to the next line
        lea rcx, [rcx + 13]
        ; shuffle into word lanes
        ; subtract ascii 0
        vpsubb ymm1, ymm1, [rel ascii_0]
        ; repeat the first 16 x 8 in the high 128 bits of ymm1
        ; this is to make it possible for the shuffle mask to address these bytes
        ; in  = a b
        ; in2 = x
        ; out = a x
        vinserti128 ymm1, ymm1, xmm1, 1
        vpshufb ymm2, ymm1, [rel bytes_to_words]
        vpaddw ymm0, ymm0, ymm2
        jmp .loop

    .end:

    vpcmpgtw ymm1, ymm0, [rel five_hundred]
    vpmovmskb rdi, ymm1

    mov rax, 0b10_10_10_10_10_10_10_10_10_10_10_10
    pext rdi, rdi, rax

    ; reverse the 12 bits
    xor esi, esi
    and eax, 1
    shl eax, 11
    xor edx, edx
    %rep 12
    .rev:
        bt edi, esi
        jnc .not
        bts edx, eax
        .not:

        dec eax
        inc esi
        cmp esi, 12
        jne .rev

    mov rdi, rdx

    mov rax, rdi
    not rax
    and rax, 0b111111111111
    and rdi, 0b111111111111
    ;int3
    imul rdi, rax

    call format_dec
    call puts

    ret

part2:
    ; scratch_buffer = stride lengths
    ; on the first time around the strides are all 1
    ; edi = loop counter, we know there are 1k inputs
    xor edi, edi
    .initial_stride_loop:
        mov [rel scratch_buffer + edi], 1
        ; we know there are 1k inputs
        inc edi
        cmp edi, 999
        jne .initial_stride_loop

    ret

global _start
_start:
    call part1
    call part2
    call exit

section .data

scratch_buffer:
    db 4096 dup 0

; buffer to put stuff in
output_buffer_used:
    dq 0
output_buffer:
    db 400 dup 0

align 32
parsed_input:
    dd 4000 dup 0
parsed_input_end: equ $

; puzzle input, input file
align 32
puzzle_input:
   incbin "d3.txt"
puzzle_input_end: equ $
align 256
