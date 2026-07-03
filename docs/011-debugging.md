# カーネルデバッグガイド

## QEMU

```bash
make run
```

Docker 開発環境内でビルドしてから、シリアル出力を接続した QEMU でカーネルを起動する。
終了したい場合はターミナルで QEMU を停止する。

普段の開発では、コンテナの作成・破棄を毎回行わないよう常駐コンテナを使う。

```bash
docker compose up -d dev
docker compose exec dev python3 -m pytest tests/boot/test_boot.py::test_kernel_start -v
```

対象テストを先に実行し、全テストは節目や統合前に実行する。現在の boot smoke は
session fixture で QEMU 出力を共有する。

`tests/kernel/` は予定テストとして skip している。新しい kernel 機能に着手するときは、
対象テストを skip から戻し、期待した理由で失敗することを確認してから実装する。

## GDB

`tools/run_qemu.py` に GDB 待機オプションはまだない。必要になったら、同じ pflash/IDE
構成に `-s -S` を足して起動し、別のターミナルから `gdb` で接続する。

## シリアルログ

カーネルログはすべて COM1 シリアルポートへ出力する。
QEMU のシリアル出力はテストハーネスが取得する。

## Panic

panic 時はカーネルがレジスタ状態を出力して停止する。
命令ポインタを解決するにはシンボル検索ヘルパーを使う。

```bash
python3 tools/symbols.py --addr 0xffff...
```
