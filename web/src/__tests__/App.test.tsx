import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi, beforeEach } from 'vitest';
import App from '../App';
import { server, http, HttpResponse } from '../test/server';

vi.mock('../components/Editor', () => ({
  default: ({ code, onChange }: { code: string; onChange: (value: string) => void }) => (
    <textarea
      data-testid="editor"
      value={code}
      onChange={(event) => onChange(event.target.value)}
    />
  ),
}));

vi.mock('../components/ScoreDisplay', () => ({
  default: () => <div data-testid="score-display" />,
}));

vi.mock('../melos-wasm/melos', () => ({
  default: vi.fn().mockResolvedValue(undefined),
  compile_to_musicxml: vi.fn((source: string) => {
    if (source.includes('BAD')) {
      throw new Error('musicxml failed');
    }
    return '<score-partwise />';
  }),
  compile_to_midi: vi.fn((source: string) => {
    if (source.includes('BAD')) {
      throw new Error('midi failed');
    }
    return new Uint8Array([0, 1, 2]);
  }),
}));

vi.mock('@tonejs/midi', () => ({
  Midi: class {
    duration = 0;
    tracks: Array<{ notes: any[]; instrument?: { number: number } }> = [];
  },
}));

const GOOD_MELOS = `Part: "Piano" Instrument: "Acoustic Grand Piano" {
  | C4 q D4 q E4 q F4 q |
}`;

const BAD_MELOS = `Part: "Piano" Instrument: "Acoustic Grand Piano" {
  | BAD |
}`;

beforeEach(() => {
  localStorage.clear();
});

describe('AI Composer chat', () => {
  it('sends a prompt to OpenRouter and applies returned Melos', async () => {
    localStorage.setItem('melos.openrouter.key', 'test-key');
    localStorage.setItem('melos.openrouter.model', 'anthropic/claude-3.5-sonnet');
    localStorage.setItem('melos.openrouter.autoCheck', 'false');

    server.use(
      http.post('https://openrouter.ai/api/v1/chat/completions', async ({ request }) => {
        const body = await request.json();
        const lastMessage = body?.messages?.[body.messages.length - 1]?.content || '';
        expect(lastMessage).toContain('Create a short piece');
        return HttpResponse.json({
          choices: [
            {
              message: {
                content: `Here is a draft:\n\`\`\`melos\n${GOOD_MELOS}\n\`\`\``,
              },
            },
          ],
        });
      }),
    );

    render(<App />);

    const input = await screen.findByPlaceholderText('Describe the vibe, structure, or edits…');
    await userEvent.type(input, 'Create a short piece.');
    await userEvent.click(screen.getByRole('button', { name: /send/i }));

    await waitFor(() => {
      expect(screen.getByTestId('editor')).toHaveValue(GOOD_MELOS);
    });

    expect(screen.getAllByText(/Compilation succeeded/i).length).toBeGreaterThan(0);
  });

  it('auto-fixes when compilation fails', async () => {
    localStorage.setItem('melos.openrouter.key', 'test-key');
    localStorage.setItem('melos.openrouter.model', 'anthropic/claude-3.5-sonnet');

    let callCount = 0;
    server.use(
      http.post('https://openrouter.ai/api/v1/chat/completions', () => {
        callCount += 1;
        if (callCount === 1) {
          return HttpResponse.json({
            choices: [
              {
                message: {
                  content: `\`\`\`melos\n${BAD_MELOS}\n\`\`\``,
                },
              },
            ],
          });
        }
        return HttpResponse.json({
          choices: [
            {
              message: {
                content: `\`\`\`melos\n${GOOD_MELOS}\n\`\`\``,
              },
            },
          ],
        });
      }),
    );

    render(<App />);

    const input = await screen.findByPlaceholderText('Describe the vibe, structure, or edits…');
    await userEvent.type(input, 'Fix the harmony.');
    await userEvent.click(screen.getByRole('button', { name: /send/i }));

    await waitFor(() => {
      expect(callCount).toBe(2);
    });

    await waitFor(() => {
      expect(screen.getByTestId('editor')).toHaveValue(GOOD_MELOS);
    });

    expect(screen.getAllByText(/Compilation succeeded/i).length).toBeGreaterThan(0);
    expect(screen.getByText(/Auto-fix/i)).toBeInTheDocument();
  });
});
