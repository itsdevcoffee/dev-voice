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

interface AppState {
  // Daemon state
  daemonStatus: DaemonStatus;
  isRecording: boolean;
  isProcessing: boolean;

  // Transcription history
  transcriptions: TranscriptionEntry[];

  // UI state
  activeView: 'dashboard' | 'settings' | 'history' | 'devtools';

  // Actions
  setDaemonStatus: (status: DaemonStatus) => void;
  setRecording: (recording: boolean) => void;
  setProcessing: (processing: boolean) => void;
  addTranscription: (entry: TranscriptionEntry) => void;
  setActiveView: (view: AppState['activeView']) => void;
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
}));
