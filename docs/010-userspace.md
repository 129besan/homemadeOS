# Userspace and Shell

## Programs

| Program | Path       | Description         |
|---------|------------|---------------------|
| init    | /init      | First process       |
| shell   | /bin/shell | Interactive shell   |
| hello   | /bin/hello | Print "hello"       |
| echo    | /bin/echo  | Print arguments     |
| cat     | /bin/cat   | Read file to stdout |
| ls      | /bin/ls    | List directory      |

## Shell Features

- Prompt: `$ `
- Line input with backspace
- Whitespace tokenization
- Builtins: `exit`, `cd`, `pwd`
- External command execution from `/bin`

## Init Flow

1. Kernel mounts initramfs as root
2. Kernel spawns `/init`
3. `/init` opens `/dev/console` for stdin/stdout/stderr
4. `/init` spawns `/bin/shell` as the interactive session
5. Shell reads commands and spawns programs
