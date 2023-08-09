mod args {
    use clap::Parser;

    mod generate {
        use clap::Parser;

        #[derive(Parser)]
        pub struct Arguments {
            pub path: std::path::PathBuf,

            #[arg(value_parser = port_in_range)]
            pub port: u16,
        }

        const PORT_RANGE: std::ops::RangeInclusive<usize> = 1..=65535;

        fn port_in_range(s: &str) -> Result<u16, String> {
            let port = s
                .parse()
                .map_err(|_| format!("`{s}` isn't a port number"))?;
            if PORT_RANGE.contains(&port) {
                Ok(port as u16)
            } else {
                Err(format!(
                    "port not in range {}-{}",
                    PORT_RANGE.start(),
                    PORT_RANGE.end()
                ))
            }
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
        args::Command::Generate(args) => {
            println!("path: {}, port: {}", args.path.to_str().unwrap(), args.port)
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn parse_args_ok() {
        // Arguments::parse
    }
}
