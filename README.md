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
* Example at <https://www.emqx.com/en/blog/how-to-use-mqtt-in-rust> Works but loses connection after a bit (and reconnects.) I'm pretty sure this (at least with the present code) was the result of VLAN firewalling on a home LAN interfering with some communications between broker and client. Client can connect to broker but broker cannot connect to client.
* Addressing message decomposition next.

## Message parsing

Some example messages (as reported by `mosquitto_sub -v` to include toipic and payload) are

```text
HA/latham/dining_rm_west/temp_humidity {"t": 1687186984, "temp": 74.7, "humid": 47.0}
HA/sodus/master_bedroom/temp_humidity {"t": 1687186986, "temp": 72.98, "humid": 54, "press": 987.96}
hass/status offline
hass/status online
CM/oak/NA/state gone
```

Most (but not all) of the messages include a JSON formatted payload. The plan is to save to the database a 

* timestamp  (synthesized from message receipt time when not included in the JSON payload)
* Other fields parsed from the topic or substituted with "(unk") when not.
* Complete JSON payload (synthesized if needed from a non-JSON payload as e.g. `{"t": 1687186984, "payload": "offline"}`)

## Errata

It is ridiculously easy to spin up a Mosquitto server for testing. Since I already have Docker installed on my desktop I use that. (<https://hub.docker.com/_/eclipse-mosquitto>)

```text
docker run -it --network=host eclipse-mosquitto
```

In another terminal I can enter the image to edit configuration, restart and monitor logging.

```text
docker exec -it <container-name> sh 
# edit /mosquitto/config/mosquitto.conf and <ctrl>D to exit
# <ctrl>C to exit interactive docker session
docker start -i <container-name>
# Restart interactive docker container
cargo run "tcp://localhost:1883"
# run Rust client and specify broker
```
