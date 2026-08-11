// テーブルアライメント対応は未実装のため #[allow(dead_code)] を残す
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Center,
    Right,
    None,
}

#[derive(Debug, Clone)]
pub struct ListItem {
    pub content: Vec<Inline>,
    pub children: Vec<Block>,
}

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum Block {
    Heading {
        level: u8,
        content: Vec<Inline>,
    },
    PageBreak,
    Paragraph {
        content: Vec<Inline>,
    },
    BulletList {
        items: Vec<ListItem>,
    },
    OrderedList {
        items: Vec<ListItem>,
        start: u64,
    },
    Table {
        headers: Vec<Vec<Inline>>,
        rows: Vec<Vec<Vec<Inline>>>,
        alignments: Vec<Alignment>,
    },
    CodeBlock {
        lang: Option<String>,
        code: String,
    },
    Image {
        alt: String,
        path: String,
    },
    BlockQuote {
        children: Vec<Block>,
    },
    ThematicBreak,
}

#[derive(Debug, Clone)]
pub enum Inline {
    Text(String),
    Code(String),
    Bold(Vec<Inline>),
    Italic(Vec<Inline>),
    Equal(Vec<Inline>),
    Link { text: Vec<Inline>, url: String },
    SoftBreak,
    HardBreak,
}

impl Inline {
    /// インライン要素からプレーンテキストを抽出する
    pub fn to_plain_text(&self) -> String {
        match self {
            Inline::Text(s) => s.clone(),
            Inline::Code(s) => format!("「{}」", s),
            Inline::Bold(children) | Inline::Italic(children) | Inline::Equal(children) => {
                children.iter().map(|c| c.to_plain_text()).collect()
            }
            Inline::Link { text, .. } => text.iter().map(|c| c.to_plain_text()).collect(),
            Inline::SoftBreak | Inline::HardBreak => String::new(),
        }
    }
}
