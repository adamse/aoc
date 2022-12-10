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

; u8x32 newlines
align 32
newlines:
    db 32 dup 10

; u32x8 mulipliers
align 32
multipliers_1:
    dd 1
    dd 7 dup 0
multipliers_2:
    dd 10, 1
    dd 6 dup 0
multipliers_3:
    dd 100, 10, 1
    dd 5 dup 0
multipliers_4:
    dd 1000, 100, 10, 1
    dd 4 dup 0
multipliers_5:
    dd 10_000, 1000, 100, 10, 1
    dd 3 dup 0
multipliers_6:
    dd 100_000, 10_000, 1000, 100, 10, 1
    dd 2 dup 0
multipliers_7:
    dd 1000_000, 100_000, 10_000, 1000, 100, 10, 1
    dd 0
multipliers_8:
    dd 10_000_000, 1000_000, 100_000, 10_000, 1000, 100, 10, 1

align 32
write_zero_shuf_mask:
    db 32 dup 0xf

align 32
test_string:
    db '123',10
    db '66',10
    db '112',10
    db '91',10
    db '1111',10
    db '1111',10
    db '1111',10
    db '1111',10
    db '1111',10
    db '1111',10

align 32
ascii_0:
    times 16 db '0'

align 32
byte_0:
    db 16 dup 0

; shufb byte mask to set the byte to 0
%define shufz 0b10_00_00_00

; how to create the shuffle masks to deposit the digits of two numbers in the right places?
;

align 32
; shuffle 4 to 1 ascii digits into u16 lanes
shuffle_digits_into_u16:
    ; 4 digits
    db 0, shufz
    db 1, shufz
    db 2, shufz
    db 3, shufz
    db 8 dup shufz
    ; 3 digits
    db shufz, shufz
    db 0, shufz
    db 1, shufz
    db 2, shufz
    db 8 dup shufz
    ; 2 digits
    db shufz, shufz
    db shufz, shufz
    db 0, shufz
    db 1, shufz
    db 8 dup shufz
    ; 1 digit
    db shufz, shufz
    db shufz, shufz
    db shufz, shufz
    db 0, shufz
    db 8 dup shufz

align 32
multipliers_4_16:
    dw 1000, 100, 10, 1
    dw 1000, 100, 10, 1

; parse ascii numbers delimited by '\n's, with simd!!!
; two at a time
parse_ascii_number_many_at_a_time:
    ; r8 = pointer where to write next input
    lea r8, [rel parsed_input]

    ; rsi = pointer to next input to parse
    lea rsi, [rel puzzle_input]

    .loop:
        ; check if we've reached the end of the input
        lea rdx, [rel puzzle_input_end]
        cmp rsi, rdx
        je .loop_end

        ; load string into xmm0
        vmovups xmm0, [rsi]
        ; compare with newlines, the ones that match will be 0xff others 0
        vpcmpeqb xmm1, xmm0, [rel newlines]
        ; smash the "one byte per bit" sized equality from the compare into bits
        vpmovmskb eax, xmm1

        ; subtract ascii '0' to find the integer value
        vpsubb xmm0, xmm0, [rel ascii_0]

        ; find the no of digits and shuffle mask for the first number

            ; trailing zero count: ecx will contain the number of digits in the
            ; first number
            tzcnt ecx, eax

            ; next number past the no of digits we saw now +1 for the newline
            ; for the next input
            lea rsi, [rsi + rcx + 1]

            ; we want to shuffle in 4-ecx 0s
            mov edx, 4
            sub edx, ecx

            ; add one to account for the newline for the next number
            inc ecx

            ; multiply by 16, to find the right offset into shuffle_digits_into_u16
            sal edx, 4

            ; load the shuffle mask for the first number
            vmovaps xmm1, [rel shuffle_digits_into_u16 + edx]

        ; find the no of digits and shuffle mask for the second number

            ; shift out the digits+newline bits from the newline comparison mask
            shr eax, cl ; cl = lower bits of ecx

            ; ecx is the byte offset for the start of the second number, store
            ; this so we can add it to the shuffle mask
            vmovd xmm2, ecx
            vpbroadcastb xmm2, xmm2

            ; find the number of digits in the second number
            tzcnt ecx, eax

            ; next number past the no of digits we saw now +1 for the newline
            ; for the next input
            lea rsi, [rsi + rcx + 1]

            ; we want to shuffle in 4-ecx 0s
            mov edx, 4
            sub edx, ecx

            ; add one to account for the newline for the next number
            inc ecx

            ; multiply by 16, to find the right offset into shuffle_digits_into_u16
            sal edx, 4

            ; load the shuffle mask for the first number and add the byte offset
            ;
            ; need to add the byte position of the first digit in this number
            ; to the shuffle mask so we actually pick the digits for the second number.
            ;
            ; it is ok to just add the byte across the board: the places where
            ; we need vpshufb to write a zero only require the high bit being
            ; set, and that will be preserved since the byte offset is at most
            ; 16 and we can store 255 in a byte.
            vpaddb xmm2, xmm2, [rel shuffle_digits_into_u16 + edx]

            ; shuffle the mask into place so we can blend it with the mask for
            ; the first number
            ;
            ; xmm2 = a b c d (4 x 32)
            ; xmm2 = c d a b, afterwards (4 x 32)
            vpshufd xmm2, xmm2, 0b01_00_11_10

        ; combine the shuffle masks for two numbers
        ; xmm1 = a b . .
        ; xmm2 = . . c d
        ; xmm1 = a b c d, afterwards
        vpblendd xmm1, xmm1, xmm2, 0b11_00

        ; shuffle the bytes into u16 lanes
        ; xmm1 = a3 a2 a1 a0 b3 b2 b1 b0, (8 x 16) afterwards
        vpshufb xmm2, xmm0, xmm1

        ; multiply by the position value, and add adjecent words
        ; in  = a b c d e f g h
        ; out = a+b c+d e+f g+h (but also first multiplied by their magnitudes!)
        vpmaddwd xmm3, xmm2, [rel multipliers_4_16]

        ; in  = a b c d
        ; out = b . d .
        vpshufd xmm4, xmm3, 0b11_11_01_01
        ; add =
        vpaddd xmm5, xmm3, xmm4

        ; and we're done!

        ; extract the first number and store it
        vpextrd eax, xmm5, 0
        mov [r8 + 0], eax

        ; extract the second number and store it
        vpextrd eax, xmm5, 2
        mov [r8 + 4], eax

        add r8, 8

        jmp .loop

    .loop_end:
    mov [rel parsed_input_end], r8
    ret

