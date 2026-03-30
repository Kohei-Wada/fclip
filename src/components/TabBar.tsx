export type Tab = "all" | "pin";

export function TabBar({ activeTab }: { activeTab: Tab }) {
  return (
    <div className="tab-bar">
      <span className={`tab-item ${activeTab === "all" ? "active" : ""}`}>All</span>
      <span className={`tab-item ${activeTab === "pin" ? "active" : ""}`}>Pin</span>
    </div>
  );
}
