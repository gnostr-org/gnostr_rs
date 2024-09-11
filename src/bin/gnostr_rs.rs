use std::convert::TryInto;
use std::io::Read;
use std::{env, process};

use gnostr_types::Event;

use gnostr_rs::DEFAULT_RELAY_URL;

fn main() {
    let relay_url = DEFAULT_RELAY_URL;
    let args_vector: Vec<String> = env::args().collect();

    //#[allow(unreachable_code)]
    for i in 0..args_vector.len() {
        if i == args_vector.len() {
            process::exit(i.try_into().unwrap());
        } else {
            if args_vector.is_empty() {
                print!("args_vector.len() = {}", 0);
            };
            if args_vector.len() == 1 {
                //
                //
                // gnostr_rs defaults to gnostr_bins::post_event
                // it may recieve a signed nostr note body from
                // (std::io::stdin) and repost it.
                //
                // TODO:
                // utility {<note_body>} | gnostr_rs --post-event --relay
                //
                //
                // {"id": "<id_string>","pubkey": "<pubkey_string>","created_at": <utc_secs>,"kind": <int>,"tags": ["tag_element_1,"tag_element_2"],"content": "","sig": "schnorr_signature"}
                //
                // {"id": "note_id","pubkey": "pubkey_string","created_at": <utc_secs>,"kind": <int>,"tags": [["t","gnostr"],["gnostr","gnostr"],["e","<event_id>"],["p","<recipient_pubkey>"],["nonce","0","0"]],"content": "<encrypted_content>","sig": "<schnorr_signature>"}
                let mut s: String = String::new();
                std::io::stdin().read_to_string(&mut s).unwrap();
                let event: Event = serde_json::from_str(&s).unwrap();
                //always reprint s for further piping
                print!("{}", s);
                gnostr_bins::post_event(relay_url, event);
            };
            if args_vector.len() == 2 {
                if relay_url == DEFAULT_RELAY_URL {};
                //catch help
                if args_vector[1] == "-h" {
                    println!("gnostr --sec <priv_key> ... | gnostr-rs");
                    println!(
                        "gnostr --sec <priv_key> ... | gnostr-post-event --relay {}",
                        DEFAULT_RELAY_URL
                    );
                    println!(
                        "gnostr --sec $(gnostr-sha256) ... | gnostr-post-event {} | gnostr-cat {}",
                        DEFAULT_RELAY_URL, DEFAULT_RELAY_URL
                    );
                    process::exit(0);
                }
                if args_vector[1] == "--help" {
                    println!(
                        "gnostr --sec <priv_key> | gnostr-post-event --relay {}",
                        DEFAULT_RELAY_URL
                    );
                    println!(
                        "gnostr --sec $(gnostr-sha256) | gnostr-post-event {} | gnostr-cat {}",
                        DEFAULT_RELAY_URL, DEFAULT_RELAY_URL
                    );
                    process::exit(0);
                }
                //catch version
                if args_vector[1] == "-v" {
                    const VERSION: &str = env!("CARGO_PKG_VERSION");
                    print!("v{}", VERSION);
                    process::exit(0);
                }
                if args_vector[1] == "--version" {
                    const VERSION: &str = env!("CARGO_PKG_VERSION");
                    print!("v{}", VERSION);
                    process::exit(0);
                }
            }
        };
    }

    //print!("gnostr");
    //use gnostr_rs::keys::{secret_key_from_str, get_str_keys_from_secret};

    //let secret_key = secret_key_from_str(&std::env::var("SECRET_KEY").unwrap()).unwrap_or(secret_key_from_str("").expect("REASON"));
    //let (secret_key_str, public_key_str) = get_str_keys_from_secret(&secret_key);

    //assert_eq!(secret_key_str, std::env::var("SECRET_KEY").unwrap());
    //assert_eq!(public_key_str, std::env::var("PUBLIC_KEY").unwrap());
}
