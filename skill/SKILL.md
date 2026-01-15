---
name: compose
description: Compose a piece of music in the Melos language, compile it to MIDI, and fix any errors. Use when asked to compose, write, or create music.
---

# Melos Composer

> **Early Feature**: This skill is an early-stage portable AI skill for music composition. The bundled `melos` binary may not include all features from the main Melos compiler.

Compose music in the Melos domain-specific language, compile to MIDI, and iteratively fix errors.

## Workflow

1. **Gather requirements** - Determine the user's intent:
   - Style/genre (classical, jazz, pop, etc.)
   - Mood/character
   - Instrumentation
   - Length (number of measures)
   - Tempo and time signature preferences
   - Any specific musical ideas or themes

2. **Plan the composition** - Before writing code, work through the compositional process (see below)

3. **Compose the piece** - Write a `.mel` file in the current directory

4. **Compile and validate** - Use the included Melos compiler:
   ```bash
   ./melos compile <filename>.mel
   ```

   This produces a `.mid` file with the same base name.

5. **Fix errors** - If compilation fails:
   - Read the error message carefully
   - Common issues: measure duration mismatches, missing octaves, syntax errors
   - Fix the `.mel` file and recompile
   - Repeat until successful

6. **Finalize** - Once compiled successfully, inform the user that the MIDI file is ready.

## Compositional Process

Before writing Melos code, work through these steps. They help produce music that's distinctive rather than generic.

**Note**: Skip any step that doesn't fit your intent. If you're deliberately avoiding a climax, don't add one just because it's a step here. This process surfaces options, not constraints.

### Step 1: Sketch Your Idea in Prose

Write 2-3 sentences describing the piece's core gesture. Be specific about what a listener would *hear*, not what they would understand intellectually.

**Weak**: "A piece about longing that builds to a climax"
**Better**: "A solo clarinet phrase that keeps reaching for a high G but falls back; when it finally arrives, the piano drops out"

Ask: *Can this be heard, or only explained?* If it requires explanation, revise until it describes sound.

### Step 2: Design Your Rhythmic Vocabulary

Write out the 3-5 rhythmic cells you'll use *before* assigning pitches:

- Does at least one rhythm push against the beat (syncopation)?
- Do different voices have different rhythmic profiles?
- Is there a distinctive rhythmic cell that makes this piece *this piece*?

**Test**: If you remove pitch information, can you still tell a theme from an accompaniment? If all parts have the same rhythmic values, they'll blur together.

Example:
```
Theme rhythm: long-short-short-LONG (half, eighth, eighth, half tied)
Accompaniment: offbeat pulses (rest-eighth, chord-quarter, rest-eighth, chord-quarter)
Countermelody: running eighths BUT with rests on strong beats
```

### Step 3: Design Your Theme's Contour

Before writing pitches, sketch the *shape*:

- Where is the highest note? Where is the lowest?
- Is there a distinctive interval (a leap of a sixth, a tritone, a repeated fourth)?
- Can you hum the shape without pitch names and recognize it?

**Test**: Write the theme. Cover the page. Can you remember how it goes? If not, ask: what would make it stick? A wider leap? A distinctive rhythm? An unexpected direction?

### Step 4: Plan Textural Contrast

List the textures you'll use and where:

- **Monophony**: single unaccompanied line
- **Homophony**: melody with chordal accompaniment
- **Polyphony**: independent melodic lines
- **Heterophony**: the same melody in multiple simultaneous versions
- **Pointillism**: isolated events with silence between

If you have only one texture type, add another.

### Step 5: Generate Three Climax Options

Before writing your moment of highest intensity, sketch three approaches:

1. **Option A**: How would you build intensity through *register* (extreme high or low)?
2. **Option B**: How would you build intensity through *texture* (voices entering or dropping out)?
3. **Option C**: How would you build intensity through *rhythm* (syncopation, silence, metric shift)?

Write a sentence for each. Then choose one—or combine elements. The goal isn't to avoid any particular gesture, but to consider alternatives before committing.

### Step 6: Variation Pass

After drafting, ask "what if":

- **Rhythmic**: What if one voice had longer notes while another had shorter?
- **Melodic**: What if the third bar leapt instead of stepped?
- **Dynamic**: What if the phrase swelled in the middle?
- **Textural**: What if one voice dropped out for two beats?

You don't have to accept every variation—but asking surfaces possibilities you wouldn't otherwise consider.

### Step 7: The Listener Test

Imagine someone hearing this piece with no context—no title, no program note.

- What would they actually *hear*?
- Would they notice the structural features you designed?

If something in your intention isn't audible, either make it more salient or remove it from your description.

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
for f in work_name/*.mel; do ./melos compile "$f"; done
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
Swing: e 0.66   // Apply swing to eighth notes
Swing: off      // Disable swing
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

### Multiple Part blocks with same instrument
Each instrument needs exactly ONE Part block containing all its measures. Multiple Part blocks with the same instrument play simultaneously as separate tracks—they do NOT concatenate.

Wrong (creates 3 simultaneous cello tracks):
```mel
Part: "Cello Intro" Instrument: Cello { | D3 w | }
Part: "Cello Theme" Instrument: Cello { | A3 w | }
Part: "Cello Coda" Instrument: Cello { | D2 w | }
```

Correct (one continuous part):
```mel
Part: "Cello" Instrument: Cello {
    // Intro
    | D3 w |
    // Theme
    | A3 w |
    // Coda
    | D2 w |
}
```

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
