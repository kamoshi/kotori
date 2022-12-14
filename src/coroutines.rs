use std::thread;
use chrono::{Datelike, Timelike};
use serenity::http::Http;
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;
use tokio::time::{sleep, Duration};


pub fn init(ctx: Context) {
    tokio::spawn(async move {
        let channel = ChannelId(841002228964786197);
        //let channel = ChannelId(1047123653847957544);

        let mut last_sent = 0_u32;
        loop {
            sleep(Duration::from_secs(10)).await;
            papal(&ctx.http, &channel, &mut last_sent).await;
        }
    });
}

async fn papal(http: &Http, channel: &ChannelId, last_sent: &mut u32) {
    let now = chrono::Local::now();
    let (d, h, m) = (now.day(), now.hour(), now.minute());

    if d != *last_sent && h == 21 && m == 37 {
        channel.send_message(&http, |m| m.content("papież"))
            .await
            .expect("TODO: panic message");
        *last_sent = d;
    }
}
