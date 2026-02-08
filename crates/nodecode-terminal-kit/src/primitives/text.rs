use unicode_width::UnicodeWidthStr;

/// Count visual rows needed to render the input text given content width.
/// Preserves a trailing blank line if text ends with a newline.
#[inline]
pub fn wrapped_row_count(text: &str, content_w: u16) -> u16 {
    let mut lines: Vec<&str> = text.lines().collect();
    if text.ends_with('\n') || text.ends_with("\r\n") {
        lines.push("");
    }
    let w = content_w.max(1) as usize;
    let total_rows: usize = lines
        .iter()
        .map(|l| {
            let lw = UnicodeWidthStr::width(*l);
            if lw == 0 {
                1
            } else {
                lw.div_ceil(w)
            }
        })
        .sum();
    total_rows as u16
}
