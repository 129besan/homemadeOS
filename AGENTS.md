このファイルはエージェント向けの作業メモです。詳しい設計は `docs/` を見る。

## この repo の前提

- x86_64 UEFI boot の学習用 OS。
- Rust `no_std` kernel + UEFI bootloader + QEMU smoke tests。
- 現在の実装構成は `bootloader/`, `kernel/`, `libs/`, `userspace/`, `tools/`, `tests/`, `docs/`。
- 現在の状態などを`docs/`以下のファイルに記録していく。

## 作業方針

- 機能単位で `feature/*` ブランチを切る。
- TDD っぽく進める。まず観測できる振る舞いを 1 つ選び、赤いテストを確認してから最小修正で緑にする。
- OS のテストは基本的に Docker 内で実行する。

```bash
docker compose run --rm dev python3 -m pytest tests/ -v
```

- テストを書いたら実行して結果を確認する。
- 作業前に `git status --short --branch` を見る。
- 既存のユーザー変更は勝手に戻さない。

## コミットルール

- commit message は `docs/012-contributing.md` の type を使い、1 commit につき 1 つの概念にする。
- 長過ぎるcommit message は避ける。


