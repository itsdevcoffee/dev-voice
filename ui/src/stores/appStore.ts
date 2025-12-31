import { create } from 'zustand';

interface DaemonStatus {
  running: boolean;
  modelLoaded: boolean;
  gpuEnabled: boolean;
  gpuName?: string;
}

interface TranscriptionEntry {
  id: string;
  text: string;
  timestamp: number;
  durationMs: number;
  model: string;
}

interface IPCCall {
  id: string;
  timestamp: number;
  command: string;
  args?: any;
  result?: any;
  error?: string;
  durationMs: number;
}

interface LogEntry {
  id: string;
  timestamp: number;
  level: 'debug' | 'info' | 'warn' | 'error';
  message: string;
  source: 'ui' | 'daemon';
}

interface AppState {
  // Daemon state
  daemonStatus: DaemonStatus;
  isRecording: boolean;
  isProcessing: boolean;

  // Transcription history
  transcriptions: TranscriptionEntry[];

  // Dev tools
  ipcCalls: IPCCall[];
  logs: LogEntry[];

  // UI state
  activeView: 'dashboard' | 'settings' | 'history' | 'devtools';

  // Actions
  setDaemonStatus: (status: DaemonStatus) => void;
  setRecording: (recording: boolean) => void;
  setProcessing: (processing: boolean) => void;
  addTranscription: (entry: TranscriptionEntry) => void;
  setActiveView: (view: AppState['activeView']) => void;
  addIPCCall: (call: IPCCall) => void;
  addLog: (log: LogEntry) => void;
  clearLogs: () => void;
  clearIPCCalls: () => void;
}

export const useAppStore = create<AppState>((set) => ({
  // Initial state
  daemonStatus: {
    running: false,
    modelLoaded: false,
    gpuEnabled: false,
  },
  isRecording: false,
  isProcessing: false,
  transcriptions: [],
  ipcCalls: [],
  logs: [],
  activeView: 'dashboard',

  // Actions
  setDaemonStatus: (status) => set({ daemonStatus: status }),
  setRecording: (recording) => set({ isRecording: recording }),
  setProcessing: (processing) => set({ isProcessing: processing }),
  addTranscription: (entry) =>
    set((state) => ({
      transcriptions: [entry, ...state.transcriptions].slice(0, 50) // Keep last 50
    })),
  setActiveView: (view) => set({ activeView: view }),
  addIPCCall: (call) =>
    set((state) => ({
      ipcCalls: [call, ...state.ipcCalls].slice(0, 100) // Keep last 100
    })),
  addLog: (log) =>
    set((state) => ({
      logs: [log, ...state.logs].slice(0, 500) // Keep last 500
    })),
  clearLogs: () => set({ logs: [] }),
  clearIPCCalls: () => set({ ipcCalls: [] }),
}));
