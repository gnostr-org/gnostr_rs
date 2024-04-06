use std::env;
use structopt::StructOpt;
use std::iter::Peekable;


use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

use nostr_rust::{
    events::extract_events_ws, nostr_client::Client, req::ReqFilter, utils::parse_content_tags,
    Identity, Message,
};

fn handle_message(relay_url: &String, message: &Message) -> Result<(), String> {
    println!("Received message from {}: {:?}", relay_url, message);

    let events = extract_events_ws(message);
    println!("Events: {:?}", events);

    Ok(())
}

fn main() {
    let secret_key = env::var("SECRET_KEY").unwrap_or_else(|_| {
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string()
         //panic!("SECRET_KEY environment variable not set");
    });

    println!("{}", secret_key);

    let my_identity = Identity::from_str(&secret_key).unwrap();

    let nostr_client = Arc::new(Mutex::new(
        Client::new(vec!["wss://relay.damus.io"]).unwrap(),
    ));

    // Run a new thread to handle messages
    let nostr_clone = nostr_client.clone();
    let handle_thread = thread::spawn(move || {
        println!("Listening...");
        let events = nostr_clone.lock().unwrap().next_data().unwrap();

        for (relay_url, message) in events.iter() {
            handle_message(relay_url, message).unwrap();
        }
    });

    // Change metadata
    nostr_client
        .lock()
        .unwrap()
        .set_metadata(
            &my_identity,
            Some("Rust Nostr Client test account"),
            Some("Hello Nostr! #5"),
            None,
            None,
            0,
        )
        .unwrap();

    // Subscribe to my last text note
    let subscription_id = nostr_client
        .lock()
        .unwrap()
        .subscribe(vec![ReqFilter {
            ids: None,
            authors: Some(vec![
                "a34b99f22c790c4e36b2b3c2c35a36db06226e41c692fc82b8b56ac1c540c5bd".to_string(),
            ]),
            kinds: None,
            e: None,
            p: None,
            since: None,
            until: None,
            limit: Some(1),
        }])
        .unwrap();

    // Unsubscribe
    nostr_client
        .lock()
        .unwrap()
        .unsubscribe(&subscription_id)
        .unwrap();

    // You can use the parse content tags method to get the content and the tags from a string
    // let tags = parse_content_tags("hello #world", vec![], Some(nostr_rust::DEFAULT_HASHTAG), true, true);
    // assert_eq!(tags.content, "hello #world");
    //  assert_eq!(tags.tags, vec![vec!["t", "world"]]);

    // Publish a text note
    nostr_client
        .lock()
        .unwrap()
        .publish_text_note(&my_identity, "Hello Nostr! :)", &[], 0)
        .unwrap();

    // Publish a proof of work text note with a difficulty target of 15
    nostr_client
        .lock()
        .unwrap()
        .publish_text_note(&my_identity, "Hello Nostr! :)", &[], 15)
        .unwrap();

    // Wait for the thread to finish
    handle_thread.join().unwrap();
}
