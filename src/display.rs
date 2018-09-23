use ansi_term::Colour::{Green, Red};

pub fn render_success(text: &str) {
    println!("{}", Green.paint(text));
}

pub fn render_error(text: &str) {
    println!("{}", Red.paint(text));
}

pub fn help() {
    let help = r##"One line
Two lines
    Three lines
    "##;
    println!("{}", help);  
}
