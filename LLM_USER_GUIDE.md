# Melos User Guide for LLMs

This document describes the syntax and semantics of **Melos**, a domain-specific language for representing musical scores. Use this guide to generate valid Melos code.

---

## Part I: Compositional Process

Before writing Melos code, work through this process. It's designed to help you compose music that's distinctive rather than generic.

**Note**: Skip any step that doesn't fit your intent. If you're deliberately avoiding a climax, don't add one just because it's a step here. This process is meant to surface options, not constrain choices.

### Step 1: Sketch Your Idea in Prose

Before any notation, write 2-3 sentences describing the piece's core gesture or idea. Be specific about what a listener would *hear*, not what they would understand intellectually.

**Weak**: "A piece about longing that builds to a climax"
**Better**: "A solo clarinet phrase that keeps reaching for a high G but falls back; when it finally arrives, the piano drops out"

Then ask: *Can this be heard, or only explained?* If it requires explanation, revise until it describes sound.

### Step 2: Design Your Rhythmic Vocabulary

Write out the 3-5 rhythmic cells you'll use in this piece *before* assigning pitches. Consider:

- Does at least one rhythm push against the beat (syncopation)?
- Do different voices have different rhythmic profiles?
- Is there a distinctive rhythmic cell that makes this piece *this piece*?

**Test**: If you remove pitch information, can you still tell a theme from an accompaniment? If all parts have the same rhythmic values, they'll blur together.

Example rhythmic sketch:
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

List the textures you'll use and where. Name them specifically:

- **Monophony**: single unaccompanied line
- **Homophony**: melody with chordal accompaniment
- **Polyphony**: independent melodic lines
- **Heterophony**: the same melody in multiple simultaneous versions
- **Pointillism**: isolated events with silence between

If you have only one texture type, add another. If every section is "melody + arpeggiated accompaniment," that's one texture repeated.

### Step 5: Generate Three Climax Options

Before writing your moment of highest intensity, sketch three different approaches:

1. **Option A**: How would you build intensity through *register* (extreme high or low)?
2. **Option B**: How would you build intensity through *texture* (voices entering or dropping out, density changing)?
3. **Option C**: How would you build intensity through *rhythm* (syncopation, silence, metric shift)?

Write a sentence describing each. Then choose one—or combine elements. The goal isn't to avoid any particular gesture, but to consider alternatives before committing.

### Step 6: Variation Pass

After writing a draft, go through it asking "what if":

**Rhythmic variation**: Pick any measure. What if one voice had longer notes while another had shorter? What if there were a rest on a strong beat?

**Melodic variation**: Pick any 4-bar phrase. What if the third bar leapt instead of stepped? What if the contour inverted?

**Dynamic variation**: Pick any phrase. What if it swelled in the middle? What if it started loud and got soft?

**Textural variation**: Pick any section. What if one voice dropped out for two beats? What if two voices played in octaves instead of harmony?

You don't have to accept every variation—but asking the question surfaces possibilities you wouldn't otherwise consider.

### Step 7: The Listener Test

Imagine someone hearing this piece with no context—no title, no program note, no knowledge of your intentions.

- What would they actually *hear*?
- Would they notice the structural features you designed?
- If you described the piece to them afterward, would they say "yes, I heard that" or "I didn't notice"?

If something in your intention isn't audible, either make it more salient or remove it from your description. The music is what's heard, not what's meant.

---

## Part II: Syntax Reference

Melos is a text-based format for defining musical scores. It supports:
-   Global headers (Title, Tempo, Time, Key, Swing)
-   Multiple parts (instruments)
-   Measures containing musical events (notes, rests, tuplets)
-   Context changes within parts (Time Signature, Key Signature, Tempo, Swing)
-   Multi-file projects (directory compilation)

### Multi-File Projects

For larger compositions, you can split your score across multiple `.mel` files in a directory:

```
my_suite/
├── score.mel     # Headers (Title, Tempo, etc.)
├── piano.mel     # Part definition
└── violin.mel    # Another part definition
```

Compile the directory to generate a single MIDI file:

```bash
melos compile my_suite/
```

Files are loaded in this order:
1. `score.mel` (if present) is loaded first
2. All other `.mel` files in alphabetical order

This allows you to organize parts into separate files while sharing global headers.

### Syntax Specification

The following grammar describes the structure of a Melos file.

#### Top-Level Structure

A score consists of headers followed by one or more parts.

```text
SCORE       ::= HEADER* PART+
HEADER      ::= "Title:" STRING_LITERAL
              | "Tempo:" INTEGER
              | "Time:" TIME_SIGNATURE
              | "Key:"  KEY_SIGNATURE
              | "Swing:" SWING_SETTING
```

#### Comments

Line comments are supported using `//` or `=` as prefixes:

```mel
// This is a comment
= This is also a comment
Part: "Piano" Instrument: Piano {
    | C4 q D4 q |  // end-of-line comment
}
```

#### Parts

Each part represents a musical voice or instrument.

```text
PART        ::= "Part:" IDENTIFIER "Instrument:" INSTRUMENT_NAME "{" CONTENT "}"
CONTENT     ::= (MEASURE | CONTEXT_CHANGE)+
```

