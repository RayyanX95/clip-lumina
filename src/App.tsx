import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";

function App() {
  const [text, setText] = useState("");

  const readClip = async () => {
    const clip = await invoke<string>("get_current_clip");
    setText(clip);
  };

  return (
    <div className="h-screen flex flex-col items-center justify-center bg-slate-900 text-white p-8">
      <h1 className="text-4xl font-bold mb-8 text-blue-400">ClipLumina</h1>
      <button 
        onClick={readClip}
        className="bg-blue-600 hover:bg-blue-500 px-6 py-3 rounded-lg font-semibold transition-all"
      >
        Check Clipboard
      </button>
      {text && (
        <div className="mt-8 p-4 bg-slate-800 rounded border border-slate-700 w-full max-w-md">
          <p className="text-sm text-slate-400 mb-2 font-mono">Current Content:</p>
          <p className="truncate">{text}</p>
        </div>
      )}
    </div>
  );
}

export default App;