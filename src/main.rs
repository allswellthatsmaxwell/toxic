extern crate csv;
#[macro_use]
extern crate serde_derive;
use std::error::Error;
use std::process;
use std::fs::File;

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
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record);
    }
    Ok(records)
}

fn main() {
    let train_path = "data/train.csv";
    if let Err(err) = read_csv(&train_path) {
        println!("error reading train file: {}", err);
        process::exit(1);
    }
}


