use std::collections::HashMap;
use std::fs;

// Add necessary libraries and modules
use serde::{Deserialize, Serialize};
use serde_json;
use twitch_irc;

#[derive(Deserialize)]
struct Config {
    username: String,
    oauth_token: String,
    channel: String,
}

fn main() {
    // Read config.json
    let config = serde_json::from_str(fs::read_to_string("config.json").unwrap().as_str()).unwrap();

    // Connect to Twitch chat
    let client = twitch_irc::Client::new(config.username, config.oauth_token, config.channel).unwrap();

    // Load booba_counts.json
    let mut counts: HashMap<String, u32> = serde_json::from_str(fs::read_to_string("booba_counts.json").unwrap().as_str()).unwrap();

    // Listen for messages in chat
    client.for_each_message(|message| {
        // Check if message contains "BOOBA"
        let data = message.data.to_lowercase();
        if data.contains("booba") {
            // Increment count for user
            let username = message.get_username().to_string();
            let count = data.matches("booba").count() as u32;
            *counts.entry(username).or_insert(0) += count;

            // Announce count in chat
            client.say(&format!("{} said 'BOOBA'! Their current count is {}", username, counts[&username]));
        }
    });

    // Save counts to booba_counts.json
    fs::write("booba_counts.json", serde_json::to_string(&counts).unwrap()).unwrap();

    // END OF CODE
}
