# 仮想メモリ配置

## アドレス空間の配置

```
0x0000_0000_0000_0000 - 0x0000_7fff_ffff_ffff  ユーザー空間 (128 TiB)
0xffff_8000_0000_0000 - 0xffff_8fff_ffff_ffff  物理メモリの直接マップ (64 TiB)
0xffff_9000_0000_0000 - 0xffff_9fff_ffff_ffff  カーネルヒープ (64 TiB)
0xffff_a000_0000_0000 - 0xffff_afff_ffff_ffff  カーネルスタック (64 TiB)
0xffff_b000_0000_0000 - 0xffff_bfff_ffff_ffff  MMIO
0xffff_c000_0000_0000 - 0xffff_ffff_ffff_ffff  カーネルイメージ / モジュール
```

## ページテーブル階層

x86_64 は 4 階層のページテーブルを使う。

- PML4  (index 39:47)  - 512 entries
- PDPT  (index 30:38)  - 512 entries
- PD    (index 21:29)  - 512 entries
- PT    (index 12:20)  - 512 entries

各末端エントリは 4 KiB ページをマップする。

## 権限モデル

| 領域 | Supervisor | User |
|-------------|------------|------|
| カーネルコード | RX | - |
| カーネルデータ | RW | - |
| 直接マップ | RW (NX) | - |
| ユーザーコード | - | RX |
| ユーザーデータ | - | RW |
| null ページ | 未マップ | 未マップ |
