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
mod http;
mod sci;
mod svg_data;
mod weather;

extern crate chrono;
extern crate clap;
extern crate dirs;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate strfmt;
extern crate toml;

use aqi::aqi_get;
use chrono::{Duration, Local};
use clap::{App, Arg};
use sci::sci_get;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use strfmt::strfmt;
use svg_data::KINDLE_WEATHER_SVG;
use weather::weather_get;

#[derive(Debug, Deserialize)]
struct KindleWeatherConfig {
    #[serde(default)]
    icons_folder: String,
    heweather_key: String,
    #[serde(default)]
    output_file: String,
    latitude: String,
    longtitude: String,
}

impl ::std::default::Default for KindleWeatherConfig {
    fn default() -> Self {
        let home_path = match dirs::home_dir() {
            Some(p) => p.to_str().unwrap().to_string(),
            None => panic!("Failed to get $HOME path"),
        };
        let default_icons_folder =
            format!("{}/.config/kindle_weather/icons", home_path);
        Self {
            icons_folder: "".into(),
            heweather_key: "".into(),
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
    let default_cfg_path =
        format!("{}/.config/kindle_weather/kindle_weather.cfg", home_path);
    let default_icons_folder =
        format!("{}/.config/kindle_weather/icons", home_path);
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

    let d0 = Local::now();
    let d1 = d0 + Duration::days(1);
    let d2 = d0 + Duration::days(2);
    let weather_data =
        weather_get(&cfg.heweather_key, &cfg.longtitude, &cfg.latitude,
                    &cfg.icons_folder);
    let mut vars = HashMap::new();
    vars.insert(
        "AQI".to_string(),
        format!(
            "{}",
            aqi_get(&cfg.heweather_key, &cfg.longtitude, &cfg.latitude)
        ),
    );
    let sci_data = sci_get();
    vars.insert(
        "TIME".to_string(),
        format!("{}", d0.format("%b %d %a %H:%M")),
    );

    vars.insert("SCI".to_string(), format!("{}", sci_data[0]));
    vars.insert("SCHG".to_string(), format!("{:.2}%", sci_data[1] * 100f32));
    vars.insert("D0".to_string(), format!("{}", d0.format("%a")));
    vars.insert("D1".to_string(), format!("{}", d1.format("%a")));
    vars.insert("D2".to_string(), format!("{}", d2.format("%a")));
    vars.insert("H0".to_string(), format!("{}", weather_data[0].temp_max));
    vars.insert("H0".to_string(), format!("{}", weather_data[0].temp_max));
    vars.insert("H1".to_string(), format!("{}", weather_data[1].temp_max));
    vars.insert("H2".to_string(), format!("{}", weather_data[2].temp_max));
    vars.insert("L0".to_string(), format!("{}", weather_data[0].temp_min));
    vars.insert("L1".to_string(), format!("{}", weather_data[1].temp_min));
    vars.insert("L2".to_string(), format!("{}", weather_data[2].temp_min));
    let svg_data = strfmt(KINDLE_WEATHER_SVG, &vars).unwrap();
    let mut svg_fd =
        File::create(&cfg.output_file).expect("Failed to create svg file");
    svg_fd.write_all(svg_data.as_bytes()).unwrap();
    println!("{}", &cfg.output_file);
}
