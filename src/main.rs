mod charted;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "pie")]
    graphtype: String,

    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.svg")]
    output: String,
}

fn generate_chart(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    match args.graphtype.to_lowercase().as_str() {
        "pie" => charted::pie::generate(args.input, args.output),
        _ => {
            println!("Unknown graph type {}", args.graphtype);
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    generate_chart(args)?;

    Ok(())
}
