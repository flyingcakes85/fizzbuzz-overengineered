mod cli_parse;

fn main() {
    let program_data = cli_parse::parse();
    println!("{:#?}", program_data);
}
