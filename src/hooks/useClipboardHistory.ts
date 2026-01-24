import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export function useClipboardHistory() {
  const [history, setHistory] = useState<string[]>([]);

  useEffect(() => {
    // 1. Load initial clipboard content
    invoke<string>("get_current_clip").then((clip) => {
      if (clip) {
        setHistory([clip]);
      }
    });

    // 2. Listen for background updates
    const unlistenPromise = listen<string>("clipboard://change", (event) => {
      const newClip = event.payload;
      setHistory((prev) => {
        // Avoid duplicates at the top
        if (prev[0] === newClip) return prev;
        
        // Add new clip and limit to 50 items
        return [newClip, ...prev].slice(0, 50);
      });
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  return { history };
}
