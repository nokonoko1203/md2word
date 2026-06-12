use crate::config::Config;
use docx_rs::*;

/// pt → half-point (Word内部単位) への変換
/// Word は半ポイント(half-point)単位でフォントサイズを管理する
pub fn pt_to_half_point(pt: f64) -> usize {
    (pt * 2.0) as usize
}

/// pt → twip (1/20 pt) への変換
/// 段落の間隔などに使用
pub fn pt_to_twip(pt: f64) -> i32 {
    (pt * 20.0) as i32
}

/// 本文スタイルの styleId
pub const BODY_TEXT_STYLE_ID: &str = "13";

/// 見出し番号の numId (numbering.xml の num 要素 ID)
pub const HEADING_NUM_ID: usize = 2;
/// 見出し番号の abstractNumId
const HEADING_ABSTRACT_NUM_ID: usize = 8;

/// 箇条書きの numId (numbering.xml の num 要素 ID)
pub const BULLET_NUM_ID: usize = 3;
/// 箇条書きの abstractNumId
const BULLET_ABSTRACT_NUM_ID: usize = 9;
/// 箇条書きスタイルの styleId
pub const BULLET_STYLE_ID: &str = "BulletList";

const HEADING1_BEFORE_PT: f64 = 24.0;
const HEADING1_AFTER_PT: f64 = 12.0;
const HEADING2_BEFORE_PT: f64 = 18.0;
const HEADING2_AFTER_PT: f64 = 8.0;

