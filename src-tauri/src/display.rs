use crate::constants::{DISPLAY_CONTEXT_CHARS, MAX_DISPLAY_LEN};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DisplaySegment {
    pub text: String,
    pub highlighted: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DisplayInfo {
    pub segments: Vec<DisplaySegment>,
    pub truncated: bool,
}

/// Build display segments from content and match indices.
///
/// Handles truncation with smart windowing: if matches fall beyond the
/// visible window, the window slides to show matches in context.
/// All string operations use char iteration, so there is no encoding
/// mismatch when the result is consumed by a JS frontend.
pub fn build_display(content: &str, match_indices: &[usize]) -> DisplayInfo {
    let chars: Vec<char> = content.chars().map(sanitize_char).collect();
    let total = chars.len();

    if total == 0 {
        return DisplayInfo {
            segments: vec![DisplaySegment {
                text: String::new(),
                highlighted: false,
            }],
            truncated: false,
        };
    }

    let index_set: HashSet<usize> = match_indices.iter().copied().collect();

    let (start, end, prefix, suffix) = compute_window(&chars, match_indices);

    let mut segments: Vec<DisplaySegment> = Vec::new();

    if prefix {
        segments.push(DisplaySegment {
            text: "...".to_string(),
            highlighted: false,
        });
    }

    let mut current_text = String::new();
    let mut current_hl = index_set.contains(&start);

    for (i, &ch) in chars.iter().enumerate().skip(start).take(end - start) {
        let is_hl = index_set.contains(&i);
        if is_hl != current_hl {
            if !current_text.is_empty() {
                segments.push(DisplaySegment {
                    text: current_text,
                    highlighted: current_hl,
                });
            }
            current_text = String::new();
            current_hl = is_hl;
        }
        current_text.push(ch);
    }
    if !current_text.is_empty() {
        segments.push(DisplaySegment {
            text: current_text,
            highlighted: current_hl,
        });
    }

    if suffix {
        segments.push(DisplaySegment {
            text: "...".to_string(),
            highlighted: false,
        });
    }

    let truncated = prefix || suffix || total > end || start > 0;
    DisplayInfo {
        segments,
        truncated,
    }
}

fn sanitize_char(c: char) -> char {
    match c {
        '\n' | '\r' | '\t' => ' ',
        _ => c,
    }
}

/// Determine the display window (start..end) and whether to show ellipsis.
fn compute_window(chars: &[char], match_indices: &[usize]) -> (usize, usize, bool, bool) {
    let total = chars.len();
    let max_len = MAX_DISPLAY_LEN;

    if total <= max_len {
        return (0, total, false, false);
    }

    // No matches — show from start
    if match_indices.is_empty() {
        return (0, max_len, false, true);
    }

    let last = *match_indices.iter().max().unwrap();

    // All matches fit in the first window
    if last < max_len {
        return (0, max_len, false, true);
    }

    // Find the densest window of max_len chars that contains the most matches
    let mut sorted_indices: Vec<usize> = match_indices.to_vec();
    sorted_indices.sort_unstable();

    let best_start = find_densest_window(&sorted_indices, max_len, total);

    let prefix = best_start > 0;
    let end = (best_start + max_len).min(total);
    let suffix = end < total;

    (best_start, end, prefix, suffix)
}

/// Find the window start position that contains the most match indices.
/// Uses a sliding window over sorted match indices.
fn find_densest_window(sorted_indices: &[usize], window_size: usize, total: usize) -> usize {
    if sorted_indices.is_empty() {
        return 0;
    }

    let context = DISPLAY_CONTEXT_CHARS;
    let mut best_start = sorted_indices[0].saturating_sub(context);
    let mut best_count = 0;

    for (i, &idx) in sorted_indices.iter().enumerate() {
        let win_start = idx.saturating_sub(context);
        let win_end = win_start + window_size;

        let count = sorted_indices[i..]
            .iter()
            .take_while(|&&j| j < win_end)
            .count()
            + sorted_indices[..i]
                .iter()
                .rev()
                .take_while(|&&j| j >= win_start)
                .count();

        if count > best_count {
            best_count = count;
            best_start = win_start;
        }
    }

    // Clamp so we don't go past the end
    if best_start + window_size > total {
        best_start = total.saturating_sub(window_size);
    }

    best_start
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(text: &str, highlighted: bool) -> DisplaySegment {
        DisplaySegment {
            text: text.to_string(),
            highlighted,
        }
    }

    #[test]
    fn test_empty_content() {
        let info = build_display("", &[]);
        assert!(!info.truncated);
        assert_eq!(info.segments, vec![seg("", false)]);
    }

    #[test]
    fn test_short_no_matches() {
        let info = build_display("hello world", &[]);
        assert!(!info.truncated);
        assert_eq!(info.segments, vec![seg("hello world", false)]);
    }

    #[test]
    fn test_short_with_matches() {
        let info = build_display("hello world", &[0, 1, 2, 3, 4]);
        assert!(!info.truncated);
        assert_eq!(
            info.segments,
            vec![seg("hello", true), seg(" world", false)]
        );
    }

    #[test]
    fn test_long_matches_in_first_window() {
        let content = "a".repeat(200);
        let info = build_display(&content, &[0, 1, 2]);
        assert!(info.truncated);
        assert_eq!(info.segments.len(), 3); // highlighted + unhighlighted + "..."
        assert_eq!(info.segments.last().unwrap(), &seg("...", false));
        assert!(info.segments[0].highlighted);
        // Total displayed chars (excluding "...") should be MAX_DISPLAY_LEN
        let text_len: usize = info
            .segments
            .iter()
            .filter(|s| s.text != "...")
            .map(|s| s.text.chars().count())
            .sum();
        assert_eq!(text_len, MAX_DISPLAY_LEN);
    }

    #[test]
    fn test_long_matches_beyond_window() {
        let content = "a".repeat(50) + &"b".repeat(50) + "MATCH" + &"c".repeat(100);
        // "MATCH" starts at char index 100
        let info = build_display(&content, &[100, 101, 102, 103, 104]);
        assert!(info.truncated);
        // Should have prefix "..." since window slides
        assert_eq!(info.segments[0], seg("...", false));
        // Should contain the highlighted "MATCH" somewhere
        let has_match = info
            .segments
            .iter()
            .any(|s| s.highlighted && s.text.contains("MATCH"));
        assert!(
            has_match,
            "MATCH should be highlighted in segments: {:?}",
            info.segments
        );
    }

    #[test]
    fn test_emoji_content() {
        let info = build_display("Hello 🌍 world", &[8, 9, 10, 11, 12]);
        // "Hello 🌍 world" — chars: H(0) e(1) l(2) l(3) o(4) ' '(5) 🌍(6) ' '(7) w(8) o(9) r(10) l(11) d(12)
        assert!(!info.truncated);
        assert_eq!(
            info.segments,
            vec![seg("Hello 🌍 ", false), seg("world", true)]
        );
    }

    #[test]
    fn test_zwj_emoji_content() {
        // Man technologist: U+1F468 ZWJ U+1F4BB (3 chars: \u{1F468}, \u{200D}, \u{1F4BB})
        let info = build_display("abc \u{1F468}\u{200D}\u{1F4BB} xyz", &[0, 1, 2]);
        // chars: a(0) b(1) c(2) ' '(3) \u{1F468}(4) \u{200D}(5) \u{1F4BB}(6) ' '(7) x(8) y(9) z(10)
        assert!(!info.truncated);
        assert_eq!(
            info.segments,
            vec![
                seg("abc", true),
                seg(" \u{1F468}\u{200D}\u{1F4BB} xyz", false)
            ]
        );
    }

    #[test]
    fn test_emoji_only_content() {
        let info = build_display("\u{1F600}\u{1F680}\u{1F30D}", &[]);
        assert!(!info.truncated);
        assert_eq!(
            info.segments,
            vec![seg("\u{1F600}\u{1F680}\u{1F30D}", false)]
        );
    }

    #[test]
    fn test_newline_replaced() {
        let info = build_display("line1\nline2\tline3", &[]);
        assert!(!info.truncated);
        assert_eq!(info.segments, vec![seg("line1 line2 line3", false)]);
    }

    #[test]
    fn test_exactly_max_len() {
        let content = "x".repeat(MAX_DISPLAY_LEN);
        let info = build_display(&content, &[]);
        assert!(!info.truncated);
        assert_eq!(info.segments.len(), 1);
    }

    #[test]
    fn test_max_len_plus_one() {
        let content = "x".repeat(MAX_DISPLAY_LEN + 1);
        let info = build_display(&content, &[]);
        assert!(info.truncated);
        let text_len: usize = info
            .segments
            .iter()
            .filter(|s| s.text != "...")
            .map(|s| s.text.chars().count())
            .sum();
        assert_eq!(text_len, MAX_DISPLAY_LEN);
    }

    #[test]
    fn test_ellipsis_not_highlighted() {
        let content = "a".repeat(200);
        let info = build_display(&content, &[99]);
        for seg in &info.segments {
            if seg.text == "..." {
                assert!(!seg.highlighted, "ellipsis must not be highlighted");
            }
        }
    }

    #[test]
    fn test_scattered_matches_shows_densest() {
        // Matches at 5 and 150. 150 is isolated, 5 is alone too.
        // Should show from start since first match is at 5.
        let content = "a".repeat(200);
        let info = build_display(&content, &[5, 150]);
        // Window should include index 5
        assert!(info.segments.iter().any(|s| s.highlighted));
    }
}
