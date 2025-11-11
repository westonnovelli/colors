use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "colors",
    about = "Convert hex color codes to RGB",
    after_help = "Note (PowerShell): values beginning with '#' must be quoted to avoid being treated as a comment."
)]
struct Cli {
    #[arg(
        value_name = "HEX",
        num_args = 1..,
        help = "One or more hex color codes (3 or 6 hex digits), with or without a leading '#'."
    )]
    hex: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut had_error = false;

    for input in &cli.hex {
        match colors::parse_hex(input) {
            Ok(rgb) => {
                println!("{}", colors::to_rgb_string(&rgb));
            }
            Err(err) => {
                had_error = true;
                eprintln!("{}: {}", input, err);
            }
        }
    }

    if had_error {
        std::process::exit(1);
    }
}
