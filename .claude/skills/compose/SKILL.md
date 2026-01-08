---
name: compose
description: Compose a piece of music in the Melos language, compile it to MIDI, and fix any errors. Use when asked to compose, write, or create music.
---

# Melos Composer

Compose music in the Melos domain-specific language, compile to MIDI, and iteratively fix errors.

**This skill includes a bundled Melos compiler binary (`melos`) for macOS x86_64 (Intel).**

## Workflow

1. **Gather requirements** - Ask the user about:
   - Style/genre (classical, jazz, pop, etc.)
   - Mood/character
   - Instrumentation
   - Length (number of measures)
   - Tempo and time signature preferences
   - Any specific musical ideas or themes

2. **Compose the piece** - Write a `.mel` file in the current directory

3. **Compile and validate** - Use the bundled compiler:
   ```bash
   .claude/skills/compose/melos compile <filename>.mel
   ```

   This produces a `.mid` file with the same base name.

4. **Fix errors** - If compilation fails:
   - Read the error message carefully
   - Common issues: measure duration mismatches, missing octaves, syntax errors
   - Fix the `.mel` file and recompile
   - Repeat until successful

5. **Offer playback** - Once compiled successfully, let the user know the MIDI file is ready

## Multi-Movement Works and Program Notes

For substantial works (string quartets, symphonies, multi-movement pieces), offer to create program notes and organize files properly.

### When to offer

- User requests a "suite", "sonata", "quartet", "symphony", or other multi-movement form
- User asks for "program notes" or "liner notes"
- The scope is substantial enough to warrant documentation

### File organization

Create a directory named after the work, with movements numbered using Roman numerals:

```
work_name/
├── PROGRAM_NOTES.md
├── I_first_movement.mel
├── I_first_movement.mid
├── II_second_movement.mel
├── II_second_movement.mid
└── ...
```

### Program notes

Program notes should be substantial, musicologically informed, and written with a distinct voice. They might include:

- Historical or conceptual context
- Discussion of form, themes, and compositional techniques
- Movement-by-movement commentary
- Performance considerations
- Personal or philosophical reflections on the music

The structure and tone should emerge organically from the work itself. Some pieces call for analytical rigor; others for poetic reflection; others for historical narrative. Trust your creative instincts.

### Compile all movements

```bash
for f in work_name/*.mel; do .claude/skills/compose/melos compile "$f"; done
```

## Melos Syntax Quick Reference

### Structure
```mel
Title: "Piece Name"
Tempo: 120
Time: 4/4
Key: C "Major"

Part: "Part Name" Instrument: Piano {
    | C4 q D4 q E4 q F4 q |
    | G4 h E4 h |
}
```

### Durations
- `w` = whole note (4 beats in 4/4)
- `h` = half note (2 beats)
- `q` = quarter note (1 beat)
- `e` = eighth note (1/2 beat)
- `s` = sixteenth note (1/4 beat)
- Add `.` for dotted: `h.` = 3 beats, `q.` = 1.5 beats

### Pitches
- Note name + optional accidental + octave: `C4`, `F#5`, `Bb3`
- Middle C = `C4`
- Rests: `r q` (quarter rest), `r h` (half rest), etc.

### Chords
```mel
[C4 E4 G4] q    // C major chord, quarter note
[D4 F#4 A4] h   // D major chord, half note
```

### Dynamics
`ppp`, `pp`, `p`, `mp`, `mf`, `f`, `ff`, `fff`
```mel
C4 q mf    // quarter note C4 at mezzo-forte
```

### Articulations
- `.` = staccato
- `>` = accent
- `-` = tenuto
```mel
C4 q .     // staccato quarter note
E4 q >     // accented quarter note
```

### Tuplets
```mel
Tuplet(3:2) { E4 q E4 q E4 q }   // triplet: 3 quarters in time of 2
```

### Context Changes (mid-piece)
```mel
Time: 3/4
Key: G "Major"
Tempo: 140
```

### Common Instruments
Piano, Violin, Viola, Cello, Flute, Clarinet, Oboe, Bassoon, Trumpet,
French Horn, Trombone, Tuba, Acoustic Guitar (Nylon), Electric Guitar (Jazz),
Acoustic Bass, Electric Bass, Strings, Choir Aahs, Synth Lead

## Common Errors and Fixes

### Measure duration mismatch
Error: "Measure X has Y ticks but expected Z"
- Count the beats in each measure
- In 4/4: must equal 4 quarter notes (w, h+h, q+q+q+q, etc.)
- In 3/4: must equal 3 quarter notes

### Missing octave
Error: "Expected octave number"
- Every pitch needs an octave: `C4` not `C`

### Tuplet ratio issues
- Tuplet content must match the ratio
- `Tuplet(3:2)` needs exactly 3 events inside

### String literal issues
- Titles and key quality need quotes: `Key: G "Major"`
- Part names need quotes: `Part: "Violin 1"`

## Example Composition

```mel
Title: "Morning Walk"
Tempo: 100
Time: 4/4
Key: G "Major"

Part: "Piano" Instrument: Piano {
    | G4 q B4 q D5 q B4 q |
    | A4 q C5 q E5 q C5 q |
    | G4 h D5 h |
    | G4 w |
}
```
