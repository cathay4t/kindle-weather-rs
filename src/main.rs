// Copyright (C) 2019 Gris Ge
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// Author: Gris Ge <cnfourt@gmail.com>

mod aqi;
mod bible;
mod http;
mod sci;
mod weather;

extern crate clap;
extern crate dirs;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate toml;

use bible::bible_get;
use clap::{App, Arg};
use sci::sci_get;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use weather::weather_get;

#[derive(Debug, Deserialize)]
struct KindleWeatherConfig {
    heweather_api: String,
    aqi_city_name: String,
    output_file: String,
    latitude: String,
    longtitude: String,
}

impl ::std::default::Default for KindleWeatherConfig {
    fn default() -> Self {
        Self {
            heweather_api: "".into(),
            aqi_city_name: "".into(),
            output_file: "/tmp/kindle_weather.png".into(),
            latitude: "".into(),
            longtitude: "".into(),
        }
    }
}

fn main() {
    let home_path = match dirs::home_dir() {
        Some(p) => p.to_str().unwrap().to_string(),
        None => panic!("Failed to get $HOME path"),
    };
    let default_cfg_path = format!("{}/.config/kindle_weather.cfg", home_path);
    let matches = App::new("kindle_weather")
        .version("0.1")
        .author("Gris Ge <cnfourt@gmail.com>")
        .about("CLI utile for generating weather PNG for kindle")
        .arg(
            Arg::from_usage("--cfg=[CFG_PATH] 'The path of config file'")
                .default_value(&default_cfg_path),
        )
        .get_matches();

    let cfg_path = matches.value_of("cfg").expect("BUG on --cfg argument");

    let mut fd = File::open(cfg_path).expect("Failed to open config file");
    let mut contents = String::new();
    fd.read_to_string(&mut contents)
        .expect("Failed to read config file");

    let cfg: KindleWeatherConfig =
        toml::from_str(&contents).expect("Failed to parse config file");

    println!("{:?}", cfg);
    //    println!("{:?}", bible_get());
    //    println!("sci: {:?}", sci_get());
    println!(
        "weather: {:?}",
        weather_get(&cfg.heweather_api, &cfg.longtitude, &cfg.latitude)
    );
}
