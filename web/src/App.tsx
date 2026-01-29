import { useState, useEffect, useCallback, useRef, useMemo } from 'react'
import type { KeyboardEvent, ReactNode } from 'react'
import './App.css'
import Editor from './components/Editor'
import ScoreDisplay from './components/ScoreDisplay'
import ChatSettings from './components/ChatSettings'
import DebugPanel from './components/DebugPanel'
import { useLlmSettings } from './hooks/useLlmSettings'
import { extractMelosCode } from './llm/prompt'
import { requestCompletion } from './llm/providers'
import { MAX_HISTORY } from './llm/constants'
import type { DebugCallbacks, DebugRecord, LlmMessage } from './llm/types'
import init, { compile_to_musicxml, compile_to_midi } from './melos-wasm/melos'
import { Midi } from '@tonejs/midi'
import Soundfont from 'soundfont-player'

const GM_INSTRUMENTS = [
  'acoustic_grand_piano',
  'bright_acoustic_piano',
  'electric_grand_piano',
  'honky_tonk_piano',
  'electric_piano_1',
  'electric_piano_2',
  'harpsichord',
  'clavinet',
  'celesta',
  'glockenspiel',
  'music_box',
  'vibraphone',
  'marimba',
  'xylophone',
  'tubular_bells',
  'dulcimer',
  'drawbar_organ',
  'percussive_organ',
  'rock_organ',
  'church_organ',
  'reed_organ',
  'accordion',
  'harmonica',
  'tango_accordion',
  'acoustic_guitar_nylon',
  'acoustic_guitar_steel',
  'electric_guitar_jazz',
  'electric_guitar_clean',
  'electric_guitar_muted',
  'overdriven_guitar',
  'distortion_guitar',
  'guitar_harmonics',
  'acoustic_bass',
  'electric_bass_finger',
  'electric_bass_pick',
  'fretless_bass',
  'slap_bass_1',
  'slap_bass_2',
  'synth_bass_1',
  'synth_bass_2',
  'violin',
  'viola',
  'cello',
  'contrabass',
  'tremolo_strings',
  'pizzicato_strings',
  'orchestral_harp',
  'timpani',
  'string_ensemble_1',
  'string_ensemble_2',
  'synth_strings_1',
  'synth_strings_2',
  'choir_aahs',
  'voice_oohs',
  'synth_voice',
  'orchestra_hit',
  'trumpet',
  'trombone',
  'tuba',
  'muted_trumpet',
  'french_horn',
  'brass_section',
  'synth_brass_1',
  'synth_brass_2',
  'soprano_sax',
  'alto_sax',
  'tenor_sax',
  'baritone_sax',
  'oboe',
  'english_horn',
  'bassoon',
  'clarinet',
  'piccolo',
  'flute',
  'recorder',
  'pan_flute',
  'blown_bottle',
  'shakuhachi',
  'whistle',
  'ocarina',
  'lead_1_square',
  'lead_2_sawtooth',
  'lead_3_calliope',
  'lead_4_chiff',
  'lead_5_charang',
  'lead_6_voice',
  'lead_7_fifths',
  'lead_8_bass__lead',
  'pad_1_new_age',
  'pad_2_warm',
  'pad_3_polysynth',
  'pad_4_choir',
  'pad_5_bowed',
  'pad_6_metallic',
  'pad_7_halo',
  'pad_8_sweep',
  'fx_1_rain',
  'fx_2_soundtrack',
  'fx_3_crystal',
  'fx_4_atmosphere',
  'fx_5_brightness',
  'fx_6_goblins',
  'fx_7_echoes',
  'fx_8_scifi',
  'sitar',
  'banjo',
  'shamisen',
  'koto',
  'kalimba',
  'bagpipe',
  'fiddle',
  'shanai',
  'tinkle_bell',
  'agogo',
  'steel_drums',
  'woodblock',
  'taiko_drum',
  'melodic_tom',
  'synth_drum',
  'reverse_cymbal',
  'guitar_fret_noise',
  'breath_noise',
  'seashore',
  'bird_tweet',
  'telephone_ring',
  'helicopter',
  'applause',
  'gunshot',
];

