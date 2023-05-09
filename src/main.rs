use std::fs;
use chrono::{DateTime, NaiveDateTime, Timelike, TimeZone, Offset, NaiveTime, Duration};
use plotters::prelude::*;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Reaction {
    reaction: String,
    actor: String
}

#[derive(Debug, Deserialize)]
struct User {
    name: String
}

#[derive(Debug, Deserialize)]
struct Message {
    sender_name: String,
    timestamp_ms: i64,
    content: Option<String>,
    reactions: Option<Vec<Reaction>>
}

#[derive(Debug, Deserialize)]
struct Messages {
    participants: Vec<User>,
    messages: Vec<Message>,
    title: String,
    is_still_participant: bool,
    thread_path: String,
    magic_words: Vec<String>,
}

fn main() {
    let file_1 = fs::File::open("message_1.json").expect("file??");
    let messages_1: Messages = serde_json::from_reader(file_1).expect("???");
    let file_2 = fs::File::open("message_2.json").expect("file??");
    let messages_2: Messages = serde_json::from_reader(file_2).expect("???");

    let my_messages = messages_1.messages.iter().chain(messages_2.messages.iter());
    let data = {
        my_messages.map(|message| {
            NaiveDateTime::from_timestamp_millis(message.timestamp_ms).expect("converted") +
            Duration::hours(10)
        })
    };

    let root = BitMapBackend::new("hi.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).expect("cant draw?");
    let mut chart = ChartBuilder::on(&root)
        .caption("our texting habits", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..24, 0..2000).expect("ahsa");
    chart.configure_mesh().draw().expect("expected");

    chart.draw_series(
        Histogram::vertical(&chart).style(RED.filled()).margin(10)
        .data(data.map(|x| (x.time().hour() as i32, 1)))
    ).expect("draw")
    .label("volume")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED.filled()));


    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw().expect("cant draw");
}
