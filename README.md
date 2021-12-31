# mackerel-plugin-co2mon

This is a [mackerel](https://mackerel.io) plugin for co2, and temperature sensors using [custom co2-mini](https://www.kk-custom.co.jp/emp/CO2-mini.html).

## Prerequisite

Set permission, and read sensor values for your co2-mini.
Please check [co2mon README permissions section](https://github.com/lnicola/co2mon#permissions).

## Usage

Add your mackerel-agent.conf for followings:

```config
[plugin.metrics.CO2MINI]
command = ["/path/to/your/build/mackerel-plugin-co2mon"]
```

Restart mackerel agent.

## LICENSE

This project is licensed under of
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
