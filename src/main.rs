extern crate iron;
extern crate bodyparser;
extern crate persistent;
extern crate graffiti;
#[macro_use]
extern crate serde_derive;

use persistent::Read;
use iron::status;
use iron::prelude::*;
use graffiti::tokenizer::*;
use graffiti::invertedindex::*;

#[derive(Debug, Clone, Deserialize)]
struct Document {
    title: String,
    body: Option<String>,
}

let tokens: &'static str = 
    "
    Alpha => 65..123
    Number => 48..57
    Whitespace => 9,10,13,32
    Punctuation => 33..46
    Punctuation => 58..65
    ";

let transitions: &'static str = 
    "
    Start => Alpha => Alpha
    Start => Number => Number
    Start => Whitespace => Whitespace
    Start => Punctuation => Punctuation
    Alpha => Alpha | Number => Alpha
    Number => Number => Number
    Number => Alpha => Alpha
    Whitespace => Whitespace => Whitespace
    Punctuation => Punctuation => Punctuation
    ";

fn log_body(req: &mut Request) -> IronResult<Response> {

    let struct_body = req.get::<bodyparser::Struct<Document>>();
    match struct_body {
        Ok(Some(struct_body)) => println!("Parsed body:\n{:?}", struct_body),
        Ok(None) => println!("No body"),
        Err(err) => println!("Error: {:?}", err)
    }

    let mut ii = InvertedIndex::new();
    let tokenizer = Tokenizer::new(&tokens, &transitions);
    let tokens = tokenizer.tokenize(&struct_body.body);
    let filtered_tokens = tokens.into_iter().filter(|f| f.s == State(utils::get_hash_val(b"Alpha"))).collect::<Vec<Token>>();

    println!("Adding doc {:?}", 0);
    ii.add_doc(filtered_tokens, 0);

    Ok(Response::with(status::Ok))
}

const MAX_BODY_LENGTH: usize = 1024 * 1024 * 10;

fn main() {
    let mut chain = Chain::new(log_body);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));
    Iron::new(chain).http("localhost:3000").unwrap();
}