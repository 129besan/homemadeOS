# Contributing

## Commit Style

```
<type>(<scope>): <summary>
```

Types: chore, docs, boot, kernel, arch, mm, paging, heap, interrupts,
driver, sync, sched, proc, syscall, fs, userlib, userspace, shell,
test, ci, tools, release

One conceptual change per commit.
No AI-generated commit messages.

## Branch Strategy

- `main` is always bootable
- Feature branches: `feature/<name>`
- Squash merge to main

## Testing

1. Run pytest: `python3 -m pytest tests/ -v`
2. Ensure boot test passes before merging
3. New features need smoke tests
