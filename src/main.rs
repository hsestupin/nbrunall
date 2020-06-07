use std::env;
use std::fs::File;
use std::io::Read;

// separator (might be empty when it's first cell in the notebook), end_index
type Cell<'a> = (&'a str, usize);

const SEPARATORS: [&str; 2] = ["#%%\n", "#%%\r\n"];

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut file = File::open(filename)?;
    println!("Evaluating file: {}", filename);

    let mut text = String::new();
    file.read_to_string(&mut text)?;
    println!("------File-------");
    println!("{}", text);
    println!("------------------");

    if text.is_empty() {
        println!("File is empty");
        return Ok(());
    }

    let mut cell_start = 0;
    while let Some((sep, cell_end)) = find_next_cell(&text[cell_start..]) {
        println!("Cell[\n{}\n]", &text[cell_start + sep.len()..cell_start + cell_end]);
        cell_start += cell_end;
    }

    Ok(())
}

fn find_next_cell(text: &str) -> Option<Cell> {
    if text.is_empty() {
        return None;
    }

    for separator in &SEPARATORS {
        if text.starts_with(separator) {
            let start_idx = separator.len();
            let next_sep = match find_next_separator(&text[start_idx..]) {
                Some(next) => next + start_idx,
                None => text.len()
            };
            return Some((separator, next_sep));
        }
    }
    let next_sep = match find_next_separator(&text) {
        Some(next) => next,
        None => text.len()
    };
    return Some(("", next_sep));
}

fn find_next_separator(text: &str) -> Option<usize> {
    let mut i = 0;
    while let Some(newline_sep_start) = text[i..].find("\n#%%") {
        let sep_start = newline_sep_start + 1; // skip newline char
        let subtext = &text[sep_start..];

        for separator in &SEPARATORS {
            if subtext.starts_with(separator) {
                return Some(sep_start);
            }
        }

        i += newline_sep_start + 1
    }
    return None;
}