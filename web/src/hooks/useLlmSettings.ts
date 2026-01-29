import { useEffect, useState } from 'react';
import type { Provider } from '../llm/types';
import {
  DEFAULT_GEMINI_BASE_URL,
  DEFAULT_GEMINI_MODEL,
  DEFAULT_OPENROUTER_API_URL,
  DEFAULT_OPENROUTER_MODEL,
} from '../llm/constants';

export const useLlmSettings = () => {
  const [provider, setProvider] = useState<Provider>('openrouter');
  const [openrouterKey, setOpenrouterKey] = useState('');
  const [openrouterBaseUrl, setOpenrouterBaseUrl] = useState(DEFAULT_OPENROUTER_API_URL);
  const [openrouterModel, setOpenrouterModel] = useState(DEFAULT_OPENROUTER_MODEL);
  const [geminiKey, setGeminiKey] = useState('');
  const [geminiBaseUrl, setGeminiBaseUrl] = useState(DEFAULT_GEMINI_BASE_URL);
  const [geminiModel, setGeminiModel] = useState(DEFAULT_GEMINI_MODEL);
  const [temperature, setTemperature] = useState(0.7);
  const [autoCheck, setAutoCheck] = useState(true);
  const [debugEnabled, setDebugEnabled] = useState(false);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    const storedProvider = window.localStorage.getItem('melos.provider');
    const storedOpenrouterKey = window.localStorage.getItem('melos.openrouter.key');
    const storedOpenrouterModel = window.localStorage.getItem('melos.openrouter.model');
    const storedOpenrouterUrl = window.localStorage.getItem('melos.openrouter.apiUrl');
    const storedGeminiKey = window.localStorage.getItem('melos.gemini.key');
    const storedGeminiModel = window.localStorage.getItem('melos.gemini.model');
    const storedGeminiUrl = window.localStorage.getItem('melos.gemini.baseUrl');
    const storedTemp = window.localStorage.getItem('melos.openrouter.temperature');
    const storedAuto = window.localStorage.getItem('melos.openrouter.autoCheck');
    const storedDebug = window.localStorage.getItem('melos.debug.enabled');

    if (storedProvider === 'openrouter' || storedProvider === 'gemini') {
      setProvider(storedProvider);
    }
    if (storedOpenrouterKey) setOpenrouterKey(storedOpenrouterKey);
    if (storedOpenrouterModel) setOpenrouterModel(storedOpenrouterModel);
    if (storedOpenrouterUrl) setOpenrouterBaseUrl(storedOpenrouterUrl);
    if (storedGeminiKey) setGeminiKey(storedGeminiKey);
    if (storedGeminiModel) setGeminiModel(storedGeminiModel);
    if (storedGeminiUrl) setGeminiBaseUrl(storedGeminiUrl);
    if (storedTemp) {
      const parsed = Number(storedTemp);
      if (Number.isFinite(parsed)) setTemperature(parsed);
    }
    if (storedAuto) setAutoCheck(storedAuto === 'true');
    if (storedDebug) setDebugEnabled(storedDebug === 'true');
  }, []);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.provider', provider);
  }, [provider]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.openrouter.key', openrouterKey);
  }, [openrouterKey]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.openrouter.model', openrouterModel);
  }, [openrouterModel]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.openrouter.apiUrl', openrouterBaseUrl);
  }, [openrouterBaseUrl]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.gemini.key', geminiKey);
  }, [geminiKey]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.gemini.model', geminiModel);
  }, [geminiModel]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.gemini.baseUrl', geminiBaseUrl);
  }, [geminiBaseUrl]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.openrouter.temperature', temperature.toString());
  }, [temperature]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.openrouter.autoCheck', autoCheck ? 'true' : 'false');
  }, [autoCheck]);

  useEffect(() => {
    if (typeof window === 'undefined') return;
    window.localStorage.setItem('melos.debug.enabled', debugEnabled ? 'true' : 'false');
  }, [debugEnabled]);

  return {
    provider,
    setProvider,
    openrouterKey,
    setOpenrouterKey,
    openrouterBaseUrl,
    setOpenrouterBaseUrl,
    openrouterModel,
    setOpenrouterModel,
    geminiKey,
    setGeminiKey,
    geminiBaseUrl,
    setGeminiBaseUrl,
    geminiModel,
    setGeminiModel,
    temperature,
    setTemperature,
    autoCheck,
    setAutoCheck,
    debugEnabled,
    setDebugEnabled,
  };
};
