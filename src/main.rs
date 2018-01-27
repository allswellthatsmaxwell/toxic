extern crate csv;
#[macro_use] extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde_derive;
use std::error::Error;
use std::borrow::Cow;
use std::fs::File;
use regex::Regex;
use regex::Captures;

#[derive(Debug, Deserialize)]
struct Record {
    id: String,
    comment_text: String,
    toxic: i8,
    severe_toxic: i8,
    obscene: i8,
    threat: i8,
    insult: i8,
    identity_hate: i8
}

fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<Error>> {
    let file = File::open(file_path)?;
    let mut records = vec![];
    let mut rdr = csv::Reader::from_reader(file);
    let mut i = 0;
    let lim = 3;
    for result in rdr.deserialize() {
        if i > lim { break }
        let record: Record = result?;
        records.push(record);
        i += 1;
    }
    Ok(records)
}

fn sanitize_text(text: &str) -> Cow<str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\s+)|(\p{P}+)").unwrap();
    }
    RE.replace_all(text, |captures: &Captures| {
        if captures.get(1).is_some() {
            " ".into()
        } else if captures.get(2).is_some() {
            "".into()
        } else {
            unreachable!("Unknown group matched")
        }
    })
}

fn main() {
    let train_path = "data/train.csv";
    let train = match read_csv(&train_path) {
        Ok(rs) => rs,
        Err(e) => panic!("error parsing training file: {}", e),
    };
    println!("{:?}", &train[1].comment_text);
    println!("{:?}", sanitize_text(&train[1].comment_text));
}


