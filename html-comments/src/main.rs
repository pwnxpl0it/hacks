use std::io;
use html5tokenizer::{NaiveParser, Token};

fn main() {
    let mut html = String::from("");

    loop {
        let mut input = String::from("");
        io::stdin().read_line(&mut input).unwrap();
        match input.as_str(){
                "" => break,
                "\n" => {},
                _ => html.push_str(&input.replace("\n","")),
            }
        }

    for token in NaiveParser::new(html.as_str()).flatten(){
        match token{
            Token::Comment(comment) => {
                println!("{}",comment.replace("\n"," ").trim());
            }
            _ => {
                // IGNORE
            }
        }
    }
}
