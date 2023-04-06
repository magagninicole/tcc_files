.section .rodata

.global HEAP_START
HEAP_START: .dword __heap_start

.global HEAP_SIZE
HEAP_SIZE: .dword __heap_size

.global TEXT_START
TEXT_START: .dword _text_start

.global TEXT_END
TEXT_END: .dword _text_end

.global RODATA_START
RODATA_START: .dword _rodata_start

.global RODATA_END
RODATA_END: .dword _rodata_end

.global DATA_START
DATA_START: .dword _data_start

.global DATA_END
DATA_END: .dword _data_end

.global BSS_START
BSS_START: .dword _bss_start

.global BSS_END
BSS_END: .dword _bss_end

.global STACK_START
STACK_START: .dword __stack_start

.global STACK_END
STACK_END: .dword __stack_end

.global UART_ADDRESS
UART_ADDRESS: .dword __uart_base_addr

.global KERNEL_STACK_END
KERNEL_STACK_END: .dword __stack_end
