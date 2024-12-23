use crate::cli::CLI;

use std::io::BufRead;

#[derive(Default)]
 pub struct Output{
    pub bytes: u64,
    lines: u64,
    words: u64,
    chars: u64,
    file:Option<String>,
}
 impl Output {
    pub fn new(file: &str) -> Self {
        let mut out = Self::default();
        out.file = Some(String::from(file));

        out
    }
}
pub fn process_lines<T: BufRead>(reader: T, from_stdin: bool, out: &mut Output) {
    for line in reader.lines() {
        match line {
            Ok(input) => {
                out.lines += 1;

                if from_stdin {
                    out.bytes += input.as_bytes().len() as u64;
                }

                let line_words: Vec<_> = input.split_terminator(" ").collect();
                out.words += line_words.len() as u64;

                line_words.iter().for_each(|w| out.chars += w.len() as u64);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

pub fn print_output(cli: &CLI, output: Vec<Output>){
    let mut print_all = false;
    let mut total = Output::default();

    for out in &output {
        if !cli.lines && !cli.words && !cli.chars && !cli.bytes {
            print_all = true;
            if let Some(f) = &out.file {
                print!("\t{}\t{}\t{}\t{}\n", out.lines, out.words, out.bytes, f);
            } else {
                print!("\t{}\t{}\t{}\n", out.lines, out.words, out.bytes);
            }
        } else {
            if cli.lines {
                print!("\t{}", out.lines);
            }
            if cli.words {
                print!("\t{}", out.words);
            }
            if cli.bytes {
                print!("\t{}", out.bytes);
            } else if cli.chars {
                print!("\t{}", out.chars);
            }
            if let Some(f) = &out.file {
                print!("\t{}\n", f);
            } else {
                println!();
            }
        }
    
        total.lines += out.lines;
        total.words += out.words;
        total.bytes += out.bytes;
        total.chars += out.chars;
    }

    if output.len() > 1 {
        if print_all {
            print!("\t{}\t{}\t{}\t{}\n", total.lines, total.words, total.bytes, "total");
        } else {
            if cli.lines {
                print!("\t{}", total.lines);
            }
            if cli.words {
                print!("\t{}", total.words);
            }
            if cli.bytes {
                print!("\t{}", total.bytes);
            } else if cli.chars {
                print!("\t{}", total.chars);
            }
            
            println!("\ttotal");
        }
    }
}