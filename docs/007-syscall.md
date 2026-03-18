# Syscall ABI

## Calling Convention

Arguments passed in registers:

| Register | Purpose |
|----------|---------|
| rax      | syscall number |
| rdi      | arg0 |
| rsi      | arg1 |
| rdx      | arg2 |
| r10      | arg3 |
| r8       | arg4 |
| r9       | arg5 |
| rax      | return value |

## Syscall Table

| # | Name   | Description          |
|---|--------|----------------------|
| 0 | exit   | terminate process    |
| 1 | write  | write to fd          |
| 2 | read   | read from fd         |
| 3 | open   | open file            |
| 4 | close  | close fd             |
| 5 | spawn  | create process       |
| 6 | wait   | wait for child       |
| 7 | getpid | get process id       |
| 8 | yield  | yield CPU            |
| 9 | mmap   | allocate memory      |
| 10| munmap | free memory          |

## Error Handling

Negative return values indicate errors (errno style):
- `-1` EPERM
- `-9` EBADF
- `-14` EFAULT
- `-38` ENOSYS
