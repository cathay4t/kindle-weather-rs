This project will generate a 1024x768 resolution PNG file suitable for kindle
paper white 2 to display.


Workflow:
 * Jeal break your kindle.
 * Install USBNET and enable password-less authentication via WIFI.
 * `cargo install --path .` from this project.
 * Install `pngcrush` and `rsvg-convert`
 * Request API key from qweather.com
 * Request API key for aqicn.org
 * Create below as `~/.config/kindle_weather.cfg`:
```toml
heweather_key = 'get_from_qweather.com'
aqicn_key = 'get_from_aqicn.org'
output_file = '/tmp/weather.png'
latitude = '30.51'
longtitude = '104.01'
rotation = 'right'
TZ1='Europe/Berlin'
TZ2='Asia/Jerusalem'
# This is the number of funding
FUND0='002264'
FUND1='000946'
FUND2='002001'
```
 * Create this script to `~/bin/pw2_weather.sh`
```bash
#!/bin/bash

~/.cargo/bin/kindle-weather-rs
scp /tmp/weather.png root@pw2:/tmp/weather.png
ssh root@pw2 /usr/sbin/eips -f -g /tmp/weather.png
```
 * Add this line to your `/etc/crontab`:
```
0,30 * * * * fge /home/fge/bin/pw2_weather.sh 1>/dev/null 2>/dev/null
```
