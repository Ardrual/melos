#[derive(Debug, PartialEq, Clone)]
pub struct Score {
    pub headers: Vec<Header>,
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Header {
    Title(String),
    Tempo(u32),
    TimeSignature(u32, u32),
    KeySignature(String, String),
    Swing(Option<(BaseDuration, f64)>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Part {
    pub name: String,
    pub instrument: String,
    pub content: Vec<MeasureBlock>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum MeasureBlock {
    Measure(Measure),
    ContextChange(ContextChange),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Measure {
    pub events: Vec<Event>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ContextChange {
    Tempo(u32),
    TimeSignature(u32, u32),
    KeySignature(String, String),
    Swing(Option<(BaseDuration, f64)>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Event {
    Note(Note),
    Chord(Vec<Pitch>, Option<Duration>, Option<String>, Option<String>),
    Rest(Option<Duration>),
    Tie,
    Tuplet(Tuplet),
    Dynamic(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Note {
    pub pitch: Pitch,
    pub duration: Option<Duration>,
    pub dynamic: Option<String>,
    pub articulation: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Pitch {
    pub step: char,
    pub accidental: Option<Accidental>,
    pub octave: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Accidental {
    Sharp,
    Flat,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tuplet {
    pub p: u32,
    pub q: u32,
    pub events: Vec<Event>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Duration {
    Base(BaseDuration, u8), // u8 is number of dots
    Subdivision(BaseDuration, u32),
    Fraction(u32, u32),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BaseDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
}
