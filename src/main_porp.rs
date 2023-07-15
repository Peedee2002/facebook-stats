use chrono::{Duration, Local, NaiveDate, NaiveDateTime};
use plotters::prelude::*;
use std::{collections::HashMap, fs};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Message {
    sender_name: String,
    timestamp_ms: i64,
}

#[derive(Debug, Deserialize)]
struct Messages {
    messages: Vec<Message>,
}

fn main() {
    let file_1 = fs::File::open("message_1.json").expect("file??");
    let messages_1: Messages = serde_json::from_reader(file_1).expect("???");
    let file_2 = fs::File::open("message_2.json").expect("file??");
    let messages_2: Messages = serde_json::from_reader(file_2).expect("???");
    let file_3 = fs::File::open("message_3.json").expect("file??");
    let messages_3: Messages = serde_json::from_reader(file_3).expect("???");
    let file_4 = fs::File::open("message_4.json").expect("file??");
    let messages_4: Messages = serde_json::from_reader(file_4).expect("???");

    let my_messages = messages_1.messages.iter().chain(messages_2.messages.iter()).chain(messages_3.messages.iter()).chain(messages_4.messages.iter());
    let data = {
        my_messages.map(|message| {
            let val = (
                (NaiveDateTime::from_timestamp_millis(message.timestamp_ms)
                    .expect("converted")
                    .date()
                    + Duration::hours(10))
                .signed_duration_since(NaiveDate::from_ymd_opt(2022, 12, 1).unwrap())
                .num_days() as i32,
                message.sender_name.clone(),
            );
            val
        })
    };

    let new_data = {
        let mut ret = HashMap::new();
        for d in 8..250 {
            let count_peter = data
                .clone()
                .filter(|v| v.0 == d && v.1.starts_with("Peter"))
                .count();
            // println!("count peter: {}", count_peter);
            let count_total = data.clone().filter(|v| v.0 == d).count();
            // println!("count total: {}", count_total);
            let value = if count_total != 0 {
                ((count_peter as f32 / count_total as f32) * 100.0) as i32
            } else {
                0
            };
            // println!("inserting {}, {}", d, value);
            ret.insert(d, value);
        }
        ret
    };
    let count_peter = data.clone().filter(|v| v.1.starts_with("Peter")).count();
    let count_total = data.clone().count();
    println!("total ratio: {}", count_peter as f32 / count_total as f32);

    let root = BitMapBackend::new("porportion.png", (1980, 1200)).into_drawing_area();
    root.fill(&WHITE).expect("cant draw?");
    let mut chart = ChartBuilder::on(&root)
        .caption("texting porportion", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            8..Local::now()
                .date_naive()
                .signed_duration_since(NaiveDate::from_ymd_opt(2022, 12, 1).unwrap())
                .num_days() as i32,
            0..100,
        )
        .expect("ahsa");
    chart.configure_mesh().draw().expect("expected");
    chart
        .draw_series(
            Histogram::vertical(&chart)
                .style(GREEN.filled())
                .margin(10)
                .data(new_data.clone().iter().map(|x| (*x.0, *x.1))),
        )
        .expect("draw");

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .expect("cant draw");
}
