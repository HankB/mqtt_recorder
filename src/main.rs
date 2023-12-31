// code slavishly copied from https://www.emqx.com/en/blog/how-to-use-mqtt-in-rust
// "The code for subscribe"

use std::{env, process, thread, time::Duration};

extern crate paho_mqtt as mqtt;

const DFLT_BROKER: &str = "tcp://mqtt:1883";
const DFLT_CLIENT: &str = "rust_subscribe_x";
// const DFLT_TOPICS:&[&str] = &["#", "#"];
const TOPIC: &str = "#";
// The qos list that match topics above.
// const DFLT_QOS:&[i32] = &[0, 1];

// Reconnect to the broker when connection is lost.
fn try_reconnect(cli: &mqtt::Client) -> bool {
    println!("Connection lost. Waiting to retry connection");
    for _ in 0..12 {
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            return true;
        }
        thread::sleep(Duration::from_millis(1000));
    }
    println!("Unable to reconnect after several attempts.");
    false
}

// Subscribes to single topics.
fn subscribe_topics(cli: &mqtt::Client) {
    if let Err(e) = cli.subscribe(TOPIC, 0) {
        println!("Error subscribes topics: {:?}", e);
        process::exit(1);
    }
}

fn main() {
    let host = env::args()
        .nth(1)
        .unwrap_or_else(|| DFLT_BROKER.to_string());

    // Define the set of options for the create.
    // Use an ID for a persistent session.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .client_id(DFLT_CLIENT.to_string())
        .finalize();

    // Create a client.
    let cli = mqtt::Client::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    // Initialize the consumer before connecting.
    let rx = cli.start_consuming();

    // Connect and wait for it to complete or fail.

    loop {
        // Define the set of options for the connection.
        /*
        let lwt = mqtt::MessageBuilder::new()
            .topic("test")
            .payload("Consumer lost connection")
            .finalize();
        */
        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(5))
            .clean_session(false)
            //.will_message(lwt)
            .finalize();
        if let Err(e) = cli.connect(conn_opts) {
            println!("Unable to connect:\n\t{:?}", e);
            thread::sleep(Duration::from_millis(1000));
        } else {
            break;
        }
    }

    // Subscribe topics.
    subscribe_topics(&cli);

    println!("Processing requests...");
    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("{}", msg);
        } else if !cli.is_connected() {
            if try_reconnect(&cli) {
                println!("Resubscribe topics...");
                subscribe_topics(&cli);
            } else {
                break;
            }
        }
    }

    // If still connected, then disconnect now.
    if cli.is_connected() {
        println!("Disconnecting");
        cli.unsubscribe(TOPIC).unwrap();
        cli.disconnect(None).unwrap();
    }
    println!("Exiting");
}
