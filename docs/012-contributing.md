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

- Treat `main` as the latest integration branch. It should be kept bootable,
  but when that is uncertain, verify it before using it as a baseline.
- Before starting a larger feature, run the current smoke tests and note whether
  the branch starts from a known-good state.
- Use feature branches for functional areas: `feature/<name>`.
- Keep each feature branch focused on one area, such as paging, interrupts,
  scheduling, syscalls, filesystem, or userspace.
- Merge a feature branch only after tests pass and boot behavior is checked.
- Prefer squash merge to main unless the branch has a useful hand-written commit
  sequence that should remain visible.

## Testing

Run build and QEMU smoke tests inside the Docker environment:

```bash
docker compose run --rm dev python3 -m pytest tests/ -v
```

The Docker image installs Rust nightly, QEMU, OVMF, mtools, and pytest. Local
host runs are useful only if the same tools are installed outside Docker.

1. Run the relevant test first while developing.
2. Run the full pytest suite before merging.
3. Ensure boot output is checked before merging.
4. New features need smoke tests.

## TDD Workflow

Use a small red-green-refactor loop for new behavior.

1. Pick one observable behavior.
2. Add or update one test that describes that behavior.
3. Run the relevant test and confirm it fails for the expected reason.
4. Implement the smallest change that makes the test pass.
5. Run the relevant test again.
6. Refactor only after the test is green.
7. Run the broader smoke suite before merging the feature branch.

Tests should exercise public behavior. For this OS, good tests usually inspect
serial output, process exit behavior, filesystem results, syscall return values,
or user program behavior. Avoid tests that only lock in private implementation
details.

When a feature is difficult to test end-to-end at first, add a tracer-bullet
smoke test that proves the path is reachable, then deepen coverage as the
interface becomes clearer.

## Merge Checklist

- Branch is based on a recently checked `main`.
- The feature has at least one behavior-level test or a documented reason why it
  cannot be tested yet.
- Relevant tests pass in Docker.
- `docker compose run --rm dev python3 -m pytest tests/ -v` passes before merge,
  unless the failure is explicitly documented.
- Boot regression has been checked through QEMU output.
- Docs are updated when behavior, architecture, or workflow changes.
