# 物理メモリマネージャ

## メモリマップ

ブートローダーは `BootInfo::memory_map_ptr` を通してメモリマップを渡す。
各エントリは開始位置、長さ、種別を持つ `MemoryRegion` である。

## フレームアロケータ

カーネルは 4 KiB の物理フレームを管理するビットマップ方式のアロケータを使う。

```
ビットマップ: 1 フレームにつき 1 bit（0 = 空き、1 = 使用中）
```

### 配置

- ビットマップは `kernel_phys_end` に置く
- サイズは `total_frames / 64` 個の `u64`
- 初期状態では全フレームを空きとする
- カーネルイメージ、BootInfo、フレームバッファ、メモリマップのフレームは早期に予約する

### API

```rust
FrameAllocator::new(bitmap, total_frames)
alloc() -> Option<PhysFrame>
dealloc(frame)
reserve_region(start, length)
stats() -> (total, used, free)
```

### 不変条件

- 使用中と記録されたフレームは返さない
- 二重解放は検出しない（release ビルドに安全策はない）
- 予約領域は割り当てない
- 複数 CPU で使う場合はスピンロックで保護する
