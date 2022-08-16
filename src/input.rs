use std::io::stdin;

pub fn get_user_input(field: String) -> String {
    println!("{}", "What is the ".to_owned() + &field.to_owned());
    let mut text = String::new();
    stdin().read_line(&mut text)
        .ok()
        .expect(&("Not a valid ".to_owned() + &field.to_owned()));

    text
}
