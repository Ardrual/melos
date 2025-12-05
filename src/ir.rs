#[derive(Debug, PartialEq, Clone)]
pub struct IrScore {
    pub tracks: Vec<IrTrack>,
    pub ppq: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IrTrack {
    pub name: String,
    pub channel: u8, // MIDI channel 0-15
    pub events: Vec<IrEvent>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IrEvent {
    pub time: u32, // Absolute ticks from start
    pub kind: IrEventKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrEventKind {
    Note {
        pitch: u8, // MIDI note number 0-127
        velocity: u8, // 0-127
        duration: u32, // Ticks
    },
    Tempo(u32), // BPM
    TimeSignature(u32, u32),
    KeySignature {
        root: String,
        scale: String,
    },
    ProgramChange(u8),
}
