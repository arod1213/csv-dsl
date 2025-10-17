use clap_derive::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = ",")]
    pub separator: String,

    #[arg(short, long, num_args = 1..)]
    pub filepaths: Vec<String>,

    #[arg(long)]
    pub schema: String,
}
