use std::fs;
use std::path::PathBuf;

use clap::Parser;

use ouroboros_dna::error::Result;
use ouroboros_dna::{Mode, TritonCore};

#[derive(Parser, Debug)]
#[command(author, version, about = "TRITON Core CLI", long_about = None)]
struct Cli {
    /// Name der TRITON-Instanz
    #[arg(long, default_value = "TRITON")]
    name: String,

    /// Betriebsmodus: auto, memory oder hybrid
    #[arg(long, default_value = "auto")]
    mode: String,

    /// Anzahl der Zyklen für Auto-Operationen
    #[arg(long, default_value_t = 40)]
    cycles: usize,

    /// Wortliste für Seed-Generierung
    #[arg(long, default_value = "data/wordlist.txt")]
    wordlist: PathBuf,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Fehler: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let wordlist = load_wordlist(&cli.wordlist)?;
    let mode: Mode = cli.mode.parse()?;

    let core = TritonCore::new(wordlist)
        .name(cli.name)
        .mode(mode)
        .cycles(cli.cycles);

    core.run()
}

fn load_wordlist(path: &PathBuf) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)?;
    let words = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    Ok(words)
}
