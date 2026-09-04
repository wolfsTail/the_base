.section .bss
buf:    .space 32  # нулевой буфер в 32 байта, дял вывода в терминал

.section .text
.globl _start  # метка точки входа
    add:  # метка функция add
        xorl %eax, %eax  # очистить rax
        addq %rsi, %rdi  # rdi = rdi + rsi
        addq %rdi, %rax  # rax = rax + rdi
        ret

    print_num:  # метка функции печати числа
            xorl %eax, %eax  # очистить rax
            addq %rdi, %rax  # rax = rax + rdi (rax был 0, так что это копирование rdi -> rax; div умеет делить только rax)
            movq $buf+31, %rsi  # положить адрес байта буфера в rsi, типо указатель для заполнения байтов на печать
            movb $'\n', (%rsi)  # по этому адресу пишем байт конца строки в память
            movq $1, %rcx  # используем счетчик для хранения размера строки (щас 1 байт = \n)
        while:  # метка цикла while
            xorq %rdx, %rdx  # нулим rdx, по спеке divq (результат в rdx и rax)
            movq $10, %rbx  # делитель 10 в регистр
            divq %rbx  # делим rax на 10 лежащем в rbx
            addq $'0', %rdx  # приводим к ascii собирая байт значения
            decq %rsi  # сдвигаем указатель на байт влево
            movb %dl, (%rsi)  # пишем сиивол в буфер (dl = младший байт rdx)
            incq %rcx  # байтов в строке стало на 1 больше
            testq %rax, %rax  # у нас ноль? пишем какие-то биты в Rflags
            jnz while  # проверяем ьиты в rflags и в случае если они не установлены идем по тметке

            movq %rcx, %rdx # длина строки для печати из счетчика
            movq $1, %rax  # пушим номер write
            movq $1, %rdi # пушим файловый дескриптор stdout
            syscall # rsi держит указатель на начало строки в памяти
            ret

    _start:
        movq $2, %rdi  # a
        movq $2, %rsi  # b
        call add # в rax результат

        movq %rax, %rdi  # 1ый и единственный параметр print_num
        call print_num

        movq $231, %rax
        movq $0, %rdi
        syscall  # ок
