import { getCurrentWindow } from "@tauri-apps/api/window";
import { useClipboardHistory } from "./hooks/useClipboardHistory";

function App() {
  const { history, deleteClip, copyToClipboard } = useClipboardHistory();

  const handleCopy = async (content: string) => {
      await copyToClipboard(content);
      await getCurrentWindow().hide();
  };

  return (
    <div className="min-h-screen bg-brand-bg bg-[radial-gradient(circle_at_top_right,_var(--tw-gradient-stops))] from-brand-bg via-brand-bg-deep to-brand-bg flex flex-col p-4 text-brand-text font-sans h-screen overflow-hidden">
      {/* Background Decor */}
      <div className="absolute top-[-20%] right-[-20%] w-[50%] h-[50%] rounded-full bg-brand-primary-strong/5 blur-[100px] pointer-events-none" />
      <div className="absolute bottom-[-10%] left-[-10%] w-[40%] h-[40%] rounded-full bg-brand-secondary-strong/5 blur-[80px] pointer-events-none" />

      {/* Header */}
      <header className="flex-none mb-4 px-2 pt-2">
        <div className="flex items-center justify-between">
            <h1 className="text-xl font-bold text-white tracking-tight flex items-center gap-2">
              <span className="w-8 h-8 rounded-lg bg-gradient-to-tr from-brand-primary-strong to-brand-accent flex items-center justify-center shadow-lg shadow-brand-primary/20">
                <svg className="w-4 h-4 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                </svg>
              </span>
              ClipLumina
            </h1>
            <span className="text-xs font-medium px-2 py-1 rounded-full bg-white/5 border border-white/5 text-brand-text-muted">
              {history.length} items
            </span>
        </div>
      </header>

      {/* History List */}
      <main className="flex-1 overflow-y-auto custom-scrollbar pr-2 space-y-3 pb-4">
        {history.length === 0 ? (
          <div className="h-full flex flex-col items-center justify-center text-brand-text-dim text-center opacity-60">
             <svg className="w-12 h-12 mb-3 text-brand-text-muted/20" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z" />
             </svg>
             <p className="text-sm">Clipboard is empty</p>
             <p className="text-xs mt-1">Copy something to see it here</p>
          </div>
        ) : (
          history.map((item) => (
            <div 
              key={item.id}
              onClick={() => handleCopy(item.content)}
              className="group relative bg-white/5 hover:bg-white/10 border border-white/5 hover:border-brand-primary/30 rounded-xl p-3 transition-all duration-200 cursor-pointer"
            >
              <div className="flex items-start gap-3">
                <div className="mt-1 w-1 h-1 rounded-full bg-brand-primary/50 group-hover:bg-brand-primary transition-colors" />
                <div className="flex-1 min-w-0 pr-8">
                  <p className="text-sm text-brand-text-muted group-hover:text-white font-medium truncate mb-1 transition-colors">
                     {/* Heuristic for title: First line or first 40 chars */}
                     {item.content.split('\n')[0].substring(0, 40) || "Untitled Clip"}
                  </p>
                  <pre className="text-xs text-brand-text-dim font-mono bg-black/20 rounded p-2 overflow-hidden text-ellipsis whitespace-nowrap border border-white/5">
                    {item.content.trim()}
                  </pre>
                </div>
                
                {/* Actions */}
                <div className="opacity-0 group-hover:opacity-100 transition-opacity absolute right-2 top-2 flex flex-col gap-1 bg-brand-bg-deep/80 backdrop-blur-sm rounded-lg border border-white/5 p-0.5 shadow-xl transform translate-x-1 group-hover:translate-x-0 transition-transform">
                   <button 
                     onClick={(e) => deleteClip(item.id, e)}
                     className="p-1.5 rounded-md hover:bg-red-500/20 text-brand-text-dim hover:text-red-400 transition-colors"
                     title="Delete"
                   >
                     <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                       <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                     </svg>
                   </button>
                </div>
              </div>
            </div>
          ))
        )}
      </main>

       {/* Scanlines / Retro Effect Override (from CSS if any) or simple footer */}
      <style dangerouslySetInnerHTML={{ __html: `
        .custom-scrollbar::-webkit-scrollbar { width: 4px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.1); border-radius: 10px; }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.2); }
      `}} />
    </div>
  );
}

export default App;