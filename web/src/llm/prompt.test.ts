import { describe, expect, it } from 'vitest';
import { buildSystemPrompt, extractMelosCode } from './prompt';

describe('prompt helpers', () => {
  it('extracts melos code blocks', () => {
    const content = 'Intro\n```melos\nPart: "Piano" Instrument: Piano { | C4 q | }\n```';
    expect(extractMelosCode(content)).toContain('Part: "Piano"');
  });

  it('returns null when no code block is present', () => {
    expect(extractMelosCode('No code here')).toBeNull();
  });

  it('builds a prompt with source and report', () => {
    const prompt = buildSystemPrompt('Part: "Test"', 'Compilation succeeded.');
    expect(prompt).toContain('Part: "Test"');
    expect(prompt).toContain('Compilation succeeded.');
  });
});
