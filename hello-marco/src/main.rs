// A command line tool that plays Marco Polo

use clap::Parser;

#[derive(Parser)]
#[clap(name = "Marco Polo", version = "1.0", author = "Your Name")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Play { name: String },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result: String = hello_marco::marco_polo(&name);
            print!("{}", result)
        }
        None => println!("What?"),
    }
}
