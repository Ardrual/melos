import React from 'react';
import type { Provider } from '../llm/types';
import {
  DEFAULT_GEMINI_BASE_URL,
  DEFAULT_GEMINI_MODEL,
  DEFAULT_OPENROUTER_API_URL,
  DEFAULT_OPENROUTER_MODEL,
  GEMINI_MODEL_SUGGESTIONS,
  OPENROUTER_MODEL_SUGGESTIONS,
} from '../llm/constants';

interface ChatSettingsProps {
  provider: Provider;
  onProviderChange: (provider: Provider) => void;
  openrouterKey: string;
  onOpenrouterKeyChange: (value: string) => void;
  openrouterBaseUrl: string;
  onOpenrouterBaseUrlChange: (value: string) => void;
  openrouterModel: string;
  onOpenrouterModelChange: (value: string) => void;
  geminiKey: string;
  onGeminiKeyChange: (value: string) => void;
  geminiBaseUrl: string;
  onGeminiBaseUrlChange: (value: string) => void;
  geminiModel: string;
  onGeminiModelChange: (value: string) => void;
  temperature: number;
  onTemperatureChange: (value: number) => void;
  autoCheck: boolean;
  onAutoCheckChange: (value: boolean) => void;
  debugEnabled: boolean;
  onDebugEnabledChange: (value: boolean) => void;
}

const ChatSettings: React.FC<ChatSettingsProps> = ({
  provider,
  onProviderChange,
  openrouterKey,
  onOpenrouterKeyChange,
  openrouterBaseUrl,
  onOpenrouterBaseUrlChange,
  openrouterModel,
  onOpenrouterModelChange,
  geminiKey,
  onGeminiKeyChange,
  geminiBaseUrl,
  onGeminiBaseUrlChange,
  geminiModel,
  onGeminiModelChange,
  temperature,
  onTemperatureChange,
  autoCheck,
  onAutoCheckChange,
  debugEnabled,
  onDebugEnabledChange,
}) => {
  return (
    <div className="chat-settings">
      <div className="settings-row">
        <label htmlFor="provider-select">Provider</label>
        <select
          id="provider-select"
          value={provider}
          onChange={(event) => onProviderChange(event.target.value as Provider)}
        >
          <option value="openrouter">OpenRouter</option>
          <option value="gemini">Gemini</option>
        </select>
      </div>
      {provider === 'openrouter' && (
        <>
          <div className="settings-row">
            <label htmlFor="openrouter-key">OpenRouter API Key</label>
            <input
              id="openrouter-key"
              type="password"
              value={openrouterKey}
              onChange={(event) => onOpenrouterKeyChange(event.target.value)}
              placeholder="sk-or-..."
              autoComplete="off"
            />
            <button className="btn-secondary" onClick={() => onOpenrouterKeyChange('')}>Clear</button>
          </div>
          <div className="settings-row">
            <label htmlFor="openrouter-model">Model</label>
            <input
              id="openrouter-model"
              type="text"
              list="model-options"
              value={openrouterModel}
              onChange={(event) => onOpenrouterModelChange(event.target.value)}
              placeholder={DEFAULT_OPENROUTER_MODEL}
            />
            <datalist id="model-options">
              {OPENROUTER_MODEL_SUGGESTIONS.map((option) => (
                <option key={option} value={option} />
              ))}
            </datalist>
          </div>
          <div className="settings-row">
            <label htmlFor="openrouter-url">Endpoint</label>
            <input
              id="openrouter-url"
              type="text"
              value={openrouterBaseUrl}
              onChange={(event) => onOpenrouterBaseUrlChange(event.target.value)}
              placeholder={DEFAULT_OPENROUTER_API_URL}
            />
          </div>
        </>
      )}
      {provider === 'gemini' && (
        <>
          <div className="settings-row">
            <label htmlFor="gemini-key">Gemini API Key</label>
            <input
              id="gemini-key"
              type="password"
              value={geminiKey}
              onChange={(event) => onGeminiKeyChange(event.target.value)}
              placeholder="AIza..."
              autoComplete="off"
            />
            <button className="btn-secondary" onClick={() => onGeminiKeyChange('')}>Clear</button>
          </div>
          <div className="settings-row">
            <label htmlFor="gemini-model">Model</label>
            <input
              id="gemini-model"
              type="text"
              list="gemini-model-options"
              value={geminiModel}
              onChange={(event) => onGeminiModelChange(event.target.value)}
              placeholder={DEFAULT_GEMINI_MODEL}
            />
            <datalist id="gemini-model-options">
              {GEMINI_MODEL_SUGGESTIONS.map((option) => (
                <option key={option} value={option} />
              ))}
            </datalist>
          </div>
          <div className="settings-row">
            <label htmlFor="gemini-url">Endpoint base</label>
            <input
              id="gemini-url"
              type="text"
              value={geminiBaseUrl}
              onChange={(event) => onGeminiBaseUrlChange(event.target.value)}
              placeholder={DEFAULT_GEMINI_BASE_URL}
            />
          </div>
        </>
      )}
      <div className="settings-row">
        <label htmlFor="llm-temperature">Temperature</label>
        <input
          id="llm-temperature"
          type="number"
          min={0}
          max={1}
          step={0.05}
          value={temperature}
          onChange={(event) => {
            const next = Number(event.target.value);
            onTemperatureChange(Number.isFinite(next) ? next : 0.7);
          }}
        />
      </div>
      <div className="settings-row inline">
        <label htmlFor="llm-autocheck">Auto-check compile</label>
        <input
          id="llm-autocheck"
          type="checkbox"
          checked={autoCheck}
          onChange={(event) => onAutoCheckChange(event.target.checked)}
        />
        <span className="settings-note">Send compiler results back to the model.</span>
      </div>
      <div className="settings-row inline">
        <label htmlFor="llm-debug">Show debug</label>
        <input
          id="llm-debug"
          type="checkbox"
          checked={debugEnabled}
          onChange={(event) => onDebugEnabledChange(event.target.checked)}
        />
        <span className="settings-note">Expose request/response payloads.</span>
      </div>
    </div>
  );
};

export default ChatSettings;
