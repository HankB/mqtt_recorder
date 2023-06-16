# mqtt_recorder

Application to read MQTT messages and store to MariaDB.

## References

* [paho-mqtt Rust API](https://docs.rs/paho-mqtt/latest/paho_mqtt/)

## Requirements

```text
sudo apt install libssl-dev build-essential cmake pkg-config
```

Minimum Supported Rust Version (MSRV) v1.63.0

## Status

* Example provided by paho does not build <https://github.com/eclipse/paho.mqtt.rust#example>
* Example at <https://www.emqx.com/en/blog/how-to-use-mqtt-in-rust> Works but loses connection after a bit (and reconnects.)