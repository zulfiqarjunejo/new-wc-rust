pub mod printer;
pub mod counter;

use clap::Parser;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

use crate::counter::get_counts_for;
use crate::printer::Printer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, default_value_t = false)]
    c: bool,

    #[arg(short, default_value_t = false)]
    m: bool,

    #[arg(short, default_value_t = false)]
    l: bool,

    #[arg(short, default_value_t = false)]
    w: bool,

    // TODO: Understand this syntax
    #[arg(required = false, value_parser = clap::value_parser!(String))]
    file_name: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut file_name = String::from("");

    let reader: Box<dyn Read> = if let Some(f) = args.file_name {
        file_name = f.clone();
        let result = File::open(f);

        let reader = match result {
            Ok(file) => BufReader::new(file),
            Err(_) => todo!(),
        };

        Box::new(reader)
    } else {
        let reader = BufReader::new(io::stdin());
        Box::new(reader)
    };

    let printer = Printer{
            should_print_bytes: args.m,
            should_print_characters: args.c,
            should_print_lines: args.l,
            should_print_words: args.w
    };

    let counts = get_counts_for(reader);

    printer.print_counts(counts, file_name);
}
