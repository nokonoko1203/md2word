use crate::ir::Inline;

/// 見出し番号管理
/// 採番レベル 1 → X, 2 → X.X, 3 → X.X.X, 4 → (N), 5 → ①
/// offset により markdown レベルと採番レベルをずらす（base_header 設定）。
/// offset=None は全見出し無番号。
pub struct HeadingManager {
    counters: [u32; 5], // 採番レベル 1..5
    offset: Option<u8>,
}

impl HeadingManager {
    pub fn new(offset: Option<u8>) -> Self {
        Self {
            counters: [0; 5],
            offset,
        }
    }

    /// markdown レベルを採番レベルに変換する。無番号レベルは None。
    fn effective_level(&self, level: u8) -> Option<u8> {
        let eff = level.checked_sub(self.offset?)?;
        (eff >= 1).then_some(eff)
    }

    /// 見出しレベルに応じて番号を更新し、フォーマットされた番号文字列を返す。
    /// 見出しテキストに既に番号が含まれている場合はそれを尊重する。
    /// 無番号レベルでは空文字列を返し、カウンタを変更しない。
    pub fn next_heading(&mut self, level: u8, content: &[Inline]) -> String {
        let Some(level) = self.effective_level(level) else {
            return String::new();
        };
        let plain_text: String = content.iter().map(|i| i.to_plain_text()).collect();

        // 既存の番号を検出する
        if let Some(existing) = self.detect_existing_number(level, &plain_text) {
            self.sync_counters(level, &existing);
            return existing;
        }

        // 自動採番
        self.increment(level);
        self.format_number(level)
    }

    /// テキストから見出し番号部分を除去し、タイトルのみを返す。
    /// `detect_existing_number` を内部で使い、番号フォーマット判定ロジックを一元化する。
    /// 無番号レベルではテキストをそのまま返す。
    pub fn strip_number(&self, level: u8, text: &str) -> String {
        let trimmed = text.trim();
        let Some(level) = self.effective_level(level) else {
            return trimmed.to_string();
        };
        if self.detect_existing_number(level, trimmed).is_some() {
            match level {
                1 => {
                    // "8 タイトル" → "タイトル"
                    if let Some(rest) = trimmed.split_once(' ') {
                        return rest.1.to_string();
                    }
                }
                2 => {
                    // "8.1 タイトル" → "タイトル"
                    if let Some(rest) = trimmed.split_once(' ') {
                        return rest.1.to_string();
                    }
                }
                3 => {
                    // "8.1.1 タイトル" → "タイトル"
                    if let Some(rest) = trimmed.split_once(' ') {
                        return rest.1.to_string();
                    }
                }
                4 => {
                    // "(1) タイトル" → "タイトル"
                    if let Some(end) = trimmed.find(')') {
                        return trimmed[end + 1..].trim_start().to_string();
                    }
                }
                5 => {
                    // "① タイトル" → "タイトル"
                    let mut chars = trimmed.chars();
                    chars.next(); // 丸数字をスキップ
                    return chars.as_str().trim_start().to_string();
                }
                _ => {}
            }
        }
        trimmed.to_string()
    }

    /// 現在の章番号（base レベルのカウンタ値）を返す。
    pub fn current_chapter_number(&self) -> u32 {
        self.counters[0]
    }

    fn increment(&mut self, level: u8) {
        let idx = (level as usize).saturating_sub(1).min(4);
        self.counters[idx] += 1;
        // 下位レベルをリセット
        for i in (idx + 1)..5 {
            self.counters[i] = 0;
        }
    }

    fn format_number(&self, level: u8) -> String {
        match level {
            1 => format!("{}", self.counters[0]),
            2 => format!("{}.{}", self.counters[0], self.counters[1]),
            3 => format!(
                "{}.{}.{}",
                self.counters[0], self.counters[1], self.counters[2]
            ),
            4 => {
                let n = self.counters[3];
                format!("({})", n)
            }
            5 => {
                let n = self.counters[4];
                num_to_circled(n)
            }
            _ => String::new(),
        }
    }

    fn detect_existing_number(&self, level: u8, text: &str) -> Option<String> {
        let trimmed = text.trim();
        match level {
            1 => {
                // "8 タイトル" → "8"
                if let Some(num_str) = trimmed.split_whitespace().next()
                    && num_str.parse::<u32>().is_ok()
                {
                    return Some(num_str.to_string());
                }
                None
            }
            2 => {
                // "8.1 タイトル" → "8.1"
                if let Some(num_str) = trimmed.split_whitespace().next() {
                    let parts: Vec<&str> = num_str.split('.').collect();
                    if parts.len() == 2
                        && parts[0].parse::<u32>().is_ok()
                        && parts[1].parse::<u32>().is_ok()
                    {
                        return Some(num_str.to_string());
                    }
                }
                None
            }
            3 => {
                // "8.1.1 タイトル" → "8.1.1"
                if let Some(num_str) = trimmed.split_whitespace().next() {
                    let parts: Vec<&str> = num_str.split('.').collect();
                    if parts.len() == 3 && parts.iter().all(|p| p.parse::<u32>().is_ok()) {
                        return Some(num_str.to_string());
                    }
                }
                None
            }
            4 => {
                // "(1) タイトル" → "(1)"
                if trimmed.starts_with('(')
                    && let Some(end) = trimmed.find(')')
                {
                    let inner = &trimmed[1..end];
                    if inner.parse::<u32>().is_ok() {
                        return Some(format!("({})", inner));
                    }
                }
                None
            }
            5 => {
                // "① タイトル" → "①"
                let first_char = trimmed.chars().next()?;
                if is_circled_number(first_char) {
                    return Some(first_char.to_string());
                }
                None
            }
            _ => None,
        }
    }

