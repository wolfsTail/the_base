.section .rodata  # глобальные константы в RO
    string:  #  метка переменной
        .ascii "Hello, World!\n"  # непосредственно байты по текущему адресу, по метке str, в видк ascii
    string_len = . - string # константа string_len, со значением вычисляемым "текущий адрес" - адресc метки str
.section .text
    .globl _start  # точка входа
    _start:
        movq $1, %rax  # кладем в rax -> 1 (syscall == write)
        movq $1, %rdi  # параметр 1 -> rdi, дескриптор файла (1 = stdout)
        movq $string, %rsi # параметр 2 -> rsi, адресс метки string
        movq $string_len, %rdx # параметр 3 -> rdx, количество байт для записи
        syscall  # вызов ядра ОС, syscall write

        movq $231, %rax # кладем в rax -> 231 (syscall == exit_group)
        movq $0, %rdi # код возврата 0 (ok)
        syscall  # вызов ядра ОС, syscall exit_group
