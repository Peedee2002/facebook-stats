use chrono::{DateTime, Duration, NaiveDateTime, NaiveTime, Offset, TimeZone, Timelike};
use encoding::Encoding;
use plotters::prelude::*;
use std::{fs, collections::HashMap};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Reaction {
    reaction: String,
    actor: String,
}

#[derive(Debug, Deserialize)]
struct User {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    sender_name: String,
    timestamp_ms: i64,
    content: Option<String>,
    reactions: Option<Vec<Reaction>>,
}

#[derive(Debug, Deserialize)]
struct Messages {
    participants: Vec<User>,
    messages: Vec<Message>,
    title: String,
    is_still_participant: bool,
    thread_path: String,
}

fn main() {
    let directory = fs::read_dir(".").expect("heh");
    let message_files = directory.into_iter().filter(|file| file.is_ok())
        .map(|a| a.unwrap())
        .filter(|entry| entry.file_name().to_str().unwrap().contains("message"));
    let my_messages = message_files.fold(Vec::new(), |acc, curr| {
        let file = fs::File::open(curr.file_name()).expect("found it before");
        let messages: Messages = serde_json::from_reader(file).expect("???");
        acc.into_iter().chain(messages.messages.into_iter()).collect()
    });
    let mut reactions = HashMap::new();
    let mut reaction_actor = HashMap::new();
    let reacted_list: Vec<_> = my_messages.into_iter().filter_map(|mut message| {
        let value = message.reactions.take();
        value
    }).flatten().collect();
    reacted_list.iter().for_each(|reaction| {
        let emoji = encoding::all::UTF_8.decode(
            &encoding::all::ISO_8859_1.encode(&reaction.reaction, encoding::EncoderTrap::Strict).expect("aa"),
            encoding::DecoderTrap::Strict
        ).expect("aaa");
        let key = emoji + " sent by " + &reaction.actor;
        let val = reactions.get(&key);
        match val {
            Some(val) => reactions.insert(key, val + 1),
            None => reactions.insert(key, 1),
        };
    });
    reacted_list.iter().for_each(|reaction| {
        let key = &reaction.actor;
        let val = reaction_actor.get(key);
        match val {
            Some(val) => reaction_actor.insert(key, val + 1),
            None => reaction_actor.insert(key, 1),
        };
    });
    reactions.into_iter().for_each(|(key, value)| {
        println!("{key}: {value}");
    });
    println!("actors:");
    reaction_actor.into_iter().for_each(|(key, value)| {
        println!("{key}: {value}");
    });
}
