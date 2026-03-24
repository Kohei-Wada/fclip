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

export interface Key {
  key: string;
  ctrl: boolean;
  shift: boolean;
  alt: boolean;
  meta: boolean;
}

export interface Keybindings {
  select: Key[];
  close: Key[];
  delete: Key[];
  next: Key[];
  prev: Key[];
  backspace: Key[];
  clear: Key[];
}
