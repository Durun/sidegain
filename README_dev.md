# 開発者向けドキュメント

## プロジェクトの初期化方法
新しいプラグインを作るときはテンプレートを使って新規作成すること。
https://github.com/robbert-vdh/nih-plug-template

このプロジェクトをコピペして別のものを作ると、IDが重複するためDAWがプラグインを誤認してしまう。


## 環境構築(Windows)
### Git Bash をインストール
```shell
winget install Git.Git
```
コマンドは原則 Git Bash で実行する想定で説明する。

### Rust toolchain をインストール
https://rustup.rs/ から rustup-init.exe をダウンロードしてきて実行する。

選択肢は以下の通りに選択する。
```
1) Quick install via the Visual Studio Community installer
   (free for individuals, academic uses, and open source).
```
```
1) Proceed with standard installation (default - just press enter)
```

## Building
```shell
cargo xtask bundle side_gain --release
```
