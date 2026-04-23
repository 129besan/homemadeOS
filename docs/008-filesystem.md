# VFS と Initramfs

## VFS アーキテクチャ

- `FileOps` trait: read、write、seek
- `InodeOps` trait: open、lookup
- `FileTable`: プロセスごとのファイルディスクリプタ表

## Initramfs

初期ファイルシステムは initramfs アーカイブとしてカーネルイメージに埋め込む。

### 形式

```
[header] [entry table] [string table] [file data]

header:
  magic: [u8; 8] = "INITRAMF"
  file_count: u32
  string_table_offset: u32
  data_offset: u32

entry:
  name_offset: u32
  data_offset: u32
  data_len: u32
```

### マウント

カーネル初期化時に `mount_root(boot_info)` が initramfs を解析し、ルートファイルシステムを作る。

## デバイスノード

- `/dev/console`: シリアルコンソールへの書き込み専用出力
- `/dev/null`: すべての書き込みを受け取る
