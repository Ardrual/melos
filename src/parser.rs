use crate::ast::*;
use crate::grammar::{MusicParser, Rule};
use anyhow::{anyhow, Result};
use pest::Parser;

pub fn parse(input: &str) -> Result<Score> {
    let mut pairs = MusicParser::parse(Rule::score, input)?;
    let score_pair = pairs.next().ok_or_else(|| anyhow!("No score found"))?;

    let mut headers = Vec::new();
    let mut parts = Vec::new();

    for pair in score_pair.into_inner() {
        match pair.as_rule() {
            Rule::header => {
                let inner = pair.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::string_literal => {
                        // Title is the only one with string_literal in the current grammar structure for header

                        let s = inner.as_str();
                        let content = s.trim_matches('"').to_string();
                        headers.push(Header::Title(content));
                    }
                    Rule::integer => {
                        let bpm = inner.as_str().parse()?;
                        headers.push(Header::Tempo(bpm));
                    }
                    Rule::time_signature => {
                        let (num, den) = parse_time_signature(inner)?;
                        headers.push(Header::TimeSignature(num, den));
                    }
                    Rule::key_signature => {
                        let (root, scale) = parse_key_signature(inner)?;
                        headers.push(Header::KeySignature(root, scale));
                    }
                    _ => {}
                }
            }
            Rule::part => {
                parts.push(parse_part(pair)?);
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(Score { headers, parts })
}

fn parse_part(pair: pest::iterators::Pair<Rule>) -> Result<Part> {
    let mut inner = pair.into_inner();

    let name_pair = inner.next().ok_or_else(|| anyhow!("Part name missing"))?;
    let name = match name_pair.as_rule() {
        Rule::part_name => {
            let inner_name = name_pair.into_inner().next().unwrap();
            match inner_name.as_rule() {
                Rule::string_literal => inner_name.as_str().trim_matches('"').to_string(),
                Rule::bare_name => inner_name.as_str().trim().to_string(),
                _ => unreachable!("part_name should only contain string_literal or bare_name"),
            }
        }
        _ => return Err(anyhow!("Expected part name")),
    };


    let instrument_pair = inner.next().ok_or_else(|| anyhow!("Instrument name missing"))?;
    let instrument = match instrument_pair.as_rule() {
        Rule::instrument_name => {
            let inner_name = instrument_pair.into_inner().next().unwrap();
            match inner_name.as_rule() {
                Rule::string_literal => inner_name.as_str().trim_matches('"').to_string(),
                Rule::bare_name => inner_name.as_str().trim().to_string(),
                _ => unreachable!("instrument_name should only contain string_literal or bare_name"),
            }
        }
        _ => return Err(anyhow!("Expected instrument name")),
    };
    

    let content_pair = inner.next().ok_or_else(|| anyhow!("Part content missing"))?;


    let content = parse_part_content(content_pair)?;

    Ok(Part { name, instrument, content })
}

fn parse_part_content(pair: pest::iterators::Pair<Rule>) -> Result<Vec<MeasureBlock>> {
    let mut blocks = Vec::new();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::measure_block => {
                blocks.extend(parse_measure_block(inner)?);
            }
            _ => {}
        }
    }
    Ok(blocks)
}

fn parse_measure_block(pair: pest::iterators::Pair<Rule>) -> Result<Vec<MeasureBlock>> {
    let mut blocks = Vec::new();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::measure => {
                blocks.push(MeasureBlock::Measure(parse_measure(inner)?));
            }
            Rule::context_change => {
                let inner = inner.into_inner().next().unwrap();
                match inner.as_rule() {
                    Rule::integer => {
                        let bpm = inner.as_str().parse()?;
                        blocks.push(MeasureBlock::ContextChange(ContextChange::Tempo(bpm)));
                    }
                    Rule::time_signature => {
                        let (num, den) = parse_time_signature(inner)?;
                        blocks.push(MeasureBlock::ContextChange(ContextChange::TimeSignature(num, den)));
                    }
                    Rule::key_signature => {
                        let (root, scale) = parse_key_signature(inner)?;
                        blocks.push(MeasureBlock::ContextChange(ContextChange::KeySignature(root, scale)));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    Ok(blocks)
}

fn parse_measure(pair: pest::iterators::Pair<Rule>) -> Result<Measure> {
    let mut events = Vec::new();
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::music_event => {
                events.push(parse_music_event(inner)?);
            }
            _ => {}
        }
    }
    Ok(Measure { events })
}

fn parse_music_event(pair: pest::iterators::Pair<Rule>) -> Result<Event> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::note => Ok(Event::Note(parse_note(inner)?)),
        Rule::chord => Ok(parse_chord(inner)?),
        Rule::rest => Ok(Event::Rest(parse_rest(inner)?)),
        Rule::tuplet => Ok(Event::Tuplet(parse_tuplet(inner)?)),
        Rule::dynamic => Ok(Event::Dynamic(inner.as_str().to_string())),
        _ => Err(anyhow!("Unknown event type")),
    }
}