const DEFAULT_CODE = `Part: "Piano" Instrument: "Acoustic Grand Piano" {
  | C4q D4q E4q F4q |
  | G4q A4q B4q C5q |
}`;

type ChatRole = 'user' | 'assistant' | 'system';

interface ChatMessage {
  id: string;
  role: ChatRole;
  content: string;
  kind?: 'compile' | 'status' | 'error';
}

interface CompileReport {
  ok: boolean;
  summary: string;
  xmlError?: string;
  midiError?: string;
}

function App() {
  const [code, setCode] = useState(DEFAULT_CODE);
  const [xml, setXml] = useState('');
  const [error, setError] = useState('');
  const [wasmReady, setWasmReady] = useState(false);
  const [showTextScore, setShowTextScore] = useState(true);
  const [showSheetScore, setShowSheetScore] = useState(true);
  const [midiUrl, setMidiUrl] = useState('');
  const [midiError, setMidiError] = useState('');
  const [midiData, setMidiData] = useState<Uint8Array | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [playbackPosition, setPlaybackPosition] = useState(0);
  const [playbackDuration, setPlaybackDuration] = useState(0);
  const [playbackError, setPlaybackError] = useState('');
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 'welcome',
      role: 'assistant',
      content: 'Welcome back. Share the mood or edits you want, and I will draft the Melos score.',
    },
  ]);
  const [chatInput, setChatInput] = useState('');
  const [isSending, setIsSending] = useState(false);
  const [chatError, setChatError] = useState('');
  const [compileReport, setCompileReport] = useState('Compilation not run yet.');
  const [showSettings, setShowSettings] = useState(false);
  const [debugRequest, setDebugRequest] = useState('');
  const [debugResponse, setDebugResponse] = useState('');
  const [debugError, setDebugError] = useState('');
  const {
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
  } = useLlmSettings();
  const audioContextRef = useRef<AudioContext | null>(null);
  const instrumentsRef = useRef<Map<string, any>>(new Map());
  const activeNodesRef = useRef<any[]>([]);
  const playbackStartRef = useRef<number>(0);
  const rafRef = useRef<number | null>(null);
  const chatStreamRef = useRef<HTMLDivElement | null>(null);
  const llmHistoryRef = useRef<LlmMessage[]>([]);
  const requestIdRef = useRef(0);
  const autoScrollRef = useRef(true);

  useEffect(() => {
    init().then(() => {
      setWasmReady(true);
    });
  }, []);

  useEffect(() => {
    const stream = chatStreamRef.current;
    if (!stream) return;
    if (!autoScrollRef.current) return;
    const frame = requestAnimationFrame(() => {
      stream.scrollTop = stream.scrollHeight;
    });
    return () => cancelAnimationFrame(frame);
  }, [messages, showSettings, debugEnabled]);

  const handleChatScroll = () => {
    const stream = chatStreamRef.current;
    if (!stream) return;
    const threshold = 24;
    const distanceFromBottom = stream.scrollHeight - stream.scrollTop - stream.clientHeight;
    autoScrollRef.current = distanceFromBottom <= threshold;
  };

  const getAudioContext = () => {
    if (!audioContextRef.current) {
      audioContextRef.current = new AudioContext();
    }
    return audioContextRef.current;
  };

  const loadInstrument = async (program: number) => {
    const key = `program-${program}`;
    const cached = instrumentsRef.current.get(key);
    if (cached) return cached;

    const ac = getAudioContext();
    const instrumentName = GM_INSTRUMENTS[program] || 'acoustic_grand_piano';

    const instrument = await Soundfont.instrument(ac, instrumentName, {
      soundfont: 'FluidR3_GM',
      format: 'mp3',
    });
    instrumentsRef.current.set(key, instrument);
    return instrument;
  };

  const stopPlayback = useCallback((resetPosition: boolean) => {
    activeNodesRef.current.forEach((node) => {
      try {
        node.stop?.();
      } catch (err) {
        // Ignore stop errors from already-ended nodes.
      }
    });
    activeNodesRef.current = [];
    if (rafRef.current) {
      cancelAnimationFrame(rafRef.current);
      rafRef.current = null;
    }
    setIsPlaying(false);
    if (resetPosition) setPlaybackPosition(0);
  }, []);

  const startPlayback = useCallback(async () => {
    if (!midiData) return;
    stopPlayback(false);
    setPlaybackError('');

    let midi: Midi;
    try {
      midi = new Midi(midiData);
    } catch (err: any) {
      setPlaybackError(`Playback error: ${err?.toString?.() || 'Invalid MIDI data.'}`);
      return;
    }

    const ac = getAudioContext();
    try {
      await ac.resume();
    } catch (err: any) {
      setPlaybackError(`Playback error: ${err?.toString?.() || 'Unable to start audio.'}`);
      return;
    }

    const instrumentKeys = new Set<string>();
    midi.tracks.forEach((track) => {
      if (!track.notes.length) return;
      const program = Number.isFinite(track.instrument?.number)
        ? track.instrument.number
        : 0;
      instrumentKeys.add(`program-${program}`);
    });

    try {
      await Promise.all(Array.from(instrumentKeys).map(async (key) => {
        const program = Number(key.replace('program-', ''));
        await loadInstrument(program);
      }));
    } catch (err: any) {
      setPlaybackError(`Playback error: ${err?.toString?.() || 'Unable to load instruments.'}`);
      return;
    }

    const startTime = ac.currentTime + 0.08;
    playbackStartRef.current = startTime;
    setPlaybackDuration(midi.duration);
    setPlaybackPosition(0);
    setIsPlaying(true);

    midi.tracks.forEach((track) => {
      if (!track.notes.length) return;
      const program = Number.isFinite(track.instrument?.number)
        ? track.instrument.number
        : 0;
      const key = `program-${program}`;
      const instrument = instrumentsRef.current.get(key);
      if (!instrument) return;

      track.notes.forEach((note) => {
        const when = startTime + note.time;
        const duration = Math.max(note.duration, 0.02);
        const gain = Math.min(Math.max(note.velocity ?? 0.8, 0), 1);
        const node = instrument.play(note.midi, when, { duration, gain });
        if (node) activeNodesRef.current.push(node);
      });
    });

    const tick = () => {
      const elapsed = Math.max(0, ac.currentTime - playbackStartRef.current);
      if (elapsed >= midi.duration) {
        setPlaybackPosition(midi.duration);
        stopPlayback(false);
        return;
      }
      setPlaybackPosition(elapsed);
      rafRef.current = requestAnimationFrame(tick);
    };
    rafRef.current = requestAnimationFrame(tick);
  }, [midiData, stopPlayback]);

  const formatTime = (value: number) => {
    const totalSeconds = Math.max(0, Math.floor(value));
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes}:${seconds.toString().padStart(2, '0')}`;
  };

  const compile = useCallback((source: string): CompileReport | null => {
    if (!wasmReady) {
      const report = {
        ok: false,
        summary: 'Compiler not ready yet.',
      };
      setCompileReport(report.summary);
      return report;
    }

    let xmlError = '';
    let midiErrorText = '';
    try {
      const musicXml = compile_to_musicxml(source);
      setXml(musicXml);
    } catch (e: any) {
      xmlError = e?.toString?.() || String(e);
      setXml('');
    }

    try {
      const midiBytes = compile_to_midi(source);
      const midiArray = midiBytes instanceof Uint8Array ? midiBytes : new Uint8Array(midiBytes as any);
      const blob = new Blob([midiBytes as any], { type: 'audio/midi' });
      const url = URL.createObjectURL(blob);
      setMidiUrl((prev) => {
        if (prev) URL.revokeObjectURL(prev);
        return url;
      });
      setMidiError('');
      setMidiData(midiArray);
    } catch (e: any) {
      midiErrorText = e?.toString?.() || String(e);
      setMidiUrl((prev) => {
        if (prev) URL.revokeObjectURL(prev);
        return '';
      });
      setMidiError(`MIDI Error: ${midiErrorText}`);
      setMidiData(null);
    }

    let summary = '';
    if (!xmlError && !midiErrorText) {
      summary = 'Compilation succeeded. MusicXML and MIDI generated.';
    } else {
      const parts = [
        xmlError ? `MusicXML error: ${xmlError}` : '',
        midiErrorText ? `MIDI error: ${midiErrorText}` : '',
      ].filter(Boolean);
      summary = `Compilation failed. ${parts.join(' ')}`.trim();
    }

    setError(xmlError || (midiErrorText ? `MIDI Error: ${midiErrorText}` : ''));
    setCompileReport(summary);

    return {
      ok: !xmlError && !midiErrorText,
      summary,
      xmlError: xmlError || undefined,
      midiError: midiErrorText || undefined,
    };
  }, [wasmReady]);

  const downloadMidi = () => {
    if (!wasmReady) return;
    try {
      const midiBytes = compile_to_midi(code);
      const blob = new Blob([midiBytes as any], { type: 'audio/midi' });
      const url = URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      a.download = 'score.mid';
      a.click();
      URL.revokeObjectURL(url);
    } catch (e: any) {
      setError("MIDI Export Error: " + e.toString());
    }
  };

  const makeId = () => {
    if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
      return crypto.randomUUID();
    }
    return `${Date.now()}-${Math.random().toString(16).slice(2)}`;
  };

  const debugCallbacks = useMemo<DebugCallbacks | undefined>(() => {
    if (!debugEnabled) return undefined;
    return {
      onRequest: (record: DebugRecord) => {
        setDebugRequest(
          JSON.stringify(
            {
              provider: record.provider,
              endpoint: record.endpoint,
              payload: record.payload,
            },
            null,
            2,
          ),
        );
        setDebugResponse('');
        setDebugError('');
      },
      onResponse: (data: unknown) => {
        setDebugResponse(JSON.stringify(data, null, 2));
      },
      onError: (message: string) => {
        setDebugError(message);
      },
    };
  }, [debugEnabled]);

  const requestOptions = useMemo(() => ({
    provider,
    temperature,
    openrouter: {
      apiKey: openrouterKey,
      model: openrouterModel,
      baseUrl: openrouterBaseUrl,
    },
    gemini: {
      apiKey: geminiKey,
      model: geminiModel,
      baseUrl: geminiBaseUrl,
    },
    debug: debugCallbacks,
  }), [
    provider,
    temperature,
    openrouterKey,
    openrouterModel,
    openrouterBaseUrl,
    geminiKey,
    geminiModel,
    geminiBaseUrl,
    debugCallbacks,
  ]);

  const appendHistory = (entry: LlmMessage) => {
    const next = [...llmHistoryRef.current, entry].slice(-MAX_HISTORY);
    llmHistoryRef.current = next;
    return next;
  };

  const renderMessageContent = (content: string) => {
    const nodes: ReactNode[] = [];
    const regex = /```([\w-]*)\n?([\s\S]*?)```/g;
    let lastIndex = 0;
    let match: RegExpExecArray | null;
    let keyIndex = 0;

    while ((match = regex.exec(content)) !== null) {
      const before = content.slice(lastIndex, match.index);
      if (before) {
        nodes.push(
          <span className="chat-text" key={`text-${keyIndex++}`}>
            {before}
          </span>,
        );
      }
      const code = match[2].trim();
      nodes.push(
        <pre key={`code-${keyIndex++}`}>
          <code>{code}</code>
        </pre>,
      );
      lastIndex = regex.lastIndex;
    }

    const rest = content.slice(lastIndex);
    if (rest) {
      nodes.push(
        <span className="chat-text" key={`text-${keyIndex++}`}>
          {rest}
        </span>,
      );
    }

    return nodes;
  };

  const handleAssistantContent = async (assistantContent: string, autoFixLevel: number) => {
    const nextCode = extractMelosCode(assistantContent);
    if (!nextCode) return;

    setCode(nextCode);
    const report = compile(nextCode);
    if (report) {
      setMessages((prev) => [
        ...prev,
        {
          id: makeId(),
          role: 'system',
          kind: 'compile',
          content: report.summary,
        },
      ]);
    }

    if (autoCheck && report && autoFixLevel < 1) {
      setMessages((prev) => [
        ...prev,
        {
          id: makeId(),
          role: 'system',
          kind: 'status',
          content: report.ok
            ? 'Auto-check: sharing compilation results with the model.'
            : 'Auto-fix: asking the model to resolve compilation errors.',
        },
      ]);
      const autoPrompt = report.ok
        ? [
          'Compilation result after applying your edits:',
          report.summary,
          'If everything looks correct, reply with a brief confirmation and no code block.',
        ].join('\n')
        : [
          'Compilation result after applying your edits:',
          report.summary,
          'Please return a corrected full Melos score inside one ```melos``` block.',
        ].join('\n');
      const history = appendHistory({ role: 'user', content: autoPrompt });
      const response = await requestCompletion(history, { source: nextCode, report: report.summary }, requestOptions);
      appendHistory({ role: 'assistant', content: response });
      setMessages((prev) => [
        ...prev,
        {
          id: makeId(),
          role: 'assistant',
          content: response,
        },
      ]);
      await handleAssistantContent(response, autoFixLevel + 1);
    }
  };

  const sendPrompt = async (prompt: string, displayPrompt: boolean) => {
    const requestId = ++requestIdRef.current;
    setChatError('');

    const userEntry: LlmMessage = { role: 'user', content: prompt };
    const history = appendHistory(userEntry);

    if (displayPrompt) {
      setMessages((prev) => [
        ...prev,
        {
          id: makeId(),
          role: 'user',
          content: prompt,
        },
      ]);
    }

    const response = await requestCompletion(history, { source: code, report: compileReport }, requestOptions);
    if (requestId !== requestIdRef.current) return;

    appendHistory({ role: 'assistant', content: response });
    setMessages((prev) => [
      ...prev,
      {
        id: makeId(),
        role: 'assistant',
        content: response,
      },
    ]);

    await handleAssistantContent(response, 0);
  };

  const handleSend = async () => {
    const trimmed = chatInput.trim();
    if (!trimmed || isSending) return;
    if (!wasmReady) {
      setChatError('Compiler is still loading. Try again in a moment.');
      return;
    }
    if (!isConfigured) {
      setChatError(provider === 'openrouter'
        ? 'Add an OpenRouter API key and model to send messages.'
        : 'Add a Gemini API key and model to send messages.');
      return;
    }

    setIsSending(true);
    setChatInput('');
    try {
      await sendPrompt(trimmed, true);
    } catch (err: any) {
      setChatError(err?.message || 'Unable to reach the model.');
    } finally {
      setIsSending(false);
    }
  };

  const handleQuickAction = async (prompt: string) => {
    if (isSending) return;
    if (!wasmReady) {
      setChatError('Compiler is still loading. Try again in a moment.');
      return;
    }
    if (!isConfigured) {
      setChatError(provider === 'openrouter'
        ? 'Add an OpenRouter API key and model to send messages.'
        : 'Add a Gemini API key and model to send messages.');
      return;
    }
    setIsSending(true);
    try {
      await sendPrompt(prompt, true);
    } catch (err: any) {
      setChatError(err?.message || 'Unable to reach the model.');
    } finally {
      setIsSending(false);
    }
  };

  const handleInputKeyDown = (event: KeyboardEvent<HTMLTextAreaElement>) => {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      handleSend();
    }
  };

  useEffect(() => {
    const timer = setTimeout(() => {
      compile(code);
    }, 500);
    return () => clearTimeout(timer);
  }, [code, compile]);

  useEffect(() => {
    stopPlayback(true);
    setPlaybackError('');
    if (!midiData) {
      setPlaybackDuration(0);
      return;
    }
    try {
      const midi = new Midi(midiData);
      setPlaybackDuration(midi.duration);
    } catch {
      setPlaybackDuration(0);
    }
  }, [midiData, stopPlayback]);

  useEffect(() => {
    return () => {
      stopPlayback(false);
      if (midiUrl) URL.revokeObjectURL(midiUrl);
      audioContextRef.current?.close?.();
    };
  }, [midiUrl, stopPlayback]);

  const layout = showTextScore && showSheetScore
    ? 'both'
    : showTextScore
      ? 'left'
      : showSheetScore
        ? 'right'
        : 'center';
  const isOpenrouterConfigured = openrouterKey.trim().length > 0 && openrouterModel.trim().length > 0;
  const isGeminiConfigured = geminiKey.trim().length > 0 && geminiModel.trim().length > 0;
  const isConfigured = provider === 'gemini' ? isGeminiConfigured : isOpenrouterConfigured;
  const statusLabel = isSending
    ? 'Sending…'
    : !wasmReady
      ? 'Loading compiler…'
      : isConfigured
        ? 'Ready'
        : provider === 'gemini'
          ? 'Add Gemini key'
          : 'Add OpenRouter key';

  return (
    <div className="app-container">
      <header className="app-header">
        <div className="header-left">
          <div className="brand">
            <span className="brand-mark">Melos</span>
            <span className="brand-subtitle">AI Studio</span>
          </div>
          <span className="brand-tag">Compose. Shape. Perform.</span>
        </div>
        <div className="header-center">
          {error && <div className="error-banner">{error}</div>}
        </div>
        <div className="header-right">
          <button
            onClick={() => setShowTextScore((prev) => !prev)}
            className={`btn-toggle ${showTextScore ? 'active' : ''}`}
            aria-pressed={showTextScore}
          >
            Text Score
          </button>
          <button
            onClick={() => setShowSheetScore((prev) => !prev)}
            className={`btn-toggle ${showSheetScore ? 'active' : ''}`}
            aria-pressed={showSheetScore}
          >
            Sheet Music
          </button>
          <button onClick={downloadMidi} className="btn-secondary">Download MIDI</button>
        </div>
      </header>
      <main className="app-main" data-layout={layout}>
        {showTextScore && (
          <aside className="sidebar left">
            <div className="sidebar-header">
              <div>
                <h3>Text Score</h3>
                <span className="sidebar-meta">Melos syntax</span>
              </div>
              <button className="icon-btn" onClick={() => setShowTextScore(false)} aria-label="Hide text score">
                ×
              </button>
            </div>
            <div className="sidebar-content editor-pane">
              <Editor code={code} onChange={setCode} />
            </div>
          </aside>
        )}

        <section className="chat-pane">
          <div className="chat-header">
            <div>
              <p className="chat-title">AI Composer</p>
              <p className="chat-subtitle">Draft motifs, refine phrasing, or request structural changes.</p>
            </div>
            <div className="chat-header-actions">
              <div className={`chat-status ${!isConfigured ? 'warning' : ''}`}>
                <span className="status-dot" />
                {statusLabel}
              </div>
              <button
                className="btn-toggle"
                onClick={() => setShowSettings((prev) => !prev)}
                aria-pressed={showSettings}
              >
                {showSettings ? 'Hide Settings' : 'Settings'}
              </button>
            </div>
          </div>
          {chatError && <div className="chat-alert">{chatError}</div>}
          {showSettings && (
            <ChatSettings
              provider={provider}
              onProviderChange={setProvider}
              openrouterKey={openrouterKey}
              onOpenrouterKeyChange={setOpenrouterKey}
              openrouterBaseUrl={openrouterBaseUrl}
              onOpenrouterBaseUrlChange={setOpenrouterBaseUrl}
              openrouterModel={openrouterModel}
              onOpenrouterModelChange={setOpenrouterModel}
              geminiKey={geminiKey}
              onGeminiKeyChange={setGeminiKey}
              geminiBaseUrl={geminiBaseUrl}
              onGeminiBaseUrlChange={setGeminiBaseUrl}
              geminiModel={geminiModel}
              onGeminiModelChange={setGeminiModel}
              temperature={temperature}
              onTemperatureChange={setTemperature}
              autoCheck={autoCheck}
              onAutoCheckChange={setAutoCheck}
              debugEnabled={debugEnabled}
              onDebugEnabledChange={setDebugEnabled}
            />
          )}
          {debugEnabled && (
            <DebugPanel request={debugRequest} response={debugResponse} error={debugError} />
          )}
          <div className="chat-stream" ref={chatStreamRef} onScroll={handleChatScroll}>
            {messages.map((message) => (
              <div
                key={message.id}
                className={`chat-message ${message.role}${message.kind ? ` message-${message.kind}` : ''}`}
              >
                <div className="chat-content">{renderMessageContent(message.content)}</div>
              </div>
            ))}
          </div>
          <div className="chat-composer">
            <div className="composer-row">
              <textarea
                placeholder="Describe the vibe, structure, or edits…"
                rows={2}
                value={chatInput}
                onChange={(event) => setChatInput(event.target.value)}
                onKeyDown={handleInputKeyDown}
              />
              <button
                className="btn-primary"
                disabled={!chatInput.trim() || !isConfigured || isSending || !wasmReady}
                onClick={handleSend}
              >
                {isSending ? 'Sending…' : 'Send'}
              </button>
            </div>
            <div className="composer-actions">
              <button
                className="chip"
                disabled={!isConfigured || isSending || !wasmReady}
                onClick={() => handleQuickAction('Generate 3 variations on the current score.')}
              >
                Generate variations
              </button>
              <button
                className="chip"
                disabled={!isConfigured || isSending || !wasmReady}
                onClick={() => handleQuickAction('Reharmonize the current score without changing the melody.')}
              >
                Reharmonize
              </button>
              <button
                className="chip"
                disabled={!isConfigured || isSending || !wasmReady}
                onClick={() => handleQuickAction('Humanize timing and add subtle rhythmic variation.')}
              >
                Humanize timing
              </button>
              <span className="composer-note">{compileReport}</span>
            </div>
          </div>
        </section>

        {showSheetScore && (
          <aside className="sidebar right">
            <div className="sidebar-header">
              <div>
                <h3>Sheet Music</h3>
                <span className="sidebar-meta">Live engraving</span>
              </div>
              <button className="icon-btn" onClick={() => setShowSheetScore(false)} aria-label="Hide sheet music">
                ×
              </button>
            </div>
            <div className="sidebar-content score-pane">
              <ScoreDisplay xml={xml} />
            </div>
          </aside>
        )}
      </main>
      <footer className="player-bar">
        <div className="player-info">
          <span className="player-title">Playback</span>
          <span className={`player-status ${(midiError || playbackError) ? 'error' : ''}`}>
            {midiError || playbackError || (midiUrl ? 'MIDI ready for playback.' : 'Compile to generate MIDI output.')}
          </span>
        </div>
        <div className="player-controls">
          <div className="player-buttons">
            <button className="btn-primary" onClick={startPlayback} disabled={!midiData || isPlaying}>
              {isPlaying ? 'Playing…' : 'Play'}
            </button>
            <button className="btn-secondary" onClick={() => stopPlayback(true)} disabled={!isPlaying && playbackPosition === 0}>
              Stop
            </button>
          </div>
          <div className="player-progress">
            <div className="player-progress-bar" style={{ width: `${playbackDuration ? (playbackPosition / playbackDuration) * 100 : 0}%` }} />
          </div>
          <div className="player-time">
            {formatTime(playbackPosition)} / {formatTime(playbackDuration)}
          </div>
          <button onClick={downloadMidi} className="btn-secondary">Download MIDI</button>
        </div>
      </footer>
    </div>
  )
}

export default App
