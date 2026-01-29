export type Provider = 'openrouter' | 'gemini';

export type LlmRole = 'system' | 'user' | 'assistant';

export interface LlmMessage {
  role: LlmRole;
  content: string;
}

export interface LlmContext {
  source: string;
  report: string;
}

export interface DebugRecord {
  provider: Provider;
  endpoint: string;
  payload: unknown;
}

export interface DebugCallbacks {
  onRequest?: (record: DebugRecord) => void;
  onResponse?: (data: unknown) => void;
  onError?: (message: string) => void;
}
