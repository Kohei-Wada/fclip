use crate::db::ClipboardEntry;
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
    pub match_indices: Vec<usize>,
}

impl From<ClipboardEntry> for SearchResult {
    fn from(e: ClipboardEntry) -> Self {
        Self {
            id: e.id,
            content: e.content,
            created_at: e.created_at,
            last_used_at: e.last_used_at,
            pinned: e.pinned,
            label: e.label,
            score: 0,
            match_indices: vec![],
        }
    }
}

impl SearchResult {
    fn with_match(entry: &ClipboardEntry, score: i64, indices: Vec<usize>) -> Self {
        Self {
            id: entry.id,
            content: entry.content.clone(),
            created_at: entry.created_at.clone(),
            last_used_at: entry.last_used_at.clone(),
            pinned: entry.pinned,
            label: entry.label.clone(),
            score,
            match_indices: indices,
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

    pub fn search(&self, entries: &[ClipboardEntry], query: &str, max_results: usize) -> Vec<SearchResult> {
        if query.is_empty() {
            return entries.iter().take(max_results).cloned().map(SearchResult::from).collect();
        }

        let mut results: Vec<SearchResult> = entries
            .iter()
            .filter_map(|e| {
                let search_text = if e.label.is_empty() {
                    e.content.clone()
                } else {
                    format!("{} {}", e.label, e.content)
                };
                let (score, indices) = self.matcher.fuzzy_indices(&search_text, query)?;
                Some(SearchResult::with_match(e, score, indices))
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
        assert!(result.match_indices.is_empty());
    }

    #[test]
    fn test_with_match() {
        let entry = make_entry(1, "test");
        let result = SearchResult::with_match(&entry, 100, vec![0, 1]);
        assert_eq!(result.score, 100);
        assert_eq!(result.match_indices, vec![0, 1]);
    }
}
