import { invoke as tauriInvoke } from '@tauri-apps/api/core';
import { useAppStore } from '../stores/appStore';

/**
 * Wrapper around Tauri's invoke that automatically logs IPC calls to the dev tools
 */
export async function invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const startTime = Date.now();
  const callId = `${command}-${startTime}`;

  try {
    const result = await tauriInvoke<T>(command, args);
    const durationMs = Date.now() - startTime;

    // Log successful IPC call
    useAppStore.getState().addIPCCall({
      id: callId,
      timestamp: startTime,
      command,
      args,
      result,
      durationMs,
    });

    useAppStore.getState().addLog({
      id: callId,
      timestamp: startTime,
      level: 'info',
      message: `IPC: ${command} completed in ${durationMs}ms`,
      source: 'ui',
    });

    return result;
  } catch (error) {
    const durationMs = Date.now() - startTime;

    // Log failed IPC call
    useAppStore.getState().addIPCCall({
      id: callId,
      timestamp: startTime,
      command,
      args,
      error: String(error),
      durationMs,
    });

    useAppStore.getState().addLog({
      id: callId,
      timestamp: startTime,
      level: 'error',
      message: `IPC: ${command} failed - ${error}`,
      source: 'ui',
    });

    throw error;
  }
}
