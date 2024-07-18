use clap::Parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    // Loading arguments
    let args = Cli::parse();
    println!("path is: {:?}", args.path);

    // Loading file
    let file = std::fs::read_to_string(&args.path).expect("could not read file");
    println!("file is: {}", file);

        
}
