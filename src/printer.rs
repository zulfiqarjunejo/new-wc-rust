use crate::counter::Counts;

pub struct Printer {
    pub should_print_bytes: bool,
    pub should_print_characters: bool,
    pub should_print_lines: bool,
    pub should_print_words: bool,
}

impl Printer {
    pub fn print_counts(self: &Self, counts: Counts, file_name: String) {
        if self.should_print_bytes {
            println!("{} {}", counts.bytes, file_name)
        } else if self.should_print_characters {
            println!("{} {}", counts.characters, file_name)
        } else if self.should_print_lines {
            println!("{} {}", counts.lines, file_name)
        } else if self.should_print_words {
            println!("{} {}", counts.words, file_name)
        }
    }
}
