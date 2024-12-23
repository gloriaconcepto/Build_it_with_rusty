mod cli;

use std::{
    fs::{self, File},
    io::{self},
};
use clap::Parser;
use cli::CLI;
use cli::output;
use crate::output::Output;
use crate::output::process_lines;
use crate::output::print_output;
fn main()->io::Result<()> {
    let cli = CLI::parse();
    let mut output: Vec<output::Output> = vec![];
    if let Some(files) = &cli.files {
        for file in files.iter() {
            let f = File::open(file)?;
            let reader = io::BufReader::new(f);

            let mut out = output::Output::new(file);
            out.bytes = fs::metadata(file)?.len();

            output::process_lines(reader, false, &mut out);
            output.push(out);
        }
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let mut out = Output::default();

        process_lines(reader, true, &mut out);
        output.push(out);
    }

    print_output(&cli, output);

    Ok(())
}