/// sample.docx のスタイル定義を Docx に適用する
///
/// - docDefaults: minorHAnsi/minorEastAsia テーマ、sz=21 (10.5pt)
/// - Normal スタイル: id="a", jc=both
/// - Heading1-4: テーマフォント、サイズ、bold、keepNext、outlineLvl
/// - AbstractNumbering (id=8): 見出し番号 Level 0-3
/// - Numbering (id=2): abstractNumId=8
pub fn setup_document_styles(docx: Docx, config: &Config) -> Docx {
    // 採番オフセット（base_header 設定）。None = 全見出し無番号
    let base_header = config.numbering.base_header;
    let offset = base_header.offset();
    // 採番対象のレベルなら見出しスタイルに採番をバインドする
    let bind_numbering = |mut style: Style, markdown_level: u8| -> Style {
        if let Some(ilvl) = base_header.heading_ilvl(markdown_level) {
            style.paragraph_property = style
                .paragraph_property
                .numbering(NumberingId::new(HEADING_NUM_ID), IndentLevel::new(ilvl));
        }
        style
    };

    // --- docDefaults ---
    // テーマファイルを生成できないため、実フォント名を直接指定
    let default_fonts = RunFonts::new()
        .ascii(&config.fonts.body_en)
        .hi_ansi(&config.fonts.body_en)
        .east_asia(&config.fonts.body_ja)
        .cs(&config.fonts.body_en);

    let docx = docx
        .default_size(pt_to_half_point(config.sizes.body)) // 10.5pt = sz 21
        .default_fonts(default_fonts);

    // --- Normal スタイル ---
    // docx-rs は空の styleId="Normal" を自動生成するため、
    // 同じ ID で上書きする（後勝ち）。styleId="a" は使わない。
    let normal_fonts = RunFonts::new()
        .ascii(&config.fonts.body_en)
        .hi_ansi(&config.fonts.body_en)
        .east_asia(&config.fonts.body_ja)
        .cs(&config.fonts.body_en);

    let normal_style = Style::new("Normal", StyleType::Paragraph)
        .name("Normal")
        .fonts(normal_fonts)
        .size(pt_to_half_point(config.sizes.body))
        .align(AlignmentType::Both);

    // --- 見出し1 (id="1") ---
    // basedOn=Normal("a"), next=Normal("a")
    // keepNext, outlineLvl=0
    // テーマフォント: majorHAnsi / majorEastAsia / majorBidi
    // 14pt (sz=28), bold
    let heading1_fonts = RunFonts::new()
        .ascii(&config.fonts.heading_en)
        .hi_ansi(&config.fonts.heading_en)
        .east_asia(&config.fonts.heading_ja)
        .cs(&config.fonts.heading_en);

    let heading1_style = Style::new("1", StyleType::Paragraph)
        .name("heading 1")
        .based_on("Normal")
        .next("Normal")
        .size(pt_to_half_point(config.sizes.heading1)) // 14pt = sz 28
        .bold()
        .fonts(heading1_fonts)
        .line_spacing(
            LineSpacing::new()
                .before(pt_to_twip(HEADING1_BEFORE_PT) as u32)
                .after(pt_to_twip(HEADING1_AFTER_PT) as u32),
        )
        .outline_lvl(0);
    let heading1_style = bind_numbering(heading1_style, 1);

    // --- 見出し2 (id="2") ---
    // basedOn=見出し1("1"), next=Normal("a")
    // outlineLvl=1
    // 12pt (sz=24)
    // フォントは見出し1から継承
    let heading2_style = Style::new("2", StyleType::Paragraph)
        .name("heading 2")
        .based_on("1")
        .next("Normal")
        .size(pt_to_half_point(config.sizes.heading2)) // 12pt = sz 24
        .line_spacing(
            LineSpacing::new()
                .before(pt_to_twip(HEADING2_BEFORE_PT) as u32)
                .after(pt_to_twip(HEADING2_AFTER_PT) as u32),
        )
        .outline_lvl(1);
    let heading2_style = bind_numbering(heading2_style, 2);

    // --- 見出し3 (id="3") ---
    // basedOn=Normal("a"), next=Normal("a")
    // keepNext, outlineLvl=2
    // テーマフォント: majorHAnsi / majorEastAsia / majorBidi
    // 11pt (sz=22), bold
    let heading3_fonts = RunFonts::new()
        .ascii(&config.fonts.heading_en)
        .hi_ansi(&config.fonts.heading_en)
        .east_asia(&config.fonts.heading_ja)
        .cs(&config.fonts.heading_en);

    let heading3_style = Style::new("3", StyleType::Paragraph)
        .name("heading 3")
        .based_on("Normal")
        .next("Normal")
        .size(pt_to_half_point(config.sizes.heading3)) // 11pt = sz 22
        .bold()
        .fonts(heading3_fonts)
        .outline_lvl(2);
    let heading3_style = bind_numbering(heading3_style, 3);

    // --- 見出し4 (id="4") ---
    // basedOn=Normal("a"), next=Normal("a")
    // keepNext, outlineLvl=3
    // テーマフォント: majorEastAsia のみ
    // 11pt (sz=22), bold
    // indent: left=709, hanging=709
    let heading4_fonts = RunFonts::new().east_asia(&config.fonts.heading_ja);

    let heading4_style = Style::new("4", StyleType::Paragraph)
        .name("heading 4")
        .based_on("Normal")
        .next("Normal")
        .size(pt_to_half_point(config.sizes.heading4)) // 11pt = sz 22
        .bold()
        .fonts(heading4_fonts)
        .indent(
            Some(config.indent.heading4_left),
            Some(SpecialIndentType::Hanging(config.indent.heading4_hanging)),
            None,
            None,
        )
        .outline_lvl(3);
    let heading4_style = bind_numbering(heading4_style, 4);

    // --- 見出し5 (id="5") ---
    // basedOn=Normal, next=Normal
    // keepNext, outlineLvl=4
    // heading4 と同パターン（East Asia フォントのみ指定）
    let heading5_fonts = RunFonts::new().east_asia(&config.fonts.heading_ja);

    let heading5_style = Style::new("5", StyleType::Paragraph)
        .name("heading 5")
        .based_on("Normal")
        .next("Normal")
        .size(pt_to_half_point(config.sizes.heading5))
        .bold()
        .fonts(heading5_fonts)
        .outline_lvl(4);
    let heading5_style = bind_numbering(heading5_style, 5);

    // --- 見出し番号定義 (abstractNumId=8, numId=2) ---
    // フォーマットは採番の深さ（ilvl）に固定し、pStyle・インデントは
    // base_header オフセットに応じた markdown レベルへバインドする。
    // base_header = "none" のときは定義そのものを出力しない。
    const HEADING_LEVEL_FORMATS: [(&str, &str); 9] = [
        ("decimal", "%1."),
        ("decimal", "%1.%2."),
        ("decimal", "%1.%2.%3"),
        ("decimal", "\u{FF08}%4\u{FF09}"),
        ("decimalEnclosedCircle", "%5"),
        ("decimalEnclosedCircle", "%6"),
        ("decimal", "%7."),
        ("aiueoFullWidth", "(%8)"),
        ("decimalEnclosedCircle", "%9"),
    ];

    // markdown レベルに応じたインデント（H7 以降は markdown に存在しないが、
    // ilvl 6-8 の定義を維持するため従来の固定値を使う）
    let heading_indent = |markdown_level: u8| -> (i32, i32) {
        match markdown_level {
            1 => (config.indent.heading1_left, config.indent.heading1_hanging),
            2 => (config.indent.heading2_left, config.indent.heading2_hanging),
            3 => (config.indent.heading3_left, config.indent.heading3_hanging),
            4 => (config.indent.heading4_left, config.indent.heading4_hanging),
            5 => (config.indent.heading5_left, config.indent.heading5_hanging),
            6 => (config.indent.heading6_left, config.indent.heading6_hanging),
            7 => (2940, 420),
            8 => (3360, 420),
            _ => (3780, 420),
        }
    };

    let heading_numbering = offset.map(|offset| {
        let mut abstract_numbering = AbstractNumbering::new(HEADING_ABSTRACT_NUM_ID);
        for (ilvl, (format, text)) in HEADING_LEVEL_FORMATS.iter().enumerate() {
            let markdown_level = ilvl as u8 + 1 + offset;
            let (left, hanging) = heading_indent(markdown_level);
            let mut level = Level::new(
                ilvl,
                Start::new(1),
                NumberFormat::new(*format),
                LevelText::new(*text),
                LevelJc::new("left"),
            )
            .indent(
                Some(left),
                Some(SpecialIndentType::Hanging(hanging)),
                None,
                None,
            );
            // 対応する見出しスタイルが存在するレベルのみ pStyle を付ける
            if (1..=5).contains(&markdown_level) {
                level = level.paragraph_style(markdown_level.to_string());
            }
            abstract_numbering = abstract_numbering.add_level(level);
        }
        abstract_numbering.multi_level_type = Some("multilevel".to_string());
        abstract_numbering
    });

    // --- 本文ｰ見出しレベル1~3 (id="13") ---
    // sample.docx 準拠の本文スタイル（字下げ付き）
    // sample: leftChars=100/left=210, rightChars=100/right=100, firstLineChars=100/firstLine=100
    // docx-rs は rightChars, firstLineChars を出力できないため、
    // 絶対値を全角1文字幅 (210 twip = 2 × drawingGridHorizontalSpacing) に補正する
    let body_text_style = Style::new(BODY_TEXT_STYLE_ID, StyleType::Paragraph)
        .name("本文ｰ見出し")
        .based_on("Normal")
        .indent(
            Some(config.indent.body_left),
            Some(SpecialIndentType::FirstLine(config.indent.body_first_line)),
            Some(config.indent.body_right),
            Some(config.indent.body_left_chars),
        );

    // --- 箇条書き用 Numbering 定義 (abstractNumId=9, numId=3) ---
    let bullet_chars = [
        &config.bullet.level0,
        &config.bullet.level1,
        &config.bullet.level2,
    ];

    let mut bullet_abstract = AbstractNumbering::new(BULLET_ABSTRACT_NUM_ID);
    for (i, ch) in bullet_chars.iter().enumerate() {
        let left = (i as i32 + 1) * 360; // 360, 720, 1080
        let hanging = 360;
        bullet_abstract = bullet_abstract.add_level(
            Level::new(
                i,
                Start::new(1),
                NumberFormat::new("bullet"),
                LevelText::new(*ch),
                LevelJc::new("left"),
            )
            .indent(
                Some(left),
                Some(SpecialIndentType::Hanging(hanging)),
                None,
                None,
            ),
        );
    }

    let bullet_numbering = Numbering::new(BULLET_NUM_ID, BULLET_ABSTRACT_NUM_ID);

    let bullet_style = Style::new(BULLET_STYLE_ID, StyleType::Paragraph)
        .name("Bullet List")
        .based_on("Normal");

    let mut docx = docx
        .add_style(normal_style)
        .add_style(body_text_style)
        .add_style(heading1_style)
        .add_style(heading2_style)
        .add_style(heading3_style)
        .add_style(heading4_style)
        .add_style(heading5_style)
        .add_style(bullet_style);
    if let Some(abstract_numbering) = heading_numbering {
        docx = docx
            .add_abstract_numbering(abstract_numbering)
            .add_numbering(Numbering::new(HEADING_NUM_ID, HEADING_ABSTRACT_NUM_ID));
    }
    docx.add_abstract_numbering(bullet_abstract)
        .add_numbering(bullet_numbering)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::BaseHeader;

    /// 指定 ilvl の `<w:lvl>` ブロックを取り出す（最初に一致したもの = 見出し用定義）
    fn lvl_block(xml: &str, ilvl: usize) -> &str {
        let start_tag = format!(r#"<w:lvl w:ilvl="{}""#, ilvl);
        let start = xml.find(&start_tag).expect("lvl should exist");
        let end = xml[start..].find("</w:lvl>").expect("lvl should close") + start;
        &xml[start..end]
    }

    /// 指定 styleId の `<w:style>` ブロックを取り出す
    fn style_block<'a>(xml: &'a str, style_id: &str) -> &'a str {
        let start_tag = format!(r#"w:styleId="{}""#, style_id);
        let start = xml.find(&start_tag).expect("style should exist");
        let end = xml[start..].find("</w:style>").expect("style should close") + start;
        &xml[start..end]
    }

    #[test]
    fn base_header_h2_shifts_numbering_levels() {
        let mut config = Config::default();
        config.numbering.base_header = BaseHeader::H2;

        let built = setup_document_styles(Docx::new(), &config).build();
        let numberings = String::from_utf8(built.numberings).unwrap();
        let styles = String::from_utf8(built.styles).unwrap();

        // 見出し用定義（abstractNumId=8）のブロックに限定する
        // （docx-rs はデフォルトの abstractNum id=1 を常に出力するため）
        let start = numberings.find(r#"w:abstractNumId="8""#).unwrap();
        let end = numberings[start..].find("</w:abstractNum>").unwrap() + start;
        let heading_part = &numberings[start..end];

        // ilvl0 は見出し2にバインドされ、フォーマットは "%1."、インデントは heading2 設定
        let lvl0 = lvl_block(heading_part, 0);
        assert!(lvl0.contains(r#"<w:lvlText w:val="%1." />"#));
        assert!(lvl0.contains(r#"<w:pStyle w:val="2" />"#));
        assert!(lvl0.contains(r#"w:left="612""#));
        assert!(lvl0.contains(r#"w:hanging="612""#));

        // ilvl1〜3 は見出し3〜5にバインド
        let lvl1 = lvl_block(heading_part, 1);
        assert!(lvl1.contains(r#"<w:pStyle w:val="3" />"#));
        assert!(lvl1.contains(r#"w:left="783""#));
        let lvl2 = lvl_block(heading_part, 2);
        assert!(lvl2.contains(r#"<w:pStyle w:val="4" />"#));
        let lvl3 = lvl_block(heading_part, 3);
        assert!(lvl3.contains(r#"<w:pStyle w:val="5" />"#));

        // 見出し1は採番にバインドされない
        assert!(!heading_part.contains(r#"<w:pStyle w:val="1" />"#));

        // スタイル側: 見出し1に numPr なし、見出し2が numId=2 を持つ
        assert!(!style_block(&styles, "1").contains("<w:numPr"));
        assert!(style_block(&styles, "2").contains(r#"<w:numId w:val="2" />"#));
    }

    #[test]
    fn base_header_none_omits_heading_numbering() {
        let mut config = Config::default();
        config.numbering.base_header = BaseHeader::None;

        let built = setup_document_styles(Docx::new(), &config).build();
        let numberings = String::from_utf8(built.numberings).unwrap();
        let styles = String::from_utf8(built.styles).unwrap();

        // 見出し採番定義そのものを出力しない（バレット定義と docx-rs の
        // デフォルト定義 id=1 は残る）
        assert!(!numberings.contains(r#"w:abstractNumId="8""#));
        assert!(!numberings.contains(r#"<w:num w:numId="2""#));
        assert!(!numberings.contains("<w:pStyle"));

        // どの見出しスタイルにも numPr を付けない
        for id in ["1", "2", "3", "4", "5"] {
            assert!(!style_block(&styles, id).contains("<w:numPr"));
        }
    }

    #[test]
    fn heading_styles_include_spacing_for_levels_one_and_two() {
        let xml = String::from_utf8(
            setup_document_styles(Docx::new(), &Config::default())
                .build()
                .styles,
        )
        .unwrap();

        assert!(xml.contains(r#"<w:spacing w:before="480" w:after="240" />"#));
        assert!(xml.contains(r#"<w:spacing w:before="360" w:after="160" />"#));
    }

    #[test]
    fn heading_numberings_use_shallow_indent_for_levels_five_and_six() {
        let xml = String::from_utf8(
            setup_document_styles(Docx::new(), &Config::default())
                .build()
                .numberings,
        )
        .unwrap();

        assert_eq!(xml.matches(r#"w:left="709""#).count(), 3);
        assert_eq!(xml.matches(r#"w:hanging="709""#).count(), 3);
    }

    #[test]
    fn heading_numberings_follow_configured_indents_for_levels_one_to_six() {
        let mut config = Config::default();
        config.indent.heading1_left = 401;
        config.indent.heading1_hanging = 402;
        config.indent.heading2_left = 501;
        config.indent.heading2_hanging = 502;
        config.indent.heading3_left = 601;
        config.indent.heading3_hanging = 602;
        config.indent.heading4_left = 701;
        config.indent.heading4_hanging = 702;
        config.indent.heading5_left = 801;
        config.indent.heading5_hanging = 802;
        config.indent.heading6_left = 901;
        config.indent.heading6_hanging = 902;

        let xml = String::from_utf8(
            setup_document_styles(Docx::new(), &config)
                .build()
                .numberings,
        )
        .unwrap();

        assert!(xml.contains(r#"w:left="401""#));
        assert!(xml.contains(r#"w:hanging="402""#));
        assert!(xml.contains(r#"w:left="501""#));
        assert!(xml.contains(r#"w:hanging="502""#));
        assert!(xml.contains(r#"w:left="601""#));
        assert!(xml.contains(r#"w:hanging="602""#));
        assert!(xml.contains(r#"w:left="701""#));
        assert!(xml.contains(r#"w:hanging="702""#));
        assert!(xml.contains(r#"w:left="801""#));
        assert!(xml.contains(r#"w:hanging="802""#));
        assert!(xml.contains(r#"w:left="901""#));
        assert!(xml.contains(r#"w:hanging="902""#));
    }
}
