import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

function App() {
  const [text, setText] = useState("");
  const [isHovering, setIsHovering] = useState(false);

  const readClip = async () => {
    try {
      const clip = await invoke<string>("get_current_clip");
      setText(clip);
    } catch (err) {
      console.error("Failed to read clipboard:", err);
    }
  };

  return (
    <div className="min-h-screen bg-[#0f172a] bg-[radial-gradient(circle_at_top_right,_var(--tw-gradient-stops))] from-slate-900 via-blue-950 to-slate-900 flex items-center justify-center p-6 text-slate-200 font-sans selection:bg-blue-500/30">
      {/* Decorative Orbs */}
      <div className="absolute top-[-10%] right-[-10%] w-[40%] h-[40%] rounded-full bg-blue-600/10 blur-[120px]" />
      <div className="absolute bottom-[-10%] left-[-10%] w-[35%] h-[35%] rounded-full bg-indigo-600/10 blur-[100px]" />

      <main className="relative w-full max-w-lg">
        <div className="backdrop-blur-2xl bg-white/5 border border-white/10 rounded-3xl p-8 shadow-[0_24px_48px_-12px_rgba(0,0,0,0.5)] overflow-hidden">
          {/* Header */}
          <header className="mb-10 text-center">
            <div className="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-gradient-to-tr from-blue-600 to-cyan-400 mb-6 shadow-lg shadow-blue-500/20">
              <svg 
                className="w-8 h-8 text-white" 
                fill="none" 
                stroke="currentColor" 
                viewBox="0 0 24 24" 
                xmlns="http://www.w3.org/2000/svg"
              >
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
              </svg>
            </div>
            <h1 className="text-4xl font-extrabold tracking-tight text-white mb-2 bg-clip-text text-transparent bg-gradient-to-b from-white to-slate-400">
              ClipLumina
            </h1>
            <p className="text-slate-400 text-sm font-medium uppercase tracking-[0.2em]">
              Next-Gen Clipboard Manager
            </p>
          </header>

          {/* Action Area */}
          <div className="space-y-6">
            <button 
              onClick={readClip}
              onMouseEnter={() => setIsHovering(true)}
              onMouseLeave={() => setIsHovering(false)}
              className="group relative w-full py-4 px-6 rounded-2xl bg-gradient-to-r from-blue-600 to-indigo-600 hover:from-blue-500 hover:to-indigo-500 text-white font-bold shadow-xl shadow-blue-900/40 transition-all duration-300 transform hover:scale-[1.02] active:scale-[0.98] overflow-hidden"
            >
              <div className="absolute inset-0 w-full h-full bg-gradient-to-r from-transparent via-white/10 to-transparent -translate-x-full group-hover:animate-[shimmer_2s_infinite]" />
              <span className="relative flex items-center justify-center gap-2">
                Check Clipboard
                <svg className={`w-5 h-5 transition-transform duration-300 ${isHovering ? 'translate-x-1' : ''}`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7l5 5m0 0l-5 5m5-5H6" />
                </svg>
              </span>
            </button>

            {/* Result Container */}
            <div className={`transition-all duration-500 ease-out ${text ? 'opacity-100 translate-y-0 scale-100' : 'opacity-0 translate-y-4 scale-95 pointer-events-none h-0 p-0 overflow-hidden'}`}>
              <div className="bg-black/20 border border-white/5 rounded-2xl p-5 group">
                <div className="flex items-center justify-between mb-3">
                  <span className="text-xs font-bold text-blue-400 uppercase tracking-widest">Current Content</span>
                  <div className="h-1.5 w-1.5 rounded-full bg-blue-500 animate-pulse" />
                </div>
                <div className="bg-slate-900/50 rounded-xl p-4 border border-white/5 max-h-48 overflow-y-auto custom-scrollbar">
                  <p className="text-slate-300 font-mono text-sm break-all leading-relaxed">
                    {text}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Footer info */}
        <p className="mt-8 text-center text-slate-500 text-xs font-medium">
          SECURE & ENCRYPTED CLIPBOARD ACCESS
        </p>
      </main>

      <style dangerouslySetInnerHTML={{ __html: `
        @keyframes shimmer {
          100% { transform: translateX(100%); }
        }
        .custom-scrollbar::-webkit-scrollbar {
          width: 4px;
        }
        .custom-scrollbar::-webkit-scrollbar-track {
          background: transparent;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb {
          background: rgba(255, 255, 255, 0.1);
          border-radius: 10px;
        }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover {
          background: rgba(255, 255, 255, 0.2);
        }
      `}} />
    </div>
  );
}

export default App;