use crate::db::ClipboardEntry;
use crate::display::{build_display, DisplayInfo};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub id: i64,
    pub content: String,
    pub created_at: String,
    pub last_used_at: String,
    pub pinned: bool,
    pub label: String,
    pub score: i64,
    pub display: DisplayInfo,
}

impl From<ClipboardEntry> for SearchResult {
    fn from(e: ClipboardEntry) -> Self {
        let display = build_display(&e.content, &[]);
        Self {
            id: e.id,
            content: e.content,
            created_at: e.created_at,
            last_used_at: e.last_used_at,
            pinned: e.pinned,
            label: e.label,
            score: 0,
            display,
        }
    }
}

impl SearchResult {
    fn with_match(entry: &ClipboardEntry, score: i64, indices: Vec<usize>) -> Self {
        let display = build_display(&entry.content, &indices);
        Self {
            id: entry.id,
            content: entry.content.clone(),
            created_at: entry.created_at.clone(),
            last_used_at: entry.last_used_at.clone(),
            pinned: entry.pinned,
            label: entry.label.clone(),
            score,
            display,
        }
    }
}

pub struct FuzzySearcher {
    matcher: SkimMatcherV2,
}

impl FuzzySearcher {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn search(
        &self,
        entries: &[ClipboardEntry],
        query: &str,
        max_results: usize,
    ) -> Vec<SearchResult> {
        if query.is_empty() {
            return entries
                .iter()
                .take(max_results)
                .cloned()
                .map(SearchResult::from)
                .collect();
        }

        let mut results: Vec<SearchResult> = entries
            .iter()
            .filter_map(|e| {
                let has_label = !e.label.is_empty();
                let search_text = if has_label {
                    format!("{} {}", e.label, e.content)
                } else {
                    e.content.clone()
                };
                let (score, indices) = self.matcher.fuzzy_indices(&search_text, query)?;
                let adjusted_indices = if has_label {
                    let offset = e.label.chars().count() + 1;
                    indices
                        .into_iter()
                        .filter(|&i| i >= offset)
                        .map(|i| i - offset)
                        .collect()
                } else {
                    indices
                };
                Some(SearchResult::with_match(e, score, adjusted_indices))
            })
            .collect();

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results.truncate(max_results);
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::DisplaySegment;

    fn make_entry(id: i64, content: &str) -> ClipboardEntry {
        ClipboardEntry {
            id,
            content: content.to_string(),
            created_at: String::new(),
            last_used_at: String::new(),
            pinned: false,
            label: String::new(),
        }
    }

    fn make_labeled_entry(id: i64, content: &str, label: &str) -> ClipboardEntry {
        ClipboardEntry {
            id,
            content: content.to_string(),
            created_at: String::new(),
            last_used_at: String::new(),
            pinned: true,
            label: label.to_string(),
        }
    }

    fn seg(text: &str, highlighted: bool) -> DisplaySegment {
        DisplaySegment {
            text: text.to_string(),
            highlighted,
        }
    }

    #[test]
    fn test_empty_query_returns_all() {
        let searcher = FuzzySearcher::new();
        let entries = vec![make_entry(1, "hello"), make_entry(2, "world")];
        let results = searcher.search(&entries, "", 50);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_fuzzy_match() {
        let searcher = FuzzySearcher::new();
        let entries = vec![
            make_entry(1, "git commit -m 'fix'"),
            make_entry(2, "docker compose up"),
        ];
        let results = searcher.search(&entries, "git", 50);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 1);
    }

    #[test]
    fn test_from_clipboard_entry() {
        let entry = make_entry(1, "hello");
        let result = SearchResult::from(entry);
        assert_eq!(result.id, 1);
        assert_eq!(result.content, "hello");
        assert_eq!(result.score, 0);
        assert!(!result.display.truncated);
        assert_eq!(result.display.segments, vec![seg("hello", false)]);
    }

    #[test]
    fn test_with_match() {
        let entry = make_entry(1, "test");
        let result = SearchResult::with_match(&entry, 100, vec![0, 1]);
        assert_eq!(result.score, 100);
        assert_eq!(
            result.display.segments,
            vec![seg("te", true), seg("st", false)]
        );
    }

    #[test]
    fn test_labeled_entry_highlight() {
        let searcher = FuzzySearcher::new();
        let entries = vec![make_labeled_entry(1, "hello world", "mypin")];
        let results = searcher.search(&entries, "hello", 50);
        assert_eq!(results.len(), 1);
        // "hello" should be highlighted in the display
        let has_hello_hl = results[0]
            .display
            .segments
            .iter()
            .any(|s| s.highlighted && s.text == "hello");
        assert!(
            has_hello_hl,
            "expected 'hello' highlighted: {:?}",
            results[0].display.segments
        );
    }

    #[test]
    fn test_multibyte_label_highlight() {
        let searcher = FuzzySearcher::new();
        let entries = vec![make_labeled_entry(1, "test data", "\u{1F4CC}")];
        let results = searcher.search(&entries, "test", 50);
        assert_eq!(results.len(), 1);
        let has_test_hl = results[0]
            .display
            .segments
            .iter()
            .any(|s| s.highlighted && s.text == "test");
        assert!(
            has_test_hl,
            "expected 'test' highlighted: {:?}",
            results[0].display.segments
        );
    }

    #[test]
    fn test_match_in_label_only() {
        let searcher = FuzzySearcher::new();
        let entries = vec![make_labeled_entry(1, "xyz", "important")];
        let results = searcher.search(&entries, "imp", 50);
        assert_eq!(results.len(), 1);
        // No highlights since matches are in label, not content
        assert!(results[0].display.segments.iter().all(|s| !s.highlighted));
    }
}
