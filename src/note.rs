use midi_msg;
use std::fmt::Display;

#[derive(Clone, Debug, Copy, serde::Serialize, serde::Deserialize)]
enum NoteLetter {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}
#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
enum Accidental {
    Flat = -1,
    Neutral = 0,
    Sharp = 1,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Note {
    letter: NoteLetter,
    accidental: Accidental,
    octave: u8,
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note = match self.letter {
            NoteLetter::C => 'C',
            NoteLetter::D => 'D',
            NoteLetter::E => 'E',
            NoteLetter::F => 'F',
            NoteLetter::G => 'G',
            NoteLetter::A => 'A',
            NoteLetter::B => 'B',
        };
        write!(f, "{}", note)?;
        match self.accidental {
            Accidental::Flat => write!(f, "b")?,
            Accidental::Neutral => (),
            Accidental::Sharp => write!(f, "#")?,
        };
        write!(f, "{}", self.octave)?;
        Ok(())
    }
}
impl std::fmt::Debug for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let note = match self.letter {
            NoteLetter::C => 'C',
            NoteLetter::D => 'D',
            NoteLetter::E => 'E',
            NoteLetter::F => 'F',
            NoteLetter::G => 'G',
            NoteLetter::A => 'A',
            NoteLetter::B => 'B',
        };
        write!(f, "{}", note)?;
        match self.accidental {
            Accidental::Flat => write!(f, "b")?,
            Accidental::Neutral => (),
            Accidental::Sharp => write!(f, "#")?,
        };
        write!(f, "{}", self.octave)?;
        Ok(())
    }
}

impl Note {
    fn to_midi_number(&self) -> u8 {
        24 + self.letter as u8 + self.octave * 12
    }
    fn to_midi_on(&self) -> Vec<u8> {
        midi_msg::MidiMsg::ChannelVoice {
            channel: midi_msg::Channel::Ch1,
            msg: midi_msg::ChannelVoiceMsg::NoteOn {
                note: self.to_midi_number(),
                velocity: 60,
            },
        }
        .to_midi()
    }
    fn to_midi_off(&self) -> Vec<u8> {
        midi_msg::MidiMsg::ChannelVoice {
            channel: midi_msg::Channel::Ch1,
            msg: midi_msg::ChannelVoiceMsg::NoteOff {
                note: self.to_midi_number(),
                velocity: 0,
            },
        }
        .to_midi()
    }
}
