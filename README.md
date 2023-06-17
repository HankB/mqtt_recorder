# mqtt_recorder

Application to read MQTT messages and store to MariaDB.

## Motivation

I have a Python process that copies messages published by a private MQTT broker to a MariaDB database so I can look at that information someday. I'm not happy wioth some of the behavior so I'm planning to reqqrite it in Rust because I'm cheap and kleen to learn. (Fawlty Towers reference.)

I have not been successful in finding an example MQTT subscribere that works 100% But there is one that (from EMQ) that comes pretty close. I have seen the issue it has and I should be able to fix it (and learn a bit more about Rust/paho-mqtt in the process.) I thought it might be useful to share with others. 

## Plan

Develop

1. An MQTT client that can subscribe and remain connected.
1. Parsers that can handle the message formats I see on my private broker, including some that do not conform to the policies that the publishers I write follow.
1. A MariaDB client that can connect to a server, initiailze a database and then save all incoming messages to that database.
1. Profit! (for some definition of ...)

## References

* [paho-mqtt Rust API](https://docs.rs/paho-mqtt/latest/paho_mqtt/)

## Requirements

```text
sudo apt install libssl-dev build-essential cmake pkg-config
```

Minimum Supported Rust Version (MSRV) v1.63.0

## Status

* Example provided by paho does not build <https://github.com/eclipse/paho.mqtt.rust#example> (I should probably revisit this after identifying the need to install `pkg-config`.)
* Example at <https://www.emqx.com/en/blog/how-to-use-mqtt-in-rust> Works but loses connection after a bit (and reconnects.)
