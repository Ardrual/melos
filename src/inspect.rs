use anyhow::{Context, Result};
use midly::{Smf, TrackEventKind, MetaMessage, MidiMessage};
use std::path::Path;
use std::fs;

pub fn inspect(path: &Path) -> Result<()> {
    let data = fs::read(path).with_context(|| format!("Failed to read MIDI file: {:?}", path))?;
    let smf = Smf::parse(&data).context("Failed to parse MIDI file")?;

    println!("Format: {:?}", smf.header.format);
    println!("Timing: {:?}", smf.header.timing);
    println!("Tracks: {}", smf.tracks.len());

    for (i, track) in smf.tracks.iter().enumerate() {
        println!("\nTrack {}:", i);
        let mut absolute_time = 0;
        let mut note_count = 0;
        let mut event_count = 0;

        for event in track {
            absolute_time += event.delta.as_int();
            event_count += 1;
            match event.kind {
                TrackEventKind::Meta(MetaMessage::Tempo(mpq)) => {
                    let bpm = 60_000_000.0 / mpq.as_int() as f64;
                    println!("  [@{}] Tempo: {:.2} BPM ({} mpq)", absolute_time, bpm, mpq.as_int());
                }
                TrackEventKind::Meta(MetaMessage::TimeSignature(num, den, _, _)) => {
                     let den_val = 2u32.pow(den as u32);
                     println!("  [@{}] Time Signature: {}/{}", absolute_time, num, den_val);
                }
                TrackEventKind::Meta(MetaMessage::KeySignature(key, scale)) => {
                    println!("  [@{}] Key Signature: {:?} {:?}", absolute_time, key, scale);
                }
                TrackEventKind::Meta(MetaMessage::TrackName(name)) => {
                     if let Ok(s) = std::str::from_utf8(name) {
                         println!("  [@{}] Track Name: {}", absolute_time, s);
                     }
                }
                TrackEventKind::Midi { message: MidiMessage::NoteOn { .. }, .. } => {
                    note_count += 1;
                }
                _ => {}
            }
        }
        println!("  Total Duration: {} ticks", absolute_time);
        println!("  Total Events: {}", event_count);
        println!("  Note On Events: {}", note_count);
    }

    Ok(())
}
