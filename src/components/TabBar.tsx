export type Tab = "recent" | "pinned";

export function TabBar({ activeTab }: { activeTab: Tab }) {
  return (
    <div className="tab-bar">
      <span className={`tab-item ${activeTab === "recent" ? "active" : ""}`}>Recent</span>
      <span className={`tab-item ${activeTab === "pinned" ? "active" : ""}`}>Pinned</span>
    </div>
  );
}
