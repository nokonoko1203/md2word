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

/// 表題スタイルの styleId（Word 標準「表題」= "Title"）
pub const TITLE_STYLE_ID: &str = "Title";

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

    let mut heading1_style = Style::new("1", StyleType::Paragraph)
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
    // sample.docx 準拠: numId のみ（ilvl は省略 → デフォルト 0）
    heading1_style.paragraph_property = heading1_style
        .paragraph_property
        .numbering_property(NumberingProperty::new().id(NumberingId::new(HEADING_NUM_ID)));

    // --- 見出し2 (id="2") ---
    // basedOn=見出し1("1"), next=Normal("a")
    // outlineLvl=1
    // 12pt (sz=24)
    // フォントは見出し1から継承
    let mut heading2_style = Style::new("2", StyleType::Paragraph)
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
    // sample.docx 準拠: ilvl のみ（numId は basedOn=heading1 から継承）
    {
        let mut np = NumberingProperty::new();
        np.level = Some(IndentLevel::new(1));
        heading2_style.paragraph_property =
            heading2_style.paragraph_property.numbering_property(np);
    }

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

    let mut heading3_style = Style::new("3", StyleType::Paragraph)
        .name("heading 3")
        .based_on("Normal")
        .next("Normal")
        .size(pt_to_half_point(config.sizes.heading3)) // 11pt = sz 22
        .bold()
        .fonts(heading3_fonts)
        .outline_lvl(2);
    heading3_style.paragraph_property = heading3_style
        .paragraph_property
        .numbering(NumberingId::new(HEADING_NUM_ID), IndentLevel::new(2));

    // --- 見出し4 (id="4") ---
    // basedOn=Normal("a"), next=Normal("a")
    // keepNext, outlineLvl=3
    // テーマフォント: majorEastAsia のみ
    // 11pt (sz=22), bold
    // indent: left=709, hanging=709
    let heading4_fonts = RunFonts::new().east_asia(&config.fonts.heading_ja);

    let mut heading4_style = Style::new("4", StyleType::Paragraph)
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
    heading4_style.paragraph_property = heading4_style
        .paragraph_property
        .numbering(NumberingId::new(HEADING_NUM_ID), IndentLevel::new(3));

    // --- 見出し5 (id="5") ---
    // basedOn=Normal, next=Normal
    // keepNext, outlineLvl=4
    // heading4 と同パターン（East Asia フォントのみ指定）
    let heading5_fonts = RunFonts::new().east_asia(&config.fonts.heading_ja);

    let mut heading5_style = Style::new("5", StyleType::Paragraph)
        .name("heading 5")
        .based_on("Normal")
        .next("Normal")
        .size(pt_to_half_point(config.sizes.heading5))
        .bold()
        .fonts(heading5_fonts)
        .outline_lvl(4);
    heading5_style.paragraph_property = heading5_style
        .paragraph_property
        .numbering(NumberingId::new(HEADING_NUM_ID), IndentLevel::new(4));

    // --- 表題 (id="Title") ---
    // heading_shift = true 時に # に対応するスタイル
    // Word 標準「表題」に合わせ: 中央揃え、大フォント、番号なし
    let title_fonts = RunFonts::new()
        .ascii(&config.fonts.heading_en)
        .hi_ansi(&config.fonts.heading_en)
        .east_asia(&config.fonts.heading_ja)
        .cs(&config.fonts.heading_en);

    let title_style = Style::new(TITLE_STYLE_ID, StyleType::Paragraph)
        .name("Title")
        .based_on("Normal")
        .next("Normal")
        .size(pt_to_half_point(config.heading.title_size))
        .bold()
        .fonts(title_fonts)
        .align(AlignmentType::Center);

    // --- 見出し番号定義 (abstractNumId=8, numId=2) ---
    let mut abstract_numbering = AbstractNumbering::new(HEADING_ABSTRACT_NUM_ID)
        // Level 0: decimal, "%1.", indent left=420, hanging=420, pStyle="1"
        .add_level(
            Level::new(
                0,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%1."),
                LevelJc::new("left"),
            )
            .paragraph_style("1")
            .indent(
                Some(config.indent.heading1_left),
                Some(SpecialIndentType::Hanging(config.indent.heading1_hanging)),
                None,
                None,
            ),
        )
        // Level 1: decimal, "%1.%2.", indent left=612, hanging=612, pStyle="2"
        .add_level(
            Level::new(
                1,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%1.%2."),
                LevelJc::new("left"),
            )
            .paragraph_style("2")
            .indent(
                Some(config.indent.heading2_left),
                Some(SpecialIndentType::Hanging(config.indent.heading2_hanging)),
                None,
                None,
            ),
        )
        // Level 2: decimal, "%1.%2.%3", indent left=783, hanging=783, pStyle="3"
        .add_level(
            Level::new(
                2,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%1.%2.%3"),
                LevelJc::new("left"),
            )
            .paragraph_style("3")
            .indent(
                Some(config.indent.heading3_left),
                Some(SpecialIndentType::Hanging(config.indent.heading3_hanging)),
                None,
                None,
            ),
        )
        // Level 3: decimal, "（%4）", indent left=709, hanging=709, pStyle="4"
        // Style 4 のインデント定義に合わせる
        .add_level(
            Level::new(
                3,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("\u{FF08}%4\u{FF09}"),
                LevelJc::new("left"),
            )
            .paragraph_style("4")
            .indent(
                Some(config.indent.heading4_left),
                Some(SpecialIndentType::Hanging(config.indent.heading4_hanging)),
                None,
                None,
            ),
        )
        // Level 4: decimalEnclosedCircle（丸数字 ① ② ...）, pStyle="5"
        .add_level(
            Level::new(
                4,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%5"),
                LevelJc::new("left"),
            )
            .paragraph_style("5")
            .indent(
                Some(config.indent.heading5_left),
                Some(SpecialIndentType::Hanging(config.indent.heading5_hanging)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                5,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%6"),
                LevelJc::new("left"),
            )
            .indent(
                Some(config.indent.heading6_left),
                Some(SpecialIndentType::Hanging(config.indent.heading6_hanging)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                6,
                Start::new(1),
                NumberFormat::new("decimal"),
                LevelText::new("%7."),
                LevelJc::new("left"),
            )
            .indent(
                Some(2940),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                7,
                Start::new(1),
                NumberFormat::new("aiueoFullWidth"),
                LevelText::new("(%8)"),
                LevelJc::new("left"),
            )
            .indent(
                Some(3360),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        )
        .add_level(
            Level::new(
                8,
                Start::new(1),
                NumberFormat::new("decimalEnclosedCircle"),
                LevelText::new("%9"),
                LevelJc::new("left"),
            )
            .indent(
                Some(3780),
                Some(SpecialIndentType::Hanging(420)),
                None,
                None,
            ),
        );
    abstract_numbering.multi_level_type = Some("multilevel".to_string());

    let numbering = Numbering::new(HEADING_NUM_ID, HEADING_ABSTRACT_NUM_ID);

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

    docx.add_style(normal_style)
        .add_style(title_style)
        .add_style(body_text_style)
        .add_style(heading1_style)
        .add_style(heading2_style)
        .add_style(heading3_style)
        .add_style(heading4_style)
        .add_style(heading5_style)
        .add_style(bullet_style)
        .add_abstract_numbering(abstract_numbering)
        .add_abstract_numbering(bullet_abstract)
        .add_numbering(numbering)
        .add_numbering(bullet_numbering)
}

#[cfg(test)]
mod tests {
    use super::*;

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
