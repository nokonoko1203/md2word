# mdd

MarkdownファイルをWord(.docx)に変換するCLIツール。

![alt text](images/README/image.png)

## 対応するMarkdown要素

| 要素             | 記法                | 変換時の処理                              |
| ---------------- | ------------------- | ----------------------------------------- |
| 見出し           | `# H1` ～ `##### H5` | 自動採番付き。H1/H2はWord上で前後に間隔を付与 |
| 段落             | 通常のテキスト      | 字下げ・余白をtwip単位で制御              |
| 箇条書き         | `- item`            | ネスト対応、行頭文字カスタマイズ可（●■▲） |
| 番号付きリスト   | `1. item`           | ネスト対応、開始番号指定可                |
| 表               | GFM形式             | 自動採番（連番 or 章番号付き、設定で切替） |
| 画像             | `![alt](path)`      | 自動採番（連番 or 章番号付き）、形式自動変換 |
| コードブロック   | ` ``` `             | Courier New / MS ゴシック、9pt            |
| 改ページ         | `\pagebreak`        | Wordの改ページを挿入                      |
| 水平線           | `---`               | 空段落に変換                              |
| 太字             | `**text**`          | 対応                                      |
| 斜体             | `*text*`            | 対応                                      |
| 文字装飾         | `==text==`          | 設定した文字サイズ・背景色を適用          |
| インラインコード | `` `code` ``        | 「」で囲んで表示                          |
| リンク           | `[text](url)`       | ハイパーリンク化、アンカーリンクにも対応  |

## pandocとの違い

pandocは汎用の文書変換ツールだが、mddは日本語のWord文書を作ることに特化している。

- **日本語フォントがデフォルト**。本文は游明朝、見出しは游ゴシック。英語フォントも別途指定でき、混植が自然になる。
- **見出しの自動採番**。H1からH5まで階層的に番号を振る（1 → 1.1 → 1.1.1 → (1) → ①）。既存の番号があれば重複を避ける。
- **表・画像の自動採番**。連番（図1, 図2…）と章番号付き（図1.1, 図1.2, 図2.1…）を設定で切り替えられる。章番号付きではH1の番号を基準にし、H1が変わるとリセットされる。
- **TOML設定ファイル一つで制御**。フォント、サイズ、ページ設定、インデント、行頭文字をまとめて指定できる。pandocのようにテンプレートdocxを用意する必要がない。
- **英日間スペースの自動削除**。日本語と英語の境界にある不要なスペースを消して、組版を整える。
- **シングルバイナリ**。Rustで書かれておりLaTeXやPython環境が不要。`cargo install`だけで使える。

## インストール

Rust 1.70以上が必要。

コマンドとしてインストールする場合は以下を実行する。

```sh
cargo install --path .
```

$HOME/.cargo/bin/mddにバイナリが入る。PATHが通っていなければ、~/.zshrcなどに次の行を足す。

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

ビルドだけしたい場合はこちら。

```sh
cargo build --release
```

バイナリはtarget/release/mddに生成される。

## 使い方

```sh
mdd <入力ファイル> [オプション]
```

mdd --helpで詳細を確認できる。

| オプション      | 説明                                                                              |
| --------------- | --------------------------------------------------------------------------------- |
| `-o, --output`  | 出力先を指定する。省略すると入力ファイル名の拡張子を `.docx` に変えたものになる。 |
| `-c, --config`  | 設定ファイル (TOML) を指定する。省略するとデフォルト設定が使われる。              |
| `-h, --help`    | ヘルプを表示する。`--help` なら設定ファイルの書式も出る。                         |
| `-V, --version` | バージョンを確認する。                                                            |

```sh
# 基本的な変換（document.docxが生成される）
mdd document.md

# 出力先を指定
mdd document.md -o output.docx

