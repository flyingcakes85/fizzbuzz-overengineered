mod cli_parse;
mod program_data;

fn main() {
    let program_data = cli_parse::parse();
    println!("{:#?}", program_data);
}
