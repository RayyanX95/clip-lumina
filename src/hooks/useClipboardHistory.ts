import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export interface ClipItem {
  id: string;
  content: string;
  timestamp: number;
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
    if (e) e.stopPropagation(); // Prevent triggering parent click events
    try {
        const newHistory = await invoke<ClipItem[]>("delete_clip", { id });
        setHistory(newHistory);
    } catch (err) {
        console.error("Failed to delete clip:", err);
    }
  };

  const copyToClipboard = async (content: string) => {
      try {
          await invoke("copy_to_clip", { content });
      } catch (err) {
          console.error("Failed to copy:", err);
      }
  };

  return { history, deleteClip, copyToClipboard };
}
