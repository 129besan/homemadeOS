# Heap Allocator

## Phases

The kernel heap evolves in phases:

1. **Bump allocator** — simple linear allocation, no free. Good for early boot.
2. **Linked-list allocator** — free list with block headers and coalescing.
3. **Slab allocator** (future) — fixed-size caches for common allocation sizes.

## Virtual Range

```
HEAP_START = 0xffff_9000_0000_0000
HEAP_SIZE  = 4 MiB
```

The heap is mapped at boot time. Additional pages are mapped on demand via
`grow()`.

## Linked-List Allocator

Each free block has a header:

```text
+------------------+
| size (8 bytes)   |
| next (8 bytes)   |
+------------------+
| free space       |
+------------------+
```

On `alloc`: traverse free list, split block if remainder > 16 bytes.
On `dealloc`: insert into free list, coalesce with adjacent free blocks.

## Safety

- The allocator is protected by atomic operations.
- Double-free is not detected.
- The global allocator is wired via `#[global_allocator]`.
