import { create } from 'zustand';
import { AppState, AudioDevice, SummaryResult, AppConfig, TranscriptionSegment } from '../types';

interface AppStore extends AppState {
  setIsRecording: (isRecording: boolean) => void;
  setRecordingDuration: (duration: number) => void;
  setAudioDevices: (devices: AudioDevice[]) => void;
  setSelectedDevice: (deviceId: string | null) => void;
  setAudioPath: (path: string | null) => void;
  setTranscript: (transcript: string) => void;
  setTranscriptSegments: (segments: TranscriptionSegment[]) => void;
  setIsTranscribing: (isTranscribing: boolean) => void;
  setTranscriptionProgress: (progress: number) => void;
  setSummary: (summary: SummaryResult | null) => void;
  setIsSummarizing: (isSummarizing: boolean) => void;
  setConfig: (config: AppConfig | null) => void;
  setShowSettings: (show: boolean) => void;
  setNotification: (message: string | null) => void;
  resetSession: () => void;
}

const initialState: AppState = {
  isRecording: false,
  recordingDuration: 0,
  audioDevices: [],
  selectedDevice: null,
  audioPath: null,
  transcript: '',
  transcriptSegments: [],
  isTranscribing: false,
  transcriptionProgress: 0,
  summary: null,
  isSummarizing: false,
  config: null,
  showSettings: false,
  notification: null,
};

export const useAppStore = create<AppStore>((set) => ({
  ...initialState,

  setIsRecording: (isRecording) => set({ isRecording }),
  setRecordingDuration: (recordingDuration) => set({ recordingDuration }),
  setAudioDevices: (audioDevices) => set({ audioDevices }),
  setSelectedDevice: (selectedDevice) => set({ selectedDevice }),
  setAudioPath: (audioPath) => set({ audioPath }),
  setTranscript: (transcript) => set({ transcript }),
  setTranscriptSegments: (transcriptSegments) => set({ transcriptSegments }),
  setIsTranscribing: (isTranscribing) => set({ isTranscribing }),
  setTranscriptionProgress: (transcriptionProgress) => set({ transcriptionProgress }),
  setSummary: (summary) => set({ summary }),
  setIsSummarizing: (isSummarizing) => set({ isSummarizing }),
  setConfig: (config) => set({ config }),
  setShowSettings: (showSettings) => set({ showSettings }),
  setNotification: (notification) => set({ notification }),

  resetSession: () => set({
    isRecording: false,
    recordingDuration: 0,
    audioPath: null,
    transcript: '',
    transcriptSegments: [],
    isTranscribing: false,
    transcriptionProgress: 0,
    summary: null,
    isSummarizing: false,
  }),
}));
