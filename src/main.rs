use std::env;
use std::fs;
mod tokenizer;
mod commands;
mod generate_asm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = args[1].to_string().clone();
    let mut contents: Vec<String> = Vec::new();
    let mut mult_files = false;
    if filename.ends_with(".vm") {
        contents = match tokenizer::file_contents(filename.clone()) {
                    Ok(description) => description,
                    Err(err) => {
                        panic!("There was a problem opening the file: {:?}", err)
                    }
                };
    } else {
        mult_files = true;
        let paths = fs::read_dir(filename.clone()).unwrap();
        let mut extra_content: Vec<String> = Vec::new();
        for path in paths {
            extra_content = match tokenizer::file_contents(path.unwrap().path().display().to_string()) {
                        Ok(description) => description,
                        Err(err) => {
                            panic!("There was a problem opening the file: {:?}", err)
                        }
                    };
            contents.append(&mut extra_content);
        }
        println!("{}", filename.clone());
    }

    let mut tokens = tokenizer::remove_comments(contents);
    let mut asm = generate_asm::asm(tokens, mult_files);
    generate_asm::write_output(asm, filename.clone());
}
