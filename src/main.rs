use std::{fs::File, io::{BufReader, Read}};

#[derive(Debug)]
struct Counts {
    bytes: u32,
    characters: u32,
    lines: u32,
    words: u32,
}

fn new_counts() -> Counts {
    Counts{
        bytes: 0,
        characters: 0,
        lines: 0,
        words: 0
    }
}

fn get_counts_for(file: &File) -> Counts {
    let reader = BufReader::new(file);

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
                },
                _ => ()
            }
            
            counts.bytes += 1;
            counts.characters += 1;

            previous_character = Some(character);
        }
    }

    counts
}

fn main() {
    println!("Hello, wc!");

    let result = File::open("test_files/test.txt");
    match result {
        Ok(file) => {
            let counts = get_counts_for(&file);
            println!("{:?}", counts);
        },
        Err(_) => println!("error !!!"),
    }
}
