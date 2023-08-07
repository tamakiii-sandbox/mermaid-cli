mod args {
    use clap::Parser;

    mod generate {
        use clap::Parser;

        #[derive(Parser)]
        pub struct Arguments {
            pub path: std::path::PathBuf,
        }
    }

    #[derive(Parser)]
    pub enum Command {
        Generate(generate::Arguments),
    }

    #[derive(Parser)]
    pub struct Arguments {
        #[clap(subcommand)]
        pub command: Command,
    }

    pub fn parse() -> Arguments {
        Arguments::parse()
    }
}

fn main() {
    let args = args::parse();

    match args.command {
        args::Command::Generate(args) => println!("path: {}", args.path.to_str().unwrap()),
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_args_ok() {
        // Arguments::parse
    }
}
