# Physical Memory Manager

## Memory Map

The bootloader provides a memory map via `BootInfo::memory_map_ptr`. Each
entry is a `MemoryRegion` with start, length, and type.

## Frame Allocator

The kernel uses a bitmap-based frame allocator for 4KiB physical frames.

```
Bitmap: 1 bit per frame (0 = free, 1 = used)
```

### Layout

- Bitmap is placed at `kernel_phys_end`
- Size = total_frames / 64 u64 entries
- All frames are initially free
- Kernel image, BootInfo, framebuffer, and memory map frames are reserved early

### API

```rust
FrameAllocator::new(bitmap, total_frames)
alloc() -> Option<PhysFrame>
dealloc(frame)
reserve_region(start, length)
stats() -> (total, used, free)
```

### Invariants

- Never returns a frame that is already marked used
- Double-free is not detected (no safety net in release)
- Reserved regions are never allocated
- Allocator is protected by a spinlock in multi-CPU scenarios
