use std::io::Read;

pub struct Counts {
    pub bytes: u32,
    pub characters: u32,
    pub lines: u32,
    pub words: u32,
}

pub fn new_counts() -> Counts {
    Counts {
        bytes: 0,
        characters: 0,
        lines: 0,
        words: 0,
    }
}

pub fn get_counts_for(reader: Box<dyn Read>) -> Counts {
    let mut counts = new_counts();

    let mut previous_character: Option<char> = None;
    for byte in reader.bytes() {
        if let Ok(b) = byte {
            let character = b as char;

            match character {
                '\n' => counts.lines += 1,
                character if character.is_whitespace() => {
                    if let Some(pc) = previous_character {
                        if !pc.is_whitespace() {
                            counts.words += 1;
                        }
                    }
                }
                _ => (),
            }

            counts.bytes += 1;
            counts.characters += 1;

            previous_character = Some(character);
        }
    }

    counts
}