/* write a twitch chat bot that counts how many times a user says "BOOBA" in chat, saves and reads from a "booba_counts.json", uses a "config.json" for the twitch chat connection, stop.

now make it have a chat command "!resetb" that wipes the "booba_counts.json" that can only be done by moderators and above, stop.

now make it have a "!top3" command that says the top 3 highest counts from "booba_counts.json" in one chat message, stop.

now make it so the "!top3" command has a cooldown feature that can be configured in the "config.json, stop.

make sure all modules and libraries that are needed to run the script are included in the code, stop.

Do it all in the RUST programming language without any comments, explanations, or commentary. */


use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // read config.json
    let mut config_file = File::open("config.json").expect("Couldn't open config.json");
    let mut config_string = String::new();
    config_file.read_to_string(&mut config_string).expect("Couldn't read config.json");
    let config: HashMap&lt;String, String&gt; = serde_json::from_str(&config_string).expect("Couldn't parse config.json");

    // connect to twitch chat
    let mut client = irc::client::prelude::Client::from_config(config).unwrap();
    client.identify().unwrap();
    client.for_each_incoming(|m| {
        if let irc::proto::Command::PRIVMSG(channel, msg) = m.command {
            // check if message contains "BOOBA"
            if msg.contains("BOOBA") {
                // read booba_counts.json
                let mut booba_file = File::open("booba_counts.json").expect("Couldn't open booba_counts.json");
                let mut booba_string = String::new();
                booba_file.read_to_string(&mut booba_string).expect("Couldn't read booba_counts.json");
                let mut booba_counts: HashMap&lt;String, u64&gt; = serde_json::from_str(&booba_string).expect("Couldn't parse booba_counts.json");

                // increment booba count for user
                let user = m.prefix.split("!").next().unwrap();
                if booba_counts.contains_key(user) {
                    booba_counts.insert(user.to_string(), booba_counts[user] + 1);
                } else {
                    booba_counts.insert(user.to_string(), 1);
                }

                // write booba_counts.json
                let serialized = serde_json::to_string(&booba_counts).unwrap();
                let mut booba_file = File::create("booba_counts.json").expect("Couldn't create booba_counts.json");
                booba_file.write_all(serialized.as_bytes()).expect("Couldn't write to booba_counts.json");
            }

            // check if message is "!resetb"
            if msg == "!resetb" {
                // check if user is moderator or above
                let user = m.prefix.split("!").next().unwrap();
                if config["moderators"].contains(user) {
                    // write empty booba_counts.json
                    let serialized = serde_json::to_string(&HashMap::new()).unwrap();
                    let mut booba_file = File::create("booba_counts.json").expect("Couldn't create booba_counts.json");
                    booba_file.write_all(serialized.as_bytes()).expect("Couldn't write to booba_counts.json");
                }
            }

            // check if message is "!top3"
            if msg == "!top3" {
                // check if top3 command is on cooldown
                let mut cooldown_file = File::open("cooldown.json").expect("Couldn't open cooldown.json");
                let mut cooldown_string = String::new();
                cooldown_file.read_to_string(&mut cooldown_string).expect("Couldn't read cooldown.json");
                let cooldown: HashMap&lt;String, u64&gt; = serde_json::from_str(&cooldown_string).expect("Couldn't parse cooldown.json");
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                if cooldown.contains_key("top3") && cooldown["top3"] &gt; now {
                    return;
                }

                // read booba_counts.json
                let mut booba_file = File::open("booba_counts.json").expect("Couldn't open booba_counts.json");
                let mut booba_string = String::new();
                booba_file.read_to_string(&mut booba_string).expect("Couldn't read booba_counts.json");
                let booba_counts: HashMap&lt;String, u64&gt; = serde_json::from_str(&booba_string).expect("Couldn't parse booba_counts.json");

                // sort booba_counts
                let mut booba_vec: Vec&lt;(&String, &u64)&gt; = booba_counts.iter().collect();
                booba_vec.sort_by(|a, b| b.1.cmp(a.1));

                // format top 3 message
                let mut top3_msg = String::new();
                for (i, (user, count)) in booba_vec.iter().enumerate() {
                    if i == 3 {
                        break;
                    }
                    top3_msg.push_str(&format!("{}: {} ", user, count));
                }

                // send top 3 message
                client.send(irc::proto::Command::PRIVMSG(channel.to_string(), top3_msg)).unwrap();

                // write cooldown.json
                let mut cooldown: HashMap&lt;String, u64&gt; = HashMap::new();
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let cooldown_time = now + config["cooldown_time"].parse::&lt;u64&gt;().unwrap();
                cooldown.insert("top3".to_string(), cooldown_time);
                let serialized = serde_json::to_string(&cooldown).unwrap();
                let mut cooldown_file = File::create("cooldown.json").expect("Couldn't create cooldown.json");
                cooldown_file.write_all(serialized.as_bytes()).expect("Couldn't write to cooldown.json");
            }
        }
    }).unwrap();
}