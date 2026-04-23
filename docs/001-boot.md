# ブート引き渡し契約

## BootInfo ABI

ブートローダーは標準呼出規約を使って、カーネルのエントリーポイントへ
`BootInfo` 構造体を渡す。x86_64 では `rdi` に `BootInfo` のポインタを格納する。

```rust
pub const BOOT_INFO_MAGIC: u64 = 0x4d_59_4f_53_42_49_00_01;
pub const BOOT_INFO_VERSION: u32 = 1;

pub struct BootInfo {
    pub magic: u64,           // must equal BOOT_INFO_MAGIC
    pub version: u32,         // must equal BOOT_INFO_VERSION
    pub _padding: u32,
    pub memory_map_ptr: u64,  // physical address of memory map
    pub memory_map_len: u64,  // number of entries
    pub framebuffer_base: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_stride: u32,
    pub framebuffer_format: u32,
    pub kernel_phys_start: u64,
    pub kernel_phys_end: u64,
    pub initramfs_start: u64,
    pub initramfs_len: u64,
    pub rsdp_addr: u64,
}
```

## 所有権

- `BootInfo` はブートローダーが確保した固定物理アドレスに置く。
- カーネルはページテーブルを変更する前に、必要なデータをコピーする。
- メモリマップのエントリとフレームバッファは、記録されたアドレスで有効である。

## UEFI の注意点

- `ExitBootServices` 後は、カーネルがランタイム領域を自分でマップしない限り、
  UEFI ランタイムサービスを利用できない。
- 引き渡し後、ブートローダーはプール割り当てに依存してはならない。
- 終了後のタイマーと割り込みの状態は未定義である。
