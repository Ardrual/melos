import { describe, expect, it } from 'vitest';
import { buildGeminiEndpoint } from './providers';

describe('provider helpers', () => {
  it('builds the Gemini endpoint without double slashes', () => {
    const endpoint = buildGeminiEndpoint('https://example.com/v1beta/models/', 'gemini-2.5-flash');
    expect(endpoint).toBe('https://example.com/v1beta/models/gemini-2.5-flash:generateContent');
  });

  it('builds the Gemini endpoint when base has no trailing slash', () => {
    const endpoint = buildGeminiEndpoint('https://example.com/v1beta/models', 'gemini-2.5-pro');
    expect(endpoint).toBe('https://example.com/v1beta/models/gemini-2.5-pro:generateContent');
  });
});
