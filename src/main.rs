use cli::cli::parse_args;
use cli::file::read_file;

pub mod cli;
pub mod logic;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path_res = parse_args(args);

    if file_path_res.is_err() {
        println!("Error: {}", file_path_res.err().unwrap());
        return;
    }

    let file_path = file_path_res.unwrap();
    let content_res = read_file(&file_path);

    if content_res.is_err() {
        println!("Error: {}", content_res.err().unwrap());
        return;
    }
    let content = content_res.unwrap();

    let total_word_count = logic::totals::words(&content);
    println!("Total words: {}", total_word_count);

    let total_char_count = logic::totals::chars(&content);
    println!("Total characters: {}", total_char_count);
}
