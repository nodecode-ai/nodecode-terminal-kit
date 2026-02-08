use unicode_width::UnicodeWidthChar;

pub fn wrapped_row_ranges(text: &str, content_width: u16) -> Vec<(usize, usize)> {
    let width = content_width.max(1) as usize;
    let mut rows = Vec::new();
    let mut start = 0usize;
    let mut width_cols = 0usize;
    let mut last_break = 0usize;
    let len = text.len();
    let mut idx = 0usize;
    while idx < len {
        let Some(ch) = text[idx..].chars().next() else {
            break;
        };
        let ch_len = ch.len_utf8();
        if ch == '\n' {
            rows.push((start, idx));
            idx += ch_len;
            start = idx;
            width_cols = 0;
            last_break = idx;
            continue;
        }
        let ch_w = UnicodeWidthChar::width(ch).unwrap_or(0).max(1);
        if width_cols > 0 && width_cols + ch_w > width {
            let wrap_pos = if last_break > start { last_break } else { idx };
            rows.push((start, wrap_pos));
            start = wrap_pos;
            width_cols = 0;
            continue;
        }
        width_cols += ch_w;
        if ch.is_whitespace() || ch == '@' || ch == '/' {
            last_break = idx + ch_len;
        }
        idx += ch_len;
    }
    rows.push((start, len));
    rows
}

pub fn visual_column(slice: &str) -> usize {
    slice
        .chars()
        .map(|c| UnicodeWidthChar::width(c).unwrap_or(0).max(1))
        .sum()
}

pub fn find_visual_row(ranges: &[(usize, usize)], cursor: usize) -> usize {
    if ranges.is_empty() {
        return 0;
    }
    let mut current_idx = 0usize;
    for (idx, (start, _)) in ranges.iter().enumerate() {
        if cursor < *start {
            break;
        }
        current_idx = idx;
    }
    current_idx.min(ranges.len().saturating_sub(1))
}

pub fn cursor_to_byte_offset(lines: &[String], cursor: (usize, usize)) -> usize {
    let (row, col_chars) = cursor;
    let mut offset = 0usize;
    for (idx, line) in lines.iter().enumerate() {
        if idx < row {
            offset += line.len() + 1;
        } else {
            for (chars_seen, (byte_idx, _ch)) in line.char_indices().enumerate() {
                if chars_seen == col_chars {
                    offset += byte_idx;
                    return offset;
                }
            }
            offset += line.len();
            return offset;
        }
    }
    offset
}

pub fn byte_offset_to_cursor(lines: &[String], offset: usize) -> (usize, usize) {
    let mut remaining = offset;
    for (idx, line) in lines.iter().enumerate() {
        if remaining <= line.len() {
            let mut chars = 0usize;
            let mut consumed = 0usize;
            for ch in line.chars() {
                let len = ch.len_utf8();
                if consumed + len > remaining {
                    break;
                }
                consumed += len;
                if consumed > remaining {
                    break;
                }
                chars += 1;
            }
            return (idx, chars);
        }
        remaining = remaining.saturating_sub(line.len() + 1);
    }
    let last_row = lines.len().saturating_sub(1);
    (
        last_row,
        lines.get(last_row).map(|s| s.chars().count()).unwrap_or(0),
    )
}
