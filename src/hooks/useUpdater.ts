import { useEffect } from 'react';
import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { listen } from '@tauri-apps/api/event';
import { relaunch } from '@tauri-apps/plugin-process';
import { invoke } from '@tauri-apps/api/core';

export function useUpdater() {
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen('update-check', async () => {
        await checkForUpdates(false);
      });
    };

    setupListener();

    // Also check automatically on startup (silently)
    checkForUpdates(true);

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const checkForUpdates = async (silent: boolean) => {
    // In development, the updater plugin will throw an error because the app isn't signed.
    if (import.meta.env.DEV) {
      if (!silent) {
        await message('The auto-updater only works in the built/production app (signed/bundled).', { 
          title: 'Development Mode', 
          kind: 'info' 
        });
      }
      return;
    }

    try {
      await invoke('set_suppress_hide', { suppress: true });
      const update = await check();
      if (update) {
        const yes = await ask(
          `Update to ${update.version} is available!\n\nRelease notes: ${update.body}`, 
          { 
            title: 'Update Available', 
            kind: 'info', 
            okLabel: 'Update', 
            cancelLabel: 'Cancel' 
          }
        );
        if (yes) {
          await update.downloadAndInstall();
          // Ask to restart
          await message('Update installed. The app will now restart.', { title: 'Update Complete' });
          await relaunch();
        }
      } else {
        if (!silent) {
          await message('You are on the latest version.', { title: 'No Updates' });
        }
      }
    } catch (error) {
      console.error(error);
      if (!silent) {
        await message(`Error checking for updates: ${error}`, { title: 'Update Error', kind: 'error' });
      }
    } finally {
      await invoke('set_suppress_hide', { suppress: false });
    }
  };
}
