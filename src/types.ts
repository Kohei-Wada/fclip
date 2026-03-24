export interface DisplaySegment {
  text: string;
  highlighted: boolean;
}

export interface DisplayInfo {
  segments: DisplaySegment[];
  truncated: boolean;
}

export interface SearchResult {
  id: number;
  content: string;
  created_at: string;
  last_used_at: string;
  pinned: boolean;
  label: string;
  score: number;
  display: DisplayInfo;
}

export interface Keybindings {
  select: string;
  close: string;
  delete: string;
  next: string;
  prev: string;
  backspace: string;
  clear: string;
}