align 32
first_shuf_mask:
;  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31
; a0 a1 a2 a3
; a0  0  0  0 a1  0  0  0 a2  0  0  0 a3  0  0  0
;  0 ff ff ff  1 ff ff ff  2 ff ff ff  3 ff ff ff
    ; pick first byte
    db 0
    db 3 dup 0xff
    ; pick second byte
    db 1
    db 3 dup 0xff
    ; pick third byte
    db 2
    db 3 dup 0xff
    ; pick fourth byte
    db 3
    db 3 dup 0xff

    ; db 28 dup 0xff ;0b1000_0000
    ; db 0, 0xf, 0xf, 0xf
    ; db 1, 0xf, 0xf, 0xf
    ; db 2, 0xf, 0xf, 0xf
    ; db 3, 0xf, 0xf, 0xf
    ; db 4, 0xf, 0xf, 0xf
    ; db 16 dup 0xf

align 32
shuffle_digits_into_u32:
    ; 4 digits
    db 0, shufz, shufz, shufz,
    db 1, shufz, shufz, shufz,
    db 2, shufz, shufz, shufz,
    db 3, shufz, shufz, shufz,

    ; 3 digits
    db shufz, shufz, shufz, shufz,
    db 0,     shufz, shufz, shufz,
    db 1,     shufz, shufz, shufz,
    db 2,     shufz, shufz, shufz,

    ; 2 digits
    db shufz, shufz, shufz, shufz,
    db shufz, shufz, shufz, shufz,
    db 0,     shufz, shufz, shufz,
    db 1,     shufz, shufz, shufz,

    ; 1 digit
    db shufz, shufz, shufz, shufz,
    db shufz, shufz, shufz, shufz,
    db shufz, shufz, shufz, shufz,
    db 0,     shufz, shufz, shufz,

; parse ascii numbers delimited by '\n's, with simd!!!
parse_ascii_number_one_at_a_time:
    ; rcx = pointer where to write next input
    lea rcx, [rel parsed_input]

    ; rsi = pointer to next input to parse
    lea rsi, [rel puzzle_input]

    .loop:
    ; check if we've reached the end of the input
    lea rdx, [rel puzzle_input_end]
    cmp rsi, rdx
    je .end

    ; load string into xmm0
    vmovups xmm0, [rsi]
    ; compare with newlines, the ones that match will be 0xff others 0
    vpcmpeqb xmm1, xmm0, [rel newlines]
    vpmovmskb eax, xmm1
    ; edi will contain the number of digits in the first number
    tzcnt edi, eax

    ; next number past the no of digits we saw now +1 for the newline
    ; for the next input
    lea rsi, [rsi + rdi + 1]

    ; we want to shuffle in 4-edi 0s
    mov edx, 4
    sub edx, edi

    ; multiply by 16, to find the right offset into shuffle_digits_into_u32
    sal edx, 4

    ; subtract ascii '0' to find the integer value
    vpsubb xmm0, xmm0, [rel ascii_0]

    ; shuffle bytes into u32s
    vpshufb xmm2, xmm0, [rel shuffle_digits_into_u32 + edx]
    ; multiply by the position value
    vpmulld xmm2, xmm2, [rel multipliers_4]

    ; smash the 4 u32s into 2 u32s
    ; vp unpack hq dq??
    ; xmm2 = a b c d (4 u32s)
    ; xmm3 = c d c d, afterwards
    vpunpckhqdq xmm3, xmm2, xmm2
    ; xmm2 = (a+c) (b+d) ...
    vpaddd xmm2, xmm2, xmm3

    ; smash the 2 u32s into 1 u32
    ; xmm2 = a b c d
    ; xmm3 = b d d d, but we only care that the first one is b
    vpshufd xmm3, xmm2, 0b11_11_11_01
    ; xmm2 = (a+b) ...
    vpaddd xmm2, xmm2, xmm3

    ; move the remaining u32 into eax and return it
    vmovd eax, xmm2
    mov [rcx], rax
    lea rcx, [rcx + 4]

    jmp .loop

    .end:
    mov [rel parsed_input_end], rcx
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
format_dec:
    lea rsi, [rel scratch_buffer]

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


