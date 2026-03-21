export interface SearchResult {
  id: number;
  content: string;
  created_at: string;
  last_used_at: string;
  pinned: boolean;
  label: string;
  score: number;
  match_indices: number[];
}

export interface Keybindings {
  select: string;
  close: string;
  delete: string;
  next: string;
  prev: string;
}
