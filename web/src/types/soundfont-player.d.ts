declare module 'soundfont-player' {
  export interface InstrumentOptions {
    format?: 'mp3' | 'ogg';
    soundfont?: string;
  }

  export interface PlayOptions {
    duration?: number;
    gain?: number;
  }

  export interface Instrument {
    play: (note: number, time?: number, options?: PlayOptions) => { stop?: () => void } | void;
  }

  export function instrument(
    context: AudioContext,
    name: string,
    options?: InstrumentOptions
  ): Promise<Instrument>;

  const Soundfont: {
    instrument: typeof instrument;
  };

  export default Soundfont;
}
