pub mod output;
// use output::Output;
use clap::Parser;

#[derive(Parser)]
#[command(name = "wc-tool")]
#[command(version = "0.1.0")]
#[command(about="The wc-tool utility displays the count of lines, words, characters, and bytes contained in each input file", long_about=None)]
pub struct CLI {
    /// The number of bytes in each input file is written to the
    /// standard output. This will cancel out any prior usage of the
    /// -m option.
    #[arg(short = 'c')]
    bytes: bool,

    /// The number of lines in each input file is written to the
    /// standard output.
    #[arg(short)]
    lines: bool,

    /// The number of words in each input file is written to the
    /// standard output.
    #[arg(short)]
    words: bool,

    /// The number of characters in each input file is written to the
    /// standard output.
    #[arg(short = 'm')]
    chars: bool,
    pub files: Option<Vec<String>>,
}