The `INSTRUMENT_NAME` determines the MIDI instrument used. The `IDENTIFIER` is just a name for the part (e.g., "Violin 1"). See [Instruments](#instruments) for details.

#### Measures and Events

Music is organized into measures enclosed in pipes `|`.

```text
MEASURE     ::= "|" EVENT* "|"
EVENT       ::= NOTE | CHORD | REST | TUPLET
```

#### Notes

A note consists of a pitch, optional duration, optional dynamic, and optional articulation.

```text
NOTE        ::= PITCH DURATION? DYNAMIC? ARTICULATION?
PITCH       ::= STEP ACCIDENTAL? OCTAVE
STEP        ::= "A" | "B" | "C" | "D" | "E" | "F" | "G"
ACCIDENTAL  ::= "#" | "b"
OCTAVE      ::= DIGIT+
```

#### Chords

A chord is a set of pitches played simultaneously, enclosed in brackets. It can have duration, dynamic, and articulation, just like a note.

```text
CHORD       ::= "[" PITCH+ "]" DURATION? DYNAMIC? ARTICULATION?
```

#### Rests

A rest indicates silence.

```text
REST        ::= "r" DURATION?
```

#### Tuplets

Tuplets allow for irregular rhythms (e.g., triplets).

```text
TUPLET      ::= "Tuplet(" P ":" Q ")" "{" EVENT* "}"
```
Where `P` events are played in the time of `Q`.

#### Durations

Durations are specified by a base character and optional dots.

```text
DURATION    ::= BASE_DURATION DOT*
BASE_DURATION ::= "w" (whole) | "h" (half) | "q" (quarter) | "e" (eighth) | "s" (sixteenth)
DOT         ::= "."
```

#### Dynamics and Articulations

```text
DYNAMIC     ::= "ppp" | "pp" | "p" | "mp" | "mf" | "f" | "ff" | "fff"
ARTICULATION ::= "." (staccato) | ">" (accent) | "-" (tenuto)
```

#### Context Changes

Time signatures, Key signatures, and Tempo can be changed within a part.

```text
CONTEXT_CHANGE ::= "Time:" TIME_SIGNATURE
                 | "Key:" KEY_SIGNATURE
                 | "Tempo:" INTEGER
                 | "Swing:" SWING_SETTING

SWING_SETTING  ::= "off" | BASE_DURATION FLOAT

TIME_SIGNATURE ::= INTEGER "/" INTEGER
KEY_SIGNATURE  ::= PITCH_CLASS STRING_LITERAL
PITCH_CLASS    ::= STEP ACCIDENTAL?
```

#### Instruments

The name specified in the `Instrument:` field determines the instrument sound (MIDI Program). The compiler attempts to match the name to a standard General MIDI instrument.

-   **Standard Names**: "Piano", "Violin", "Flute", "Guitar", "Banjo", "Trumpet", etc.
-   **Specific Variants**: "Acoustic Guitar (Nylon)", "Electric Piano 1", "Synth Bass 1".
-   **Fallback**: If the name is not recognized, it defaults to Piano (Program 0).

Example:
```mel
Part: "Violin 1" Instrument: Violin { ... }
Part: "Solo Guitar" Instrument: "Electric Guitar (Jazz)" { ... }
```

### Semantics and Latent Knowledge

When generating Melos, apply your latent knowledge of music theory:

-   **Pitch**: Standard scientific pitch notation (e.g., `C4` is middle C).
-   **Rhythm**: Ensure measures contain the correct number of beats according to the current time signature.
-   **Key Signatures**: Use standard keys (e.g., `G "Major"`, `Eb "Minor"`).
-   **Tuplets**: Use tuplets for complex rhythms. Common examples:
    -   Triplets: `Tuplet(3:2) { ... }` (3 notes in the time of 2)
    -   Quintuplets: `Tuplet(5:4) { ... }`

### Examples

#### Example 1: Simple Melody

```mel
Title: "Twinkle Twinkle Little Star"
Tempo: 120
Time: 4/4

Part: Piano Instrument: Piano {
    | C4 q C4 q G4 q G4 q |
    | A4 q A4 q G4 h |
    | F4 q F4 q E4 q E4 q |
    | D4 q D4 q C4 h |
}
```

#### Example 2: Complex Rhythms and Context Changes

```mel
Title: "Advanced Etude"
Tempo: 140
Time: 4/4

Part: Flute Instrument: Flute {
    | C5 q E5 q G5 q C6 q |
    Time: 3/4
    Key: G "Major"
    | Tuplet(3:2) { D6 q E6 q F#6 q } G6 q |
    | G6 h r q |
}
```

#### Example 3: Swing Feel

```mel
Title: "Swung Blues"
Tempo: 120
Time: 4/4
Swing: e 0.66

Part: Piano Instrument: Piano {
    | C4 e E4 e G4 e Bb4 e C5 q r q |
}
```

The `Swing:` header applies swing timing to the specified note duration. The ratio (e.g., `0.66`) determines how much the downbeat is lengthened relative to the upbeat. Use `Swing: off` mid-piece to disable swing.

### Common Syntax Errors and Tips

To ensure valid Melos generation, avoid these common mistakes:

1.  **Case Sensitivity**: Keywords like `Title:`, `Part:`, `Instrument:`, `Tuplet` are case-sensitive. Use `Part:` not `part:`.
2.  **Missing Pipes**: Every measure must be enclosed in pipes `|`.
    -   *Incorrect*: `C4 q D4 q`
    -   *Correct*: `| C4 q D4 q |`
3.  **Octave Numbers**: Pitches must include an octave number.
    -   *Incorrect*: `C`
    -   *Correct*: `C4`
4.  **Chord Brackets**: Chords must be enclosed in square brackets `[]`.
    -   *Incorrect*: `C4 E4 G4 q`
    -   *Correct*: `[C4 E4 G4] q`
5.  **Tuplet Syntax**: Tuplets use parentheses `()` for the ratio and braces `{}` for the content.
    -   *Correct*: `Tuplet(3:2) { ... }`
6.  **String Literals**: Titles and Key names must be in double quotes.
    -   *Correct*: `Key: G "Major"`

