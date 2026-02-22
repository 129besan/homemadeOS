# Interrupt Architecture

## Components

- **GDT**: Global Descriptor Table (kernel code/data, user code/data, TSS)
- **TSS**: Task State Segment (IST stacks for double fault)
- **IDT**: Interrupt Descriptor Table (256 entries)
- **PIC**: Legacy interrupt controller (disabled; masked)
- **PIT**: Programmable Interval Timer (periodic interrupt source)

## Interrupt Vectors

| Vector  | Description        |
|---------|--------------------|
| 0x00    | Divide Error       |
| 0x06    | Invalid Opcode     |
| 0x08    | Double Fault       |
| 0x0d    | General Protection |
| 0x0e    | Page Fault         |
| 0x20    | PIT Timer          |
| 0x21    | Keyboard           |
| 0x80    | Syscall (future)   |

## Exception Handling

- All exceptions print diagnostics to serial
- Page fault prints CR2, instruction pointer
- Double fault uses a dedicated IST stack
- Kernel panics on fatal exceptions

## Double Fault IST

The double fault handler has a dedicated stack to guard against stack overflow
causing a triple fault. The stack is set up in the TSS IST1 entry.
