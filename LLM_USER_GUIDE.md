# MusicDSL User Guide for LLMs

This document describes the syntax and semantics of MusicDSL, a domain-specific language for representing musical scores. Use this guide to generate valid MusicDSL code.

## 1. Overview

MusicDSL is a text-based format for defining musical scores. It supports:
-   Global headers (Title, Tempo, Time Signature)
-   Multiple parts (instruments)
-   Measures containing musical events (notes, rests, tuplets)
-   Context changes within parts (Time Signature, Key Signature)

## 2. Syntax Specification

The following grammar describes the structure of a MusicDSL file.

### 2.1. Top-Level Structure

A score consists of headers followed by one or more parts.

```text
SCORE       ::= HEADER* PART+
HEADER      ::= "Title:" STRING_LITERAL
              | "Tempo:" INTEGER
              | "Time:" TIME_SIGNATURE
```

### 2.2. Parts

Each part represents a musical voice or instrument.

```text
```text
PART        ::= "Part:" IDENTIFIER "Instrument:" INSTRUMENT_NAME "{" CONTENT "}"
CONTENT     ::= (MEASURE | CONTEXT_CHANGE)+
```

The `INSTRUMENT_NAME` determines the MIDI instrument used. The `IDENTIFIER` is just a name for the part (e.g., "Violin 1"). See [Instruments](#211-instruments) for details.

### 2.3. Measures and Events

Music is organized into measures enclosed in pipes `|`.

```text
MEASURE     ::= "|" EVENT* "|"
EVENT       ::= NOTE | CHORD | REST | TUPLET
```

### 2.4. Notes

A note consists of a pitch, optional duration, optional dynamic, and optional articulation.

```text
NOTE        ::= PITCH DURATION? DYNAMIC? ARTICULATION?
PITCH       ::= STEP ACCIDENTAL? OCTAVE
STEP        ::= "A" | "B" | "C" | "D" | "E" | "F" | "G"
ACCIDENTAL  ::= "#" | "b"
OCTAVE      ::= DIGIT+
```

### 2.5. Chords

A chord is a set of pitches played simultaneously, enclosed in brackets. It can have duration, dynamic, and articulation, just like a note.

```text
CHORD       ::= "[" PITCH+ "]" DURATION? DYNAMIC? ARTICULATION?
```

### 2.6. Rests

A rest indicates silence.

```text
REST        ::= "r" DURATION?
```

### 2.7. Tuplets

Tuplets allow for irregular rhythms (e.g., triplets).

```text
TUPLET      ::= "Tuplet(" P ":" Q ")" "{" EVENT* "}"
```
Where `P` events are played in the time of `Q`.

### 2.8. Durations

Durations are specified by a base character and optional dots.

```text
DURATION    ::= BASE_DURATION DOT*
BASE_DURATION ::= "w" (whole) | "h" (half) | "q" (quarter) | "e" (eighth) | "s" (sixteenth)
DOT         ::= "."
```

### 2.9. Dynamics and Articulations

```text
DYNAMIC     ::= "ppp" | "pp" | "p" | "mp" | "mf" | "f" | "ff" | "fff"
ARTICULATION ::= "." (staccato) | ">" (accent) | "-" (tenuto)
```

### 2.10. Context Changes

Time and Key signatures can be changed within a part.

```text
CONTEXT_CHANGE ::= "Time:" TIME_SIGNATURE
                 | "Key:" KEY_SIGNATURE

TIME_SIGNATURE ::= INTEGER "/" INTEGER
KEY_SIGNATURE  ::= PITCH_CLASS STRING_LITERAL
PITCH_CLASS    ::= STEP ACCIDENTAL?
```

### 2.11. Instruments

The name specified in the `Instrument:` field determines the instrument sound (MIDI Program). The compiler attempts to match the name to a standard General MIDI instrument.

-   **Standard Names**: "Piano", "Violin", "Flute", "Guitar", "Banjo", "Trumpet", etc.
-   **Specific Variants**: "Acoustic Guitar (Nylon)", "Electric Piano 1", "Synth Bass 1".
-   **Fallback**: If the name is not recognized, it defaults to Piano (Program 0).

Example:
```dsl
Part: "Violin 1" Instrument: Violin { ... }
Part: "Solo Guitar" Instrument: "Electric Guitar (Jazz)" { ... }
```

## 3. Semantics and Latent Knowledge

When generating MusicDSL, apply your latent knowledge of music theory:

-   **Pitch**: Standard scientific pitch notation (e.g., `C4` is middle C).
-   **Rhythm**: Ensure measures contain the correct number of beats according to the current time signature.
-   **Key Signatures**: Use standard keys (e.g., `G "Major"`, `Eb "Minor"`).
-   **Tuplets**: Use tuplets for complex rhythms. Common examples:
    -   Triplets: `Tuplet(3:2) { ... }` (3 notes in the time of 2)
    -   Quintuplets: `Tuplet(5:4) { ... }`

## 4. Examples

### Example 1: Simple Melody

```dsl
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

### Example 2: Complex Rhythms and Context Changes

```dsl
Title: "Advanced Etude"
Tempo: 140
Time: 4/4

Part: Flute Instrument: Flute {
    | C5 q E5 q G5 q C6 q |
    Time: 3/4
    Key: G "Major"
    | Tuplet(3:2) { D6 q E6 q F#6 q } |
    | G6 h r q |
}
```

## 5. Common Syntax Errors and Tips

To ensure valid MusicDSL generation, avoid these common mistakes:

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

