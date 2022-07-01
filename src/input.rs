use std::ffi::OsString;

pub fn get_user_selection(files: Vec<OsString>) -> String {
    use std::io::{stdin, stdout, Write};
    println!("Pick a rom to load!");
    for i in 0..files.len() {
        let file = &files[i].clone().into_string().expect("Has Name");
        println!("{}: {}", i, file);
    }
    let _ = stdout().flush();

    let index = loop {
        let mut s = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        print!("{}", s);
        if let Ok(x) = s.parse::<usize>() {
            if x < files.len() && x >= 0 {
                break x;
            }
        }
        println!(" is not a valid input");
        println!("Please enter a number from 0 to {}", files.len() - 1);
    };
    return files[index].clone().into_string().expect("has name");
}
