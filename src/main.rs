use std::env;
use std::fs;
use std::path::PathBuf;
mod commands;
mod generate_asm;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args[1].to_string().clone();
    let mut contents: Vec<String>;
    let mut mult_files = false;
    let mut tokens: Vec<String>;
    let mut asm_code: Vec<String> = Vec::new();
    let mut cur_filename = String::new();

    if filename.ends_with(".vm") {
        contents = match tokenizer::file_contents(filename.clone()) {
            Ok(description) => description,
            Err(err) => {
                panic!("There was a problem opening the file: {:?}", err)
            }
        };
        tokens = tokenizer::remove_comments(contents);
        asm_code.append(&mut generate_asm::asm(
            tokens,
            cur_filename.clone(),
            mult_files,
        ));
        generate_asm::write_output(asm_code, filename.clone())
            .map_err(|err| println!("{:?}", err))
            .ok();
    } else {
        mult_files = true;
        let paths = fs::read_dir(filename.clone()).unwrap();
        for path in paths {
            cur_filename = path.unwrap().path().display().to_string();
            let path_split = PathBuf::from(cur_filename.clone());
            let crop_filename = path_split
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
            contents = match tokenizer::file_contents(cur_filename.clone()) {
                Ok(description) => description,
                Err(err) => {
                    panic!("There was a problem opening the file: {:?}", err)
                }
            };
            tokens = tokenizer::remove_comments(contents);
            asm_code.append(&mut generate_asm::asm(
                tokens,
                crop_filename.clone(),
                mult_files,
            ));
        }
        generate_asm::write_output(asm_code, filename.clone())
            .map_err(|err| println!("{:?}", err))
            .ok();
    }
}
