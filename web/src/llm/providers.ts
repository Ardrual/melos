import type { DebugCallbacks, LlmContext, LlmMessage, Provider } from './types';
import { buildSystemPrompt } from './prompt';
import { DEFAULT_GEMINI_BASE_URL, DEFAULT_OPENROUTER_API_URL } from './constants';

interface OpenRouterConfig {
  apiKey: string;
  model: string;
  baseUrl?: string;
}

interface GeminiConfig {
  apiKey: string;
  model: string;
  baseUrl?: string;
}

export interface LlmRequestOptions {
  provider: Provider;
  temperature: number;
  openrouter: OpenRouterConfig;
  gemini: GeminiConfig;
  debug?: DebugCallbacks;
}

export const buildGeminiEndpoint = (baseUrl: string, model: string) => {
  const trimmed = baseUrl.replace(/\/$/, '');
  return `${trimmed}/${model}:generateContent`;
};

const parseJson = async (response: Response) => {
  try {
    return await response.json();
  } catch {
    return { error: 'Invalid JSON response.' };
  }
};

const handleError = (message: string, debug?: DebugCallbacks) => {
  debug?.onError?.(message);
  throw new Error(message);
};

const requestOpenRouter = async (
  history: LlmMessage[],
  context: LlmContext,
  options: LlmRequestOptions,
) => {
  const { apiKey, model, baseUrl } = options.openrouter;
  if (!apiKey.trim()) {
    handleError('Missing OpenRouter API key.', options.debug);
  }
  if (!model.trim()) {
    handleError('Missing OpenRouter model name.', options.debug);
  }

  const payload = {
    model,
    temperature: options.temperature,
    messages: [
      { role: 'system', content: buildSystemPrompt(context.source, context.report) },
      ...history,
    ],
  };

  const endpoint = (baseUrl || DEFAULT_OPENROUTER_API_URL).trim() || DEFAULT_OPENROUTER_API_URL;
  options.debug?.onRequest?.({ provider: 'openrouter', endpoint, payload });
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      Authorization: `Bearer ${apiKey}`,
      'Content-Type': 'application/json',
      'HTTP-Referer': window.location.origin,
      'X-Title': 'Melos AI Studio',
    },
    body: JSON.stringify(payload),
  });

  const data = await parseJson(response);
  options.debug?.onResponse?.(data);

  if (!response.ok) {
    const errorText = typeof data?.error === 'string'
      ? data.error
      : data?.error?.message || response.statusText;
    handleError(`OpenRouter error ${response.status}: ${errorText || response.statusText}`, options.debug);
  }

  const content = data?.choices?.[0]?.message?.content;
  if (!content) {
    handleError('OpenRouter returned an empty response.', options.debug);
  }
  return content as string;
};

const requestGemini = async (
  history: LlmMessage[],
  context: LlmContext,
  options: LlmRequestOptions,
) => {
  const { apiKey, model, baseUrl } = options.gemini;
  if (!apiKey.trim()) {
    handleError('Missing Gemini API key.', options.debug);
  }
  if (!model.trim()) {
    handleError('Missing Gemini model name.', options.debug);
  }

  const contents = history
    .filter((entry) => entry.role !== 'system')
    .map((entry) => ({
      role: entry.role === 'assistant' ? 'model' : 'user',
      parts: [{ text: entry.content }],
    }));

  const payload = {
    system_instruction: {
      parts: [{ text: buildSystemPrompt(context.source, context.report) }],
    },
    contents,
    generationConfig: {
      temperature: options.temperature,
    },
  };

  const endpoint = buildGeminiEndpoint(baseUrl?.trim() || DEFAULT_GEMINI_BASE_URL, model);
  options.debug?.onRequest?.({ provider: 'gemini', endpoint, payload });
  const response = await fetch(endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'x-goog-api-key': apiKey,
    },
    body: JSON.stringify(payload),
  });

  const data = await parseJson(response);
  options.debug?.onResponse?.(data);

  if (!response.ok) {
    const errorText = typeof data?.error === 'string'
      ? data.error
      : data?.error?.message || response.statusText;
    handleError(`Gemini error ${response.status}: ${errorText || response.statusText}`, options.debug);
  }

  const parts = data?.candidates?.[0]?.content?.parts;
  const content = Array.isArray(parts)
    ? parts.map((part: { text?: string }) => part.text ?? '').join('')
    : '';
  if (!content) {
    handleError('Gemini returned an empty response.', options.debug);
  }
  return content;
};

export const requestCompletion = async (
  history: LlmMessage[],
  context: LlmContext,
  options: LlmRequestOptions,
) => {
  if (options.provider === 'gemini') {
    return requestGemini(history, context, options);
  }
  return requestOpenRouter(history, context, options);
};
