mod config;
mod converter;
mod heading;
mod ir;
mod parser;
mod styles;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

use crate::config::Config;

// NOTE: after_long_help 内のデフォルト値は config.rs のデフォルト関数と同期すること。
// README.md の設定例も同様。変更時は 3 箇所を合わせて更新する。
#[derive(Parser)]
#[command(
    name = "mdd",
    version,
    about = "Markdown ファイルを Word (.docx) に変換する CLI ツール",
    after_long_help = "\
使用例:
  mdd document.md                          入力と同名の .docx を生成
  mdd document.md -o report.docx           出力先を指定
  mdd document.md -c mdd.toml              設定ファイルを指定
  mdd document.md -o out.docx -c my.toml   両方指定

設定ファイル (TOML):
  省略時はデフォルト値が使われます。全項目省略可能。

  [fonts]
  body_ja    = \"游明朝\"        # 本文の日本語フォント
  body_en    = \"Century\"       # 本文の英語フォント
  heading_ja = \"游ゴシック\"    # 見出しの日本語フォント
  heading_en = \"Century\"       # 見出しの英語フォント

  [sizes]                       # 単位: pt
  body     = 10.5               # 本文
  table_body = 9.5              # 表本文
  table_header = 9.5            # 表ヘッダー
  heading1 = 14.0               # 見出し1
  heading2 = 12.0               # 見出し2
  heading3 = 11.0               # 見出し3
  heading4 = 11.0               # 見出し4
  heading5 = 10.5               # 見出し5

  [page]                        # 単位: twip
  width         = 11906         # ページ幅 (既定: A4 縦)
  height        = 16838         # ページ高さ
  margin_top    = 1985          # 上余白
  margin_right  = 1701          # 右余白
  margin_bottom = 1701          # 下余白
  margin_left   = 1701          # 左余白
  margin_header = 851           # ヘッダー余白
  margin_footer = 992           # フッター余白
  margin_gutter = 0             # とじしろ

  [indent]                      # 単位: twip (1 twip = 1/20 pt, 210 twip ≒ 全角1文字)
  body_left       = 210         # 本文の左インデント
  body_first_line = 210         # 本文の字下げ
  body_right      = 210         # 本文の右インデント
  body_left_chars = 100         # 本文の左インデント (文字数×100)
  heading1_left   = 420         # 見出し1の左インデント
  heading1_hanging = 420        # 見出し1のぶら下げインデント
  heading2_left   = 612         # 見出し2の左インデント
  heading2_hanging = 612        # 見出し2のぶら下げインデント
  heading3_left   = 783         # 見出し3の左インデント
  heading3_hanging = 783        # 見出し3のぶら下げインデント
  heading4_left    = 709        # 見出し4の左インデント
  heading4_hanging = 709        # 見出し4のぶら下げインデント
  heading5_left   = 709         # 見出し5の左インデント
  heading5_hanging = 709        # 見出し5のぶら下げインデント
  heading6_left   = 709         # 見出し6の左インデント
  heading6_hanging = 709        # 見出し6のぶら下げインデント

  [bullet]
  level0 = \"●\"                # 箇条書きレベル0
  level1 = \"■\"                # 箇条書きレベル1
  level2 = \"▲\"                # 箇条書きレベル2

  [numbering]
  figure_format = \"sequential\"   # 図番号の形式（sequential / chapter）
  table_format  = \"sequential\"   # 表番号の形式（sequential / chapter）

  [line_numbers]
  enabled  = false              # 行番号を有効にする（true / false）
  count_by = 1                  # N 行ごとに番号を表示
  start    = 1                  # 開始番号
  restart  = \"newPage\"          # 採番リセット: newPage / newSection / continuous

  [code_block]
  border = false                # コードブロックを罫線で囲む（true / false）

対応する Markdown 要素:
  見出し (H1-H5, 自動採番)    段落                  箇条書き (ネスト対応)
  番号付きリスト (ネスト対応)  表 (自動表番号付与)   コードブロック
  画像 (自動図番号付与)        改ページ (`\\pagebreak`)   水平線
  インライン: テキスト / コード / 太字 / 斜体 / リンク"
)]
struct Cli {
    /// 変換する Markdown ファイルのパス
    input: PathBuf,

    /// 出力ファイルパス [省略時: <入力ファイル名>.docx]
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// 設定ファイルパス (TOML) [省略時: デフォルト設定]
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 設定ファイルの読み込み
    let config = match &cli.config {
        Some(path) => Config::load(path)
            .with_context(|| format!("設定ファイルの読み込みに失敗: {}", path.display()))?,
        None => Config::default(),
    };

    // 入力ファイルの読み込み
    let input_path = &cli.input;
    let markdown = std::fs::read_to_string(input_path)
        .with_context(|| format!("入力ファイルの読み込みに失敗: {}", input_path.display()))?;

    // 出力パスの決定
    let output_path = cli.output.unwrap_or_else(|| {
        let mut p = input_path.clone();
        p.set_extension("docx");
        p
    });

    // ベースパス（画像の相対パス解決用）
    let base_path = input_path.parent().unwrap_or_else(|| Path::new("."));

    // Markdown → IR
    let blocks = parser::parse_markdown(&markdown)
        .with_context(|| format!("Markdownの解釈に失敗: {}", input_path.display()))?;

    // IR → docx
    let docx = converter::convert_to_docx(&blocks, &config, base_path)?;

    // ファイル書き出し
    let file = std::fs::File::create(&output_path)
        .with_context(|| format!("出力ファイルの作成に失敗: {}", output_path.display()))?;

    let mut xml_docx = docx.build();

    // 行番号が有効な場合、document.xml の <w:sectPr> に <w:lnNumType> を注入する
    if config.line_numbers.enabled {
        let ln_xml = format!(
            r#"<w:lnNumType w:countBy="{}" w:start="{}" w:restart="{}"/>"#,
            config.line_numbers.count_by,
            config.line_numbers.start,
            config.line_numbers.restart,
        );
        let doc_str = String::from_utf8(std::mem::take(&mut xml_docx.document))
            .context("document.xml が有効な UTF-8 ではありません")?;
        // 最後の </w:sectPr> の直前に挿入（メインセクションプロパティを対象にする）
        if let Some(pos) = doc_str.rfind("</w:sectPr>") {
            let mut modified = doc_str;
            modified.insert_str(pos, &ln_xml);
            xml_docx.document = modified.into_bytes();
        }
    }

    xml_docx.pack(file)?;

    println!("変換完了: {}", output_path.display());
    Ok(())
}
