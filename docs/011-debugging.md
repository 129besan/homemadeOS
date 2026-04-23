# カーネルデバッグガイド

## QEMU

```bash
make run
```

シリアル出力を接続した QEMU でカーネルを起動する。

普段の開発では、コンテナの作成・破棄を毎回行わないよう常駐コンテナを使う。

```bash
docker compose up -d dev
docker compose exec dev python3 -m pytest tests/boot/test_boot.py::test_kernel_start -v
```

対象テストを先に実行し、全テストは節目や統合前に実行する。現在のテストは各テスト関数が
QEMU を個別起動するため、全件実行には時間がかかる。

## GDB

```bash
qemu-system-x86_64 -s -S -cdrom myos.iso
```

別のターミナルで次を実行する。
```bash
gdb target/kernel.elf
(gdb) target remote :1234
(gdb) break kernel_main
(gdb) continue
```

## シリアルログ

カーネルログはすべて COM1 シリアルポートへ出力する。
QEMU のシリアル出力はテストハーネスが取得する。

## Panic

panic 時はカーネルがレジスタ状態を出力して停止する。
命令ポインタを解決するにはシンボル検索ヘルパーを使う。

```bash
python3 tools/symbols.py --addr 0xffff...
```
