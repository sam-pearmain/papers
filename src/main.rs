use std::path::PathBuf;

use clap::{Parser, Subcommand};
use papers::bibtex::parser::Parser as BibtexParser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands, 
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialises a .papers file for bibliography tracking
    Init,
    /// Adds entries from a .bib file to the current tracked bibliography
    Add { bibfile: PathBuf },
    /// Removes a specific citation corresponding to the given citekey 
    Remove { citekey: String }, 
}

fn main() {
    let args = Args::parse();

    let bibtex_input = r#"
            @article{Smith2023,
                author = "John Smith",
                title = {A Paper},
                journal = {Some Journal},
                year = {2023},
                pages = {100--110}
            }

            @book{Jones2022,
                title = "A Book",
                author = "Jane Jones",
                publisher = "A Publisher",
                year = {2022}
            }
        "#;

        let mut parser = BibtexParser::new(bibtex_input);
        let result = parser.parse();
        println!("{:?}", result.ok().unwrap());
}
