use std::io::{BufRead, BufReader};

pub struct Reader<R> {
    r: BufReader<R>,
}

pub fn new_reader() -> Reader<&'static [u8]> {
    Reader {
        r: BufReader::new(&include_bytes!("../../gutenberg/cache/epub/100/pg100.txt")[..]),
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Play {
    pub title: String,
    pub acts: Vec<Act>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Act {
    pub number: String,
    pub scenes: Vec<Scene>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scene {
    pub number: String,
    pub description: String,
    pub dialogues: Vec<Dialogue>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Dialogue {
    Speech { speaker: String, text: Vec<String> },
    StageDirection(String),
}

pub const TITLES: &[&str] = &[
    "THE SONNETS",
    "ALL’S WELL THAT ENDS WELL",
    "THE TRAGEDY OF ANTONY AND CLEOPATRA",
    "AS YOU LIKE IT",
    "THE COMEDY OF ERRORS",
    "THE TRAGEDY OF CORIOLANUS",
    "CYMBELINE",
    "THE TRAGEDY OF HAMLET, PRINCE OF DENMARK",
    "THE FIRST PART OF KING HENRY THE FOURTH",
    "THE SECOND PART OF KING HENRY THE FOURTH",
    "THE LIFE OF KING HENRY THE FIFTH",
    "THE FIRST PART OF HENRY THE SIXTH",
    "THE SECOND PART OF KING HENRY THE SIXTH",
    "THE THIRD PART OF KING HENRY THE SIXTH",
    "KING HENRY THE EIGHTH",
    "THE LIFE AND DEATH OF KING JOHN",
    "THE TRAGEDY OF JULIUS CAESAR",
    "THE TRAGEDY OF KING LEAR",
    "LOVE’S LABOUR’S LOST",
    "THE TRAGEDY OF MACBETH",
    "MEASURE FOR MEASURE",
    "THE MERCHANT OF VENICE",
    "THE MERRY WIVES OF WINDSOR",
    "A MIDSUMMER NIGHT’S DREAM",
    "MUCH ADO ABOUT NOTHING",
    "THE TRAGEDY OF OTHELLO, THE MOOR OF VENICE",
    "PERICLES, PRINCE OF TYRE",
    "KING RICHARD THE SECOND",
    "KING RICHARD THE THIRD",
    "THE TRAGEDY OF ROMEO AND JULIET",
    "THE TAMING OF THE SHREW",
    "THE TEMPEST",
    "THE LIFE OF TIMON OF ATHENS",
    "THE TRAGEDY OF TITUS ANDRONICUS",
    "TROILUS AND CRESSIDA",
    "TWELFTH NIGHT; OR, WHAT YOU WILL",
    "THE TWO GENTLEMEN OF VERONA",
    "THE TWO NOBLE KINSMEN",
    "THE WINTER’S TALE",
    "A LOVER’S COMPLAINT",
    "THE PASSIONATE PILGRIM",
    "THE PHOENIX AND THE TURTLE",
    "THE RAPE OF LUCRECE",
    "VENUS AND ADONIS",
];

impl<R: std::io::Read> Reader<R> {
    pub fn read_until_sonnets(self) -> Vec<Vec<String>> {
        let mut lines = self.r.lines()
            .skip_while(|l| l.as_ref().map_or(true, |s| !s.contains("THE SONNETS")))
            .skip(1)
            .skip_while(|l| l.as_ref().map_or(true, |s| !s.contains("THE SONNETS")))
            .skip(1);

        (0..154)
            .map(|_| {
                lines.by_ref()
                    .take(18)
                    .map(|l| l.map(|s| s.trim().to_string()))
                    .filter(|l| l.as_ref().map_or(true, |s| !s.is_empty()))
                    .skip(1)
                    .collect::<Result<Vec<String>, _>>()
            })
            .collect::<Result<_, _>>().unwrap()
    }

    pub fn read_play(self, title: &str) -> Play {
        let mut lines = self.r.lines()
            .map_while(Result::ok)
            .skip_while(|l| !l.trim().to_uppercase().contains(&title.to_uppercase()))
            .skip(1)
            .skip_while(|l| !l.trim().to_uppercase().contains(&title.to_uppercase()))
            .skip(1)
            .skip_while(|l| !l.trim().contains("Dramatis Personæ"))
            .skip(1);

        let mut play = Play {
            title: title.to_string(),
            acts: Vec::new(),
        };

        let mut current_act: Option<Act> = None;
        let mut current_scene: Option<Scene> = None;
        let mut current_speaker: Option<String> = None;
        let mut current_speech: Vec<String> = Vec::new();

        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            // Check for end of play (next play's title)
            if TITLES.iter().any(|t| t == &trimmed && *t != title.to_uppercase()) {
                break;
            }

            if trimmed.starts_with("ACT ") {
                if let Some(mut s) = current_scene.take() {
                    if let Some(speaker) = current_speaker.take() {
                        s.dialogues.push(Dialogue::Speech { speaker, text: current_speech.drain(..).collect() });
                    }
                    if let Some(mut a) = current_act.take() {
                        a.scenes.push(s);
                        play.acts.push(a);
                    }
                } else if let Some(a) = current_act.take() {
                    play.acts.push(a);
                }
                current_act = Some(Act {
                    number: trimmed.to_string(),
                    scenes: Vec::new(),
                });
                continue;
            }

            if trimmed.starts_with("SCENE ") {
                if let Some(mut s) = current_scene.take() {
                    if let Some(speaker) = current_speaker.take() {
                        s.dialogues.push(Dialogue::Speech { speaker, text: current_speech.drain(..).collect() });
                    }
                    if let Some(a) = current_act.as_mut() {
                        a.scenes.push(s);
                    }
                }
                let parts: Vec<&str> = trimmed.splitn(2, '.').collect();
                current_scene = Some(Scene {
                    number: parts[0].to_string(),
                    description: parts.get(1).unwrap_or(&"").trim().to_string(),
                    dialogues: Vec::new(),
                });
                continue;
            }

            if trimmed.starts_with("[_") && trimmed.ends_with("_]") {
                if let Some(s) = current_scene.as_mut() {
                    if let Some(speaker) = current_speaker.take() {
                        s.dialogues.push(Dialogue::Speech { speaker, text: current_speech.drain(..).collect() });
                    }
                    s.dialogues.push(Dialogue::StageDirection(trimmed.to_string()));
                }
                continue;
            }

            // Detect speaker (ALL CAPS followed by period)
            if trimmed.chars().all(|c| c.is_uppercase() || c == ' ' || c == '\'' || c == '.' || c == '-') && trimmed.ends_with('.') && trimmed.len() > 1 {
                if let Some(s) = current_scene.as_mut() {
                    if let Some(speaker) = current_speaker.take() {
                        s.dialogues.push(Dialogue::Speech { speaker, text: current_speech.drain(..).collect() });
                    }
                    current_speaker = Some(trimmed.trim_end_matches('.').to_string());
                }
                continue;
            }

            // Append to current speech or skip if no context
            if let Some(_speaker) = &current_speaker {
                current_speech.push(trimmed.to_string());
            }
        }

        // Push final act/scene if any
        if let Some(mut s) = current_scene.take() {
            if let Some(speaker) = current_speaker.take() {
                s.dialogues.push(Dialogue::Speech { speaker, text: current_speech.drain(..).collect() });
            }
            if let Some(mut a) = current_act.take() {
                a.scenes.push(s);
                play.acts.push(a);
            }
        } else if let Some(a) = current_act.take() {
            play.acts.push(a);
        }

        play
    }
}
