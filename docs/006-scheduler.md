# Scheduler

## Design

Round-robin preemptive scheduler for kernel threads.

## Components

- **Thread**: kernel stack, context, state, TID/PID
- **CpuContext**: callee-saved registers for context switch
- **Scheduler**: run queue (VecDeque), current thread tracking
- **context_switch**: assembly function that saves/restores registers

## States

- `Runnable`: ready to run, in run queue
- `Running`: currently executing
- `Sleeping`: blocked, not in run queue
- `Zombie`: exited, waiting for cleanup

## Context Switch

The context switch saves/restores:
- rsp, r15, r14, r13, r12, rbx, rbp

CS, SS, RIP, RFLAGS are managed by the regular call/ret mechanism
for kernel threads. User threads additionally need an iret frame.

## Preemption

The PIT timer generates an interrupt at ~100 Hz. The timer handler
calls `scheduler::timer_tick()` which yields the current thread.

## Idle Thread

When the run queue is empty, the idle thread runs. It executes `hlt`
to save power until the next interrupt.
