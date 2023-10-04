use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input text
    #[arg(required(true), num_args(1..))]
    text: Vec<String>,

    /// Skip newline at the end
    #[arg(short, default_value_t = false)]
    newline_skip: bool,
}

fn main() {
    let args = Args::parse();
    let ending = if args.newline_skip { "" } else { "\n" };
    let input_text = args.text.join(" ");

    print!("{}{}", input_text, ending);
}
