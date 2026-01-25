import { getCurrentWindow } from "@tauri-apps/api/window";
import { useClipboardHistory } from "./hooks/useClipboardHistory";
import { Header } from "./components/layout/Header";
import { EmptyState } from "./components/features/EmptyState";
import { ClipItemCard } from "./components/features/ClipItemCard";

function App() {
  const { history, deleteClip, togglePin, clearAll, copyToClipboard } =
    useClipboardHistory();

  const handleCopy = async (content: string, kind?: string) => {
    await copyToClipboard(content, kind);
    await getCurrentWindow().hide();
  };

  const sortedHistory = [...history].sort((a, b) => {
    if (a.pinned === b.pinned) return 0;
    return a.pinned ? -1 : 1;
  });

  return (
    <div className="min-h-screen bg-brand-bg bg-[radial-gradient(circle_at_top_right,_var(--tw-gradient-stops))] from-brand-bg via-brand-bg-deep to-brand-bg flex flex-col p-4 text-brand-text font-sans h-screen overflow-hidden">
      {/* Background Decor */}
      <div className="absolute top-[-20%] right-[-20%] w-[50%] h-[50%] rounded-full bg-brand-primary-strong/5 blur-[100px] pointer-events-none" />
      <div className="absolute bottom-[-10%] left-[-10%] w-[40%] h-[40%] rounded-full bg-brand-secondary-strong/5 blur-[80px] pointer-events-none" />

      <Header count={history.length} onClear={clearAll} />

      {/* History List */}
      <main className="flex-1 overflow-y-auto custom-scrollbar pr-2 space-y-3 pb-4">
        {history.length === 0 ? (
          <EmptyState />
        ) : (
          sortedHistory.map((item) => (
            <ClipItemCard
              key={item.id}
              item={item}
              onCopy={handleCopy}
              onDelete={deleteClip}
              onPin={togglePin}
            />
          ))
        )}
      </main>

      {/* Scanlines / Retro Effect Override (from CSS if any) or simple footer */}
      <style
        dangerouslySetInnerHTML={{
          __html: `
        .custom-scrollbar::-webkit-scrollbar { width: 4px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 10px; }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.2); }
      `,
        }}
      />
    </div>
  );
}

export default App;
