# コントリビューションガイド

## コミット形式

```
<type>(<scope>): <summary>
```

type: chore, docs, boot, kernel, arch, mm, paging, heap, interrupts,
driver, sync, sched, proc, syscall, fs, userlib, userspace, shell,
test, ci, tools, release

1 コミットにつき 1 つの概念を扱う。
AI が生成したような説明的すぎるコミットメッセージは避ける。

## ブランチ戦略

- `main` は最新の統合ブランチとして扱う。起動可能な状態を保つべきだが、不明な場合は
  ベースラインとして使う前に検証する。
- 大きな機能に着手する前に現在のスモークテストを実行し、既知の正常状態から始めるか記録する。
- 機能領域ごとに `feature/<name>` ブランチを使う。
- 各 feature ブランチは paging、interrupts、scheduling、syscalls、filesystem、
  userspace など 1 つの領域に集中させる。
- テスト通過と起動動作を確認してから feature ブランチを統合する。
- 残す価値のある手書きコミット列でなければ、`main` への squash merge を優先する。

## テスト

ビルドと QEMU スモークテストは Docker 環境内で実行する。

```bash
make test
docker compose run --rm dev python3 -m pytest tests/ -v
```

Docker イメージには Rust nightly、QEMU、OVMF、mtools、pytest が入っている。
ホスト上での実行は、同じツールがホストにも入っている場合だけ有効である。

普段の開発では常駐コンテナと対象テストを使い、起動コストを抑える。

```bash
docker compose up -d dev
docker compose exec dev python3 -m pytest tests/boot/test_boot.py::test_kernel_start -v
```

現在 `tests/kernel/` は予定している振る舞いを表すテスト群で、baseline では skip する。
機能を実装するときは対象テストを skip 対象から外し、観測可能な振る舞いとして妥当な
assertion に直してから red-green-refactor を始める。

1. 開発中は関連するテストを先に実行する。
2. 統合前に pytest の全テストを実行する。
3. 統合前に起動出力を確認する。
4. 新機能にはスモークテストを追加する。

## TDD ワークフロー

新しい振る舞いには、小さな red-green-refactor のループを使う。

1. 観測可能な振る舞いを 1 つ選ぶ。
2. その振る舞いを記述するテストを 1 つ追加または更新する。
3. 関連テストを実行し、期待した理由で失敗することを確認する。
4. テストを通す最小の変更を実装する。
5. 関連テストをもう一度実行する。
6. テストが green になってからリファクタリングする。
7. feature ブランチを統合する前に、より広いスモークテストを実行する。

テストでは外部から観測できる振る舞いを検証する。この OS では通常、シリアル出力、
プロセス終了動作、ファイルシステムの結果、システムコールの戻り値、
ユーザープログラムの動作を確認する。内部実装だけを固定するテストは避ける。

最初から機能を end-to-end でテストするのが難しい場合は、その経路へ到達できることを示す
tracer-bullet スモークテストを追加し、インターフェースが明確になるにつれて範囲を深める。

## 統合チェックリスト

- ブランチは直近で確認済みの `main` を基点にしている。
- 機能には振る舞いレベルのテストが 1 つ以上ある。まだテストできない場合は理由を記録している。
- 関連テストが Docker 内で通る。
- 明示的に失敗または skip を記録した場合を除き、統合前に
  `make test` または `docker compose run --rm dev python3 -m pytest tests/ -v` が通る。
- QEMU 出力で起動リグレッションを確認している。
- 振る舞い、アーキテクチャ、ワークフローを変更した場合は docs を更新している。
