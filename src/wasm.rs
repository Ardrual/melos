use wasm_bindgen::prelude::*;
use crate::parser::parse;
use crate::walker::walk;
use crate::codegen::generate;

#[wasm_bindgen]
pub fn compile_to_midi(source: &str) -> Result<Vec<u8>, JsValue> {
    let score = parse(source).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let ir = walk(&score).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let smf = generate(&ir).map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    let mut buf = Vec::new();
    smf.write(&mut buf).map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    Ok(buf)
}

#[wasm_bindgen]
pub fn compile_to_musicxml(source: &str) -> Result<String, JsValue> {
    let score = parse(source).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let ir = walk(&score).map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // TODO: Implement IR to MusicXML conversion
    let xml = ir_to_musicxml(&ir);
    Ok(xml)
}

fn ir_to_musicxml(score: &crate::ir::IrScore) -> String {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0">
  <part-list>
"#);

    for (i, track) in score.tracks.iter().enumerate() {
        if track.name == "Conductor" { continue; }
        xml.push_str(&format!(r#"    <score-part id="P{}">
      <part-name>{}</part-name>
    </score-part>
"#, i + 1, track.name));
    }

    xml.push_str("  </part-list>\n");

    for (i, track) in score.tracks.iter().enumerate() {
        if track.name == "Conductor" { continue; }
        xml.push_str(&format!(r#"  <part id="P{}">
"#, i + 1));

        let ppq = score.ppq;
        let mut notes: Vec<&crate::ir::IrEvent> = Vec::new();
        let mut time_signatures: Vec<(u32, (u32, u32))> = Vec::new();

        for event in &track.events {
            match event.kind {
                crate::ir::IrEventKind::Note { .. } => notes.push(event),
                crate::ir::IrEventKind::TimeSignature(num, den) => {
                    time_signatures.push((event.time, (num, den)));
                }
                _ => {}
            }
        }

        notes.sort_by(|a, b| a.time.cmp(&b.time));
        time_signatures.sort_by(|a, b| a.0.cmp(&b.0));
        time_signatures.dedup_by(|a, b| {
            if a.0 == b.0 {
                a.1 = b.1;
                true
            } else {
                false
            }
        });

        let mut ts_index = 0;
        let mut current_time_signature = (4, 4);
        while ts_index < time_signatures.len() && time_signatures[ts_index].0 == 0 {
            current_time_signature = time_signatures[ts_index].1;
            ts_index += 1;
        }

        let mut current_measure = 1;
        let mut measure_start_time = 0u32;
        let mut ticks_per_measure = (current_time_signature.0 * ppq * 4) / current_time_signature.1;
        let mut next_ts_time = time_signatures.get(ts_index).map(|entry| entry.0);
        let mut measure_end_time = measure_start_time + ticks_per_measure;
        if let Some(next_ts) = next_ts_time {
            if next_ts > measure_start_time && next_ts < measure_end_time {
                measure_end_time = next_ts;
            }
        }

        let mut cursor_time = measure_start_time;

        let open_measure = |xml: &mut String, measure_number: u32, ts: (u32, u32)| {
            xml.push_str(&format!(r#"    <measure number="{}">
      <attributes>
        <divisions>{}</divisions>
        <clef>
          <sign>G</sign>
          <line>2</line>
        </clef>
        <time>
          <beats>{}</beats>
          <beat-type>{}</beat-type>
        </time>
      </attributes>
"#, measure_number, ppq, ts.0, ts.1));
        };

        let close_measure = |xml: &mut String| {
            xml.push_str("    </measure>\n");
        };

        let emit_rest = |xml: &mut String, duration: u32| {
            if duration == 0 {
                return;
            }
            xml.push_str("      <note>\n");
            xml.push_str("        <rest/>\n");
            xml.push_str(&format!("        <duration>{}</duration>\n", duration));
            xml.push_str("        <voice>1</voice>\n");
            xml.push_str("      </note>\n");
        };

        open_measure(&mut xml, current_measure, current_time_signature);

        let mut index = 0;
        while index < notes.len() {
            let start_time = notes[index].time;

            while start_time >= measure_end_time {
                if cursor_time < measure_end_time {
                    emit_rest(&mut xml, measure_end_time - cursor_time);
                }
                close_measure(&mut xml);
                current_measure += 1;
                measure_start_time = measure_end_time;

                while ts_index < time_signatures.len() && time_signatures[ts_index].0 == measure_start_time {
                    current_time_signature = time_signatures[ts_index].1;
                    ts_index += 1;
                }

                ticks_per_measure = (current_time_signature.0 * ppq * 4) / current_time_signature.1;
                next_ts_time = time_signatures.get(ts_index).map(|entry| entry.0);
                measure_end_time = measure_start_time + ticks_per_measure;
                if let Some(next_ts) = next_ts_time {
                    if next_ts > measure_start_time && next_ts < measure_end_time {
                        measure_end_time = next_ts;
                    }
                }

                cursor_time = measure_start_time;
                open_measure(&mut xml, current_measure, current_time_signature);
            }

            if start_time > cursor_time {
                emit_rest(&mut xml, start_time - cursor_time);
            }

            let mut group_end_time = start_time;
            let mut chord_index = 0;
            while index < notes.len() && notes[index].time == start_time {
                if let crate::ir::IrEventKind::Note { pitch, duration, .. } = notes[index].kind {
                    let step = match pitch % 12 {
                        0 => "C", 1 => "C", 2 => "D", 3 => "D", 4 => "E",
                        5 => "F", 6 => "F", 7 => "G", 8 => "G", 9 => "A",
                        10 => "A", 11 => "B",
                        _ => "C"
                    };
                    let alter = match pitch % 12 {
                        1 | 3 | 6 | 8 | 10 => 1,
                        _ => 0
                    };
                    let octave = (pitch as i32 / 12) - 1;

                    xml.push_str("      <note>\n");
                    if chord_index > 0 {
                        xml.push_str("        <chord/>\n");
                    }
                    xml.push_str("        <pitch>\n");
                    xml.push_str(&format!("          <step>{}</step>\n", step));
                    if alter != 0 {
                        xml.push_str(&format!("          <alter>{}</alter>\n", alter));
                    }
                    xml.push_str(&format!("          <octave>{}</octave>\n", octave));
                    xml.push_str("        </pitch>\n");
                    xml.push_str(&format!("        <duration>{}</duration>\n", duration));
                    xml.push_str("        <voice>1</voice>\n");

                    let note_type = if duration >= 1920 { "whole" }
                        else if duration >= 960 { "half" }
                        else if duration >= 480 { "quarter" }
                        else if duration >= 240 { "eighth" }
                        else { "16th" };

                    xml.push_str(&format!("        <type>{}</type>\n", note_type));
                    xml.push_str("      </note>\n");

                    let note_end = start_time + duration;
                    if note_end > group_end_time {
                        group_end_time = note_end;
                    }
                }
                chord_index += 1;
                index += 1;
            }

            cursor_time = group_end_time;
        }

        if cursor_time < measure_end_time {
            emit_rest(&mut xml, measure_end_time - cursor_time);
        }

        close_measure(&mut xml);
        xml.push_str("  </part>\n");
    }

    xml.push_str("</score-partwise>\n");
    xml
}
