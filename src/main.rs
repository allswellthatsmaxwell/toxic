extern crate csv;
#[macro_use] extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate getopts;
use std::error::Error;
use std::borrow::Cow;
use std::fs::File;
use std::env;
use std::str::Split;
use std::io::prelude::*;
use regex::Regex;
use regex::Captures;
use getopts::Options;
use getopts::Matches;
use std::collections::HashMap;
use std::collections::BTreeSet;

fn construct_opts(args: Vec<String>) -> Matches {
    let mut opts = Options::new();
    opts.optopt("", "train", "training set csv", "TRAIN");
    opts.optopt("", "test", "test set csv", "TEST");
    opts.optopt("", "action", "action to perform", "ACT");
    match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(e) => { panic!(e.to_string()) }        
    }    
}

fn match_arg(arg_short: &str, matches: &Matches) -> String {
    match matches.opt_str(arg_short) {
        Some(val) => val,
        None => panic!("Argument not supplied: {}", arg_short)
    }
}

#[derive(Debug, Deserialize)]
struct FlatRecord {
    id: String,
    comment_text: String,
    toxic: i8,
    severe_toxic: i8,
    obscene: i8,
    threat: i8,
    insult: i8,
    identity_hate: i8
}

#[derive(Debug)]
struct Record<'a> {
    id: String,
    comment_text: String,
    responses: BTreeSet<&'a str>
}

// Collect response columns into a hashmap<variables, response>
fn unflatten_record<'a>(flat_record: FlatRecord) -> Record<'a> {
    let mut responses = BTreeSet::new();
    if flat_record.toxic         == 1 {responses.insert("toxic");}
    if flat_record.severe_toxic  == 1 {responses.insert("severe_toxic");}
    if flat_record.obscene       == 1 {responses.insert("obscene");}
    if flat_record.threat        == 1 {responses.insert("threat");}
    if flat_record.insult        == 1 {responses.insert("insult");}
    if flat_record.identity_hate == 1 {responses.insert("identity_hate");}
    Record {id: flat_record.id,
            comment_text: flat_record.comment_text,
            responses: responses}
}

fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<Error>> {
    let file = File::open(file_path)?;
    let mut records = vec![];
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let flat_record: FlatRecord = result?;
        let record = unflatten_record(flat_record);
        records.push(record);
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

fn count_response_vars(records: Vec<Record>) -> HashMap<BTreeSet<&str>, u32> {
    let mut counts = HashMap::new();
    for record in records {
        *counts.entry(record.responses).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let matches = construct_opts(args);

    let train_path = match_arg("train", &matches);
    // let test_path = match_arg("test", &matches);
    let action = match_arg("action", &matches);
    let train = match read_csv(&train_path) {
        Ok(rs) => rs,
        Err(e) => panic!("error parsing training file: {}", e),
    };

    match action.as_ref() {
        "cat_train" => {
            for record in train {
                let comment = &record.comment_text;
                println!("{:?}", comment);
            }
        },
        "count_tokens" => {
            for record in train {
                let comment = &record.comment_text;
                let sanitized = sanitize_text(&comment);
                let tokens = sanitized.split(" ");
                
            }
        },
        "count_responses" => {
            let counts = count_response_vars(train);
            for (response, count) in &counts {
                println!("{:?}: {}", response, count)
            }
        },
        _ => println!("Unknown action: {}", action)
    }
    
    
    //println!("{:?}", sanitize_text(example_comment));
}