; turn a number into a hex string
;
; puts '0x<number>\n' into output_buffer
;
; arguments
; rdi = number to display
;
; return
; nothing
format_hex:
    ; rsi = scratch buffer pointer
    lea rsi, [rel scratch_buffer]

    ; put lowest digit of number into scratch buffer
    ; and repeat as long as number is not 0
    ; rax = scratch for current digit
    .digits:
        mov rax, rdi
        and rax, 15
        mov r8b, byte [rel hexdigits + rax]
        mov [rsi], byte r8b
        inc rsi
        sub rdi, rax
        shr rdi, 4
        jnz .digits
    .end_digits:

    mov [rel output_buffer + 0], byte '0'
    mov [rel output_buffer + 1], byte 'x'
    lea rdx, [rel output_buffer + 1]
    lea rcx, [rel scratch_buffer]
    .copy_backwards:
        dec rsi
        inc rdx
        ; copy from scratch_buffer(rsi) to output_buffer(rdx)
        mov r8b, byte [rsi]
        mov [rdx], byte r8b
        cmp rsi, rcx
        jne .copy_backwards
    .end_copy_backwards:


    ; end the line
    mov [rdx + 1], byte 10 ; \n
    add rdx, 2
    lea rcx, [rel output_buffer]
    sub rdx, rcx
    mov [rel output_buffer_used], rdx

    ret

; arguments
; rdi = buffer
; rsi = max bytes
;
; return
; rax = read bytes
gets:
    ; max len into third argument
    mov rdx, rsi
    ; buffer address into second argument
    mov rsi, rdi
    ; fd into first argument
    mov rdi, STDIN
    mov rax, SYS_READ
    syscall
    ret

test_input:
    dq 199, 200, 208, 210, 200, 207, 240, 269, 260, 263
test_input_end: equ $

part1:
    ; how many measurements were larger than the previous day?
    ; rdi = result: count of measurements that were larger
    ; rsi = current measurement pointer
    lea rsi, [rel parsed_input]
    xor edi, edi ; 0 greater measurements so far

    .loop:
        ; load yesterdays value
        mov r8d, [rsi]

        ; update pointer
        lea rsi, [rsi + 4]

        ; test if we're at the end
        cmp rsi, [rel parsed_input_end]
        je .loop_end

        ; load todays value
        mov r9d, [rsi]

        ; today greater than yesterday?
        cmp r9d, r8d
        jle .not_greater
        inc edi
        .not_greater:

        jmp .loop

    .loop_end:
    call format_dec
    call puts

    ret

part2:
    ; rsi = pointer to start of window
    lea rsi, [rel parsed_input]
    ; rdx = pointer to end of window
    lea rdx, [rel parsed_input + 3*4]
    xor edi, edi ; 0 greater measurements so far
    mov r8d, [rsi + 0*4]
    add r8d, [rsi + 1*4]
    add r8d, [rsi + 2*4]

    ; r8d = yesterdays value
    .loop:
        ; parsed_input_end points at the last value, but rdx points at the next
        ; slot, so subtract a slot from parsed_input_end to not have off-by-one
        ; errors :)
        mov r9, [rel parsed_input_end]
        sub r9, 4
        cmp rdx, r9
        je .end_loop

        mov r9d, r8d
        ; subtract the value that fell out of the window
        sub r9d, [rsi]
        ; add the current value to the window
        add r9d, [rdx]
        ; check if the current window is larger than the last
        cmp r9d, r8d
        jna .not_above
        inc edi
        .not_above:

        ; copy the current window to the last for the next iteration
        mov r8d, r9d

        ; update pointers to the next window
        lea rsi, [rsi + 4]
        lea rdx, [rdx + 4]

        jmp .loop
    .end_loop:

    call format_dec
    call puts
    ret

global _start
_start:
    call parse_ascii_number_many_at_a_time
    mov rdi, rax
    call format_dec
    call puts

    lea rax, [rel parsed_input]
    mov rdi, [rel parsed_input_end]
    sub rdi, rax
    sar rdi, 2
    call format_dec
    call puts

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

; place to store some stuff in
parsed_input:
    dd 4000 dup 0
parsed_input_end: equ $

; puzzle input, input file
puzzle_input:
   incbin "d1.txt"
puzzle_input_end: equ $
