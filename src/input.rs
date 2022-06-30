use std::ffi::OsString;

pub fn get_user_selection(files: Vec<OsString>) -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Pick a rom to load!");
    for i in 0..files.len() {
        let file = &files[i].clone().into_string().expect("Has Name");
        println!("{}: {}", i, file);
    }
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    println!("You typed: {}", s);
    let index: usize = s.parse().unwrap();
    return files[index].clone().into_string().expect("has name");
}