fn parse_chord(pair: pest::iterators::Pair<Rule>) -> Result<Event> {
    let inner = pair.into_inner();
    let mut pitches = Vec::new();
    let mut duration = None;
    let mut dynamic = None;
    let mut articulation = None;

    for p in inner {
        match p.as_rule() {
            Rule::pitch => pitches.push(parse_pitch(p)?),
            Rule::duration => duration = Some(parse_duration(p)?),
            Rule::dynamic => dynamic = Some(p.as_str().to_string()),
            Rule::articulation => articulation = Some(p.as_str().to_string()),
            _ => {}
        }
    }

    Ok(Event::Chord(pitches, duration, dynamic, articulation))
}

fn parse_note(pair: pest::iterators::Pair<Rule>) -> Result<Note> {
    let mut inner = pair.into_inner();
    let pitch = parse_pitch(inner.next().unwrap())?;
    let mut duration = None;
    let mut dynamic = None;
    let mut articulation = None;

    for p in inner {
        match p.as_rule() {
            Rule::duration => duration = Some(parse_duration(p)?),
            Rule::dynamic => dynamic = Some(p.as_str().to_string()),
            Rule::articulation => articulation = Some(p.as_str().to_string()),
            _ => {}
        }
    }

    Ok(Note {
        pitch,
        duration,
        dynamic,
        articulation,
    })
}

fn parse_pitch(pair: pest::iterators::Pair<Rule>) -> Result<Pitch> {
    let mut inner = pair.into_inner();
    let step_str = inner.next().unwrap().as_str();
    let step = step_str.chars().next().unwrap();
    
    let mut accidental = None;
    let mut octave = 4;

    for p in inner {
        match p.as_rule() {
            Rule::accidental => {
                accidental = match p.as_str() {
                    "#" => Some(Accidental::Sharp),
                    "b" => Some(Accidental::Flat),
                    _ => None,
                };
            }
            Rule::octave => {
                octave = p.as_str().parse()?;
            }
            _ => {}
        }
    }

    Ok(Pitch {
        step,
        accidental,
        octave,
    })
}

fn parse_duration(pair: pest::iterators::Pair<Rule>) -> Result<Duration> {
    let mut inner = pair.into_inner();
    let base_pair = inner.next().unwrap();
    let base = match base_pair.as_str() {
        "w" => BaseDuration::Whole,
        "h" => BaseDuration::Half,
        "q" => BaseDuration::Quarter,
        "e" => BaseDuration::Eighth,
        "s" => BaseDuration::Sixteenth,
        _ => return Err(anyhow!("Unknown duration base")),
    };

    let dots = inner.count() as u8;
    Ok(Duration::Base(base, dots))
}

fn parse_rest(pair: pest::iterators::Pair<Rule>) -> Result<Option<Duration>> {
    let mut inner = pair.into_inner();
    if let Some(duration_pair) = inner.next() {
        Ok(Some(parse_duration(duration_pair)?))
    } else {
        Ok(None)
    }
}

fn parse_tuplet(pair: pest::iterators::Pair<Rule>) -> Result<Tuplet> {
    let mut inner = pair.into_inner();
    let p = inner.next().unwrap().as_str().parse()?;
    let q = inner.next().unwrap().as_str().parse()?;
    
    let mut events = Vec::new();
    for event_pair in inner {
        events.push(parse_music_event(event_pair)?);
    }

    Ok(Tuplet { p, q, events })
}

fn parse_time_signature(pair: pest::iterators::Pair<Rule>) -> Result<(u32, u32)> {
    let mut inner = pair.into_inner();
    let num = inner.next().unwrap().as_str().parse()?;
    let den = inner.next().unwrap().as_str().parse()?;
    Ok((num, den))
}

fn parse_key_signature(pair: pest::iterators::Pair<Rule>) -> Result<(String, String)> {
    let mut inner = pair.into_inner();
    let pitch_class_pair = inner.next().unwrap();
    let scale_pair = inner.next().unwrap();
    
    let pitch_class = pitch_class_pair.as_str().trim().to_string();
    let scale = scale_pair.as_str().trim_matches('"').to_string();
    
    Ok((pitch_class, scale))
}