# 設定ファイルを指定
mdd document.md -o output.docx -c mdd.toml
```

## 設定ファイル

TOML形式でフォントやサイズをカスタマイズできる。すべての項目は省略可能で、省略した項目にはデフォルト値が入る。全項目とデフォルト値はmdd --helpで確認できる。

```toml
[fonts]
body_ja = "游明朝"
body_en = "Century"
heading_ja = "游ゴシック"
heading_en = "Century"

[sizes]
body = 10.5
table_body = 9.5
table_header = 9.5
heading1 = 14.0
heading2 = 12.0
heading3 = 11.0
heading4 = 11.0
heading5 = 10.5

[page]
width = 11906
height = 16838
margin_top = 1985
margin_right = 1701
margin_bottom = 1701
margin_left = 1701
margin_header = 851
margin_footer = 992
margin_gutter = 0

[indent]
body_left = 210
body_first_line = 210
body_right = 210
body_left_chars = 100
heading1_left = 420
heading1_hanging = 420
heading2_left = 612
heading2_hanging = 612
heading3_left = 783
heading3_hanging = 783
heading4_left = 709
heading4_hanging = 709
heading5_left = 709
heading5_hanging = 709
heading6_left = 709
heading6_hanging = 709

[bullet]
level0 = "●"
level1 = "■"
level2 = "▲"

[numbering]
figure_format = "sequential"
table_format = "sequential"

[equal]
enabled = true
font_size = 18.0
background_color = "#FFFF00"
```

fontsセクションでは本文と見出しそれぞれの日本語フォント、英語フォントを指定する。sizesセクションで本文、表、各レベルの見出し（H1〜H5）のフォントサイズをpt単位で設定する。

pageセクションではページサイズと余白をtwip単位で設定する。既定値はA4縦相当で、画像の最大幅もこのページ幅と左右余白から自動計算される。そのため、横長の画像でも本文幅に収まる。レイアウトを変えたい場合は `width` と `margin_left` / `margin_right` を主に調整すればよい。

indentセクションのtwipはWordの内部単位で、1twipは1/20pt。210twipがおおむね全角1文字分にあたる。body_left_charsはWord独自の文字数単位で、100が1文字に相当する。見出し1〜6の左インデントとぶら下げインデントも個別に変更できる。

bulletセクションで箇条書きの各レベルに使う行頭文字を変更できる。

numberingセクションで図番号・表番号の採番形式を指定する。`"sequential"`は連番（図1, 図2, 図3…）、`"chapter"`は章番号付き（図1.1, 図1.2, 図2.1…）になる。章番号はH1（見出し1）の番号を基準とし、H1が変わるとリセットされる。H2以下の変化ではリセットされない。

equalセクションで`==text==`の文字装飾を指定する。`enabled = true`の場合のみ記法を有効にする。`font_size`を省略すると周囲と同じ文字サイズになり、`background_color`を省略すると背景色を付けない。背景色は`"#FFFF00"`または`"FFFF00"`のようなRGB形式で指定する。equalセクション自体を省略した場合、文字装飾は無効になる。

## 対応するMarkdown要素

見出しはH1からH5まで対応しており、自動採番が付く。H1とH2にはWord上で前後の段落間隔を設定し、章や節の区切りが詰まりすぎないようにしている。段落、箇条書き、番号付きリストはネストにも対応する。表には自動で表番号が振られ、画像には図番号が付く。表本文と表ヘッダーの文字サイズは`sizes.table_body`と`sizes.table_header`で変更できる。採番形式は設定で連番・章番号付きを切り替えられる。コードブロック、改ページ、水平線にも対応している。インライン要素としてはテキスト、コード、太字、斜体、`==text==`による文字装飾、リンクを扱える。

## 改ページ

段落単位で`\pagebreak`のみを書いた場合、その位置にWordの改ページを挿入する。

```md
# 1章
本文

\pagebreak

# 2章
次のページから始まる本文
```

`\pagebreak`を通常段落や見出し、コードブロックなどに混在させた場合は、曖昧な解釈を避けるためエラーにする。
