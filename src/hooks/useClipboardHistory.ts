import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export interface ClipItem {
  id: string;
  content: string;
  timestamp: number;
  pinned: boolean;
  kind?: string;
}

export function useClipboardHistory() {
  const [history, setHistory] = useState<ClipItem[]>([]);

  useEffect(() => {
    // 1. Fetch persistent history
    invoke<ClipItem[]>("get_history")
      .then((items) => setHistory(items))
      .catch((err) => console.error("Failed to load history:", err));

    // 2. Listen for background updates (full list sync)
    const unlistenPromise = listen<ClipItem[]>("clipboard://update", (event) => {
      setHistory(event.payload);
    });

    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const deleteClip = async (id: string, e?: React.MouseEvent) => {
    if (e) e.stopPropagation();
    try {
        const newHistory = await invoke<ClipItem[]>("delete_clip", { id });
        setHistory(newHistory);
    } catch (err) {
        console.error("Failed to delete clip:", err);
    }
  };

  const togglePin = async (id: string, e?: React.MouseEvent) => {
    if (e) e.stopPropagation();
    try {
        const newHistory = await invoke<ClipItem[]>("toggle_pin_clip", { id });
        setHistory(newHistory);
    } catch (err) {
        console.error("Failed to toggle pin:", err);
    }
  };

  const clearAll = async () => {
      try {
          const newHistory = await invoke<ClipItem[]>("clear_history");
          setHistory(newHistory);
      } catch (err) {
          console.error("Failed to clear history:", err);
      }
  };
    
  const copyToClipboard = async (content: string) => {
      try {
          await invoke("copy_to_clip", { content });
      } catch (err) {
          console.error("Failed to copy:", err);
      }
  };

  return { history, deleteClip, togglePin, clearAll, copyToClipboard };
}