    fn sync_counters(&mut self, level: u8, number: &str) {
        match level {
            1 => {
                if let Ok(n) = number.parse::<u32>() {
                    self.counters[0] = n;
                    for i in 1..5 {
                        self.counters[i] = 0;
                    }
                }
            }
            2 => {
                let parts: Vec<&str> = number.split('.').collect();
                if parts.len() == 2
                    && let (Ok(a), Ok(b)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>())
                {
                    self.counters[0] = a;
                    self.counters[1] = b;
                    for i in 2..5 {
                        self.counters[i] = 0;
                    }
                }
            }
            3 => {
                let parts: Vec<&str> = number.split('.').collect();
                if parts.len() == 3
                    && let (Ok(a), Ok(b), Ok(c)) = (
                        parts[0].parse::<u32>(),
                        parts[1].parse::<u32>(),
                        parts[2].parse::<u32>(),
                    )
                {
                    self.counters[0] = a;
                    self.counters[1] = b;
                    self.counters[2] = c;
                    for i in 3..5 {
                        self.counters[i] = 0;
                    }
                }
            }
            4 => {
                let inner = number.trim_start_matches('(').trim_end_matches(')');
                if let Ok(n) = inner.parse::<u32>() {
                    self.counters[3] = n;
                    self.counters[4] = 0;
                }
            }
            5 => {
                if let Some(n) = circled_to_num(number.chars().next().unwrap_or('①')) {
                    self.counters[4] = n;
                }
            }
            _ => {}
        }
    }
}

fn num_to_circled(n: u32) -> String {
    const CIRCLED: &[char] = &[
        '①', '②', '③', '④', '⑤', '⑥', '⑦', '⑧', '⑨', '⑩', '⑪', '⑫', '⑬', '⑭', '⑮', '⑯', '⑰', '⑱',
        '⑲', '⑳',
    ];
    if (1..=20).contains(&n) {
        CIRCLED[(n - 1) as usize].to_string()
    } else {
        format!("({})", n)
    }
}

fn is_circled_number(c: char) -> bool {
    ('\u{2460}'..='\u{2473}').contains(&c) // ①-⑳
}

fn circled_to_num(c: char) -> Option<u32> {
    if is_circled_number(c) {
        Some((c as u32) - 0x245F)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(s: &str) -> Vec<Inline> {
        vec![Inline::Text(s.to_string())]
    }

    #[test]
    fn numbers_from_h1_with_zero_offset() {
        let mut mgr = HeadingManager::new(Some(0));
        assert_eq!(mgr.next_heading(1, &text("はじめに")), "1");
        assert_eq!(mgr.next_heading(2, &text("背景")), "1.1");
        assert_eq!(mgr.current_chapter_number(), 1);
    }

    #[test]
    fn shifts_numbering_down_with_offset_one() {
        let mut mgr = HeadingManager::new(Some(1));
        assert_eq!(mgr.next_heading(1, &text("タイトル")), "");
        assert_eq!(mgr.next_heading(2, &text("はじめに")), "1");
        assert_eq!(mgr.next_heading(3, &text("背景")), "1.1");
        assert_eq!(mgr.next_heading(4, &text("詳細")), "1.1.1");
        assert_eq!(mgr.next_heading(5, &text("補足")), "(1)");
        assert_eq!(mgr.next_heading(6, &text("メモ")), "①");
        assert_eq!(mgr.current_chapter_number(), 1);
    }

    #[test]
    fn syncs_existing_number_at_shifted_level() {
        let mut mgr = HeadingManager::new(Some(1));
        assert_eq!(mgr.next_heading(2, &text("3 仕様")), "3");
        assert_eq!(mgr.current_chapter_number(), 3);
        assert_eq!(mgr.strip_number(2, "3 仕様"), "仕様");
        assert_eq!(mgr.next_heading(3, &text("詳細")), "3.1");
    }

    #[test]
    fn unnumbered_levels_keep_text_and_counters_untouched() {
        let mut mgr = HeadingManager::new(Some(1));
        assert_eq!(mgr.next_heading(1, &text("1 タイトル")), "");
        assert_eq!(mgr.strip_number(1, "1 タイトル"), "1 タイトル");
        assert_eq!(mgr.current_chapter_number(), 0);

        let mut none_mgr = HeadingManager::new(None);
        assert_eq!(none_mgr.next_heading(1, &text("はじめに")), "");
        assert_eq!(none_mgr.next_heading(2, &text("1.1 節")), "");
        assert_eq!(none_mgr.strip_number(2, "1.1 節"), "1.1 節");
        assert_eq!(none_mgr.current_chapter_number(), 0);
    }
}
