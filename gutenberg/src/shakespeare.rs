use std::io::{BufRead, BufReader};

use name::Name;

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
            if let Ok(book) = trimmed.parse::<Name>() && name::shakespeare::BOOKS.iter().any(|b| *b == book) {
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
