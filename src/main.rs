use crate::editor::editor::Editor;
use buffer::Buffer;
use std::env;
use std::io;

mod buffer;
mod editor;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let file: Option<&String> = arguments.get(1);

    match file {
        Some(file) => {
            let stdout = io::stdout();
            let buffer = Buffer::from_file(file);

            match buffer {
                Ok(buffe) => {
                    let mut editor = Editor::new(stdout, buffe);

                    // now running the editor
                    editor.run();
                }
                Err(err) => panic!("Couldnot open the file {}", err),
            }
        }
        None => println!("Invalid arguments."),
    }
}
