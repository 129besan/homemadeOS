# 現在の状態

このリポジトリには Docker ベースの OS ビルド環境と QEMU テスト環境がある。

## ブランチ

現在の統合ブランチは `main`。今後の作業では対象を絞った `feature/*` ブランチを使う。

現在の `main` はカーネルエントリーのスモークテストが通る baseline である。

## 動作しているもの

- Docker イメージに Rust nightly、QEMU、OVMF、mtools、pytest が入る。
- ブートローダーを `x86_64-unknown-uefi` 向けにビルドできる。
- カーネルを独自 x86_64 ターゲット向けにビルドできる。
- `tools/build_image.py` でディスクイメージを生成できる。
- QEMU が UEFI 経由で起動する。
- ブートローダーが `Hello from MyOS!` を出力する。
- ブートローダーが静的カーネル ELF をロードする。
- カーネルのシリアル出力が `kernel started` まで到達する。
- `ExitBootServices` は memory map key が変わった場合に再試行する。
- UEFI memory map を BootInfo の memory regions へ変換して渡せる。
- UEFI GOP から取得したフレームバッファ情報を BootInfo 経由で渡せる。

## 現在通る確認項目

最初のブートスモークテストは通る。

```bash
docker compose run --rm dev python3 -m pytest tests/boot/test_boot.py::test_kernel_start -v
```

期待する出力には次が含まれる。

```text
kernel started
```

全体テストは boot smoke と tools test が通り、`tests/kernel/` は予定テストとして skip する。

```bash
docker compose run --rm dev python3 -m pytest tests/ -v
```

現在の期待値は `7 passed, 27 skipped`。

## 残っている問題

- `tests/kernel/` の多くは、動作中の機能ではなく予定している振る舞いを記述している。
- 予定テストは単語の部分一致が多く、機能ごとに観測可能な振る舞いへ書き直す必要がある。
- GitHub Actions は失敗し続ける状態を避けるため、現在は手動実行のみである。

## 修正済みの問題

元の失敗には複数の原因があった。

- workspace ビルドでカーネルのリンカーフラグが適用されていなかった。
- カーネル ELF が静的実行ファイルではなく PIE/DYN として生成されていた。
- ブートローダーが埋め込み ELF ヘッダを、アラインされていない可能性のある型付き参照で読んでいた。
- UEFI ブートローダーがカーネルの SysV ABI ではなく UEFI/Win64 ABI でカーネルを呼んでいた。
- シリアルドライバが I/O ポート用の x86 `in`/`out` 命令ではなく、メモリの volatile アクセスを使っていた。

## よく使うコマンド

Docker でビルドとテストを実行する。

```bash
make build
make run
make test
docker compose run --rm dev python3 -m pytest tests/ -v
```

Docker 内でカーネル ELF を調査する。

```bash
docker compose run --rm dev readelf -h target/x86_64-unknown-none/debug/kernel
docker compose run --rm dev readelf -l target/x86_64-unknown-none/debug/kernel
```

カーネルまたはブートローダーの再ビルド後にイメージを再生成する。

```bash
docker compose run --rm dev python3 tools/build_image.py
```

## 主要ファイル

- `bootloader/src/main.rs`
- `bootloader/src/elf_loader.rs`
- `bootloader/src/handoff.rs`
- `kernel/src/main.rs`
- `kernel/linker.ld`
- `kernel/x86_64-myos.json`
- `kernel/.cargo/config.toml`
- `tools/build_image.py`
- `tools/run_qemu.py`
- `tests/boot/test_boot.py`
