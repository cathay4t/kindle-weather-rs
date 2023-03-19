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
mod fund;
mod http;
mod sci;
mod sun;
mod svg_data;
mod weather;

use aqi::aqi_get;
use chrono::Duration;
use chrono::Local;
use chrono_tz::Tz;
use sci::sci_get;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use strfmt::strfmt;
use svg_data::KINDLE_WEATHER_SVG;
use weather::weather_get;

const TMP_SVG_FILE_PATH: &str = "/tmp/kindle_weather.svg";
const TMP_PNG_FILE_PATH: &str = "/tmp/_kindle_weather.png";
const TMP_PNG_FILE_PATH2: &str = "/tmp/__kindle_weather.png";

#[derive(Debug, Deserialize)]
struct KindleWeatherConfig {
    heweather_key: String,
    aqicn_key: String,
    #[serde(default)]
    output_file: String,
    latitude: String,
    longtitude: String,
    #[serde(rename = "TZ1")]
    tz1: String,
    #[serde(rename = "TZ2")]
    tz2: String,
    #[serde(default = "default_rotation")]
    rotation: String,
    #[serde(rename = "FUND0")]
    fund0: String,
    #[serde(rename = "FUND1")]
    fund1: String,
    #[serde(rename = "FUND2")]
    fund2: String,
}

impl ::std::default::Default for KindleWeatherConfig {
    fn default() -> Self {
        Self {
            heweather_key: "".into(),
            aqicn_key: "".into(),
            output_file: "/tmp/kindle_weather.png".into(),
            latitude: "".into(),
            longtitude: "".into(),
            tz1: "".into(),
            tz2: "".into(),
            rotation: "right".into(),
            fund0: "".into(),
            fund1: "".into(),
            fund2: "".into(),
        }
    }
}

fn get_time(tz_str: &str) -> String {
    let now = Local::now();
    let tz: Tz = tz_str.parse().unwrap();
    format!("{}", now.with_timezone(&tz).format("%H:%M"))
}

fn main() {
    let home_path = match dirs::home_dir() {
        Some(p) => p.to_str().unwrap().to_string(),
        None => panic!("Failed to get $HOME path"),
    };
    let default_cfg_path = format!("{}/.config/kindle_weather.cfg", home_path);
    let matches = clap::Command::new("kindle_weather")
        .version("0.1")
        .author("Gris Ge <cnfourt@gmail.com>")
        .about("CLI utile for generating weather PNG for kindle")
        .arg(
            clap::Arg::new("CFG_PATH")
                .long("cfg")
                .takes_value(true)
                .default_value(&default_cfg_path)
                .help("The path of config file"),
        )
        .get_matches();

    let cfg_path = matches.value_of("CFG_PATH").expect("BUG on --cfg argument");

    let mut fd = File::open(cfg_path).expect("Failed to open config file");
    let mut contents = String::new();
    fd.read_to_string(&mut contents)
        .expect("Failed to read config file");

    let cfg: KindleWeatherConfig =
        toml::from_str(&contents).expect("Failed to parse config file");

    let now = Local::now();
    let weather_data =
        weather_get(&cfg.heweather_key, &cfg.longtitude, &cfg.latitude);
    let mut vars = HashMap::new();
    let (aqi, aqi_main) =
        aqi_get(&cfg.aqicn_key, &cfg.longtitude, &cfg.latitude);
    vars.insert("AQI".to_string(), format!("{}", aqi));
    vars.insert("AQI_MAIN".to_string(), format!("{}", aqi_main));
    let sci_data = sci_get();
    vars.insert(
        "TIME".to_string(),
        format!("{}", now.format("%b %d %a %H:%M")),
    );
    vars.insert("SCI".to_string(), format!("{:.2}", sci_data[0]));
    vars.insert("SCHG".to_string(), format!("{:.2}%", sci_data[1] * 100f32));
    vars.insert("H0".to_string(), format!("{}", weather_data[0].temp_max));
    vars.insert("H0".to_string(), format!("{}", weather_data[0].temp_max));
    vars.insert("H1".to_string(), format!("{}", weather_data[1].temp_max));
    vars.insert("H2".to_string(), format!("{}", weather_data[2].temp_max));
    vars.insert("L0".to_string(), format!("{}", weather_data[0].temp_min));
    vars.insert("L1".to_string(), format!("{}", weather_data[1].temp_min));
    vars.insert("L2".to_string(), format!("{}", weather_data[2].temp_min));
    vars.insert("C0".to_string(), format!("{}", weather_data[0].condition));
    vars.insert("C1".to_string(), format!("{}", weather_data[1].condition));
    vars.insert("C2".to_string(), format!("{}", weather_data[2].condition));
    vars.insert("ICON0".to_string(), format!("{}", weather_data[0].icon));
    vars.insert("ICON1".to_string(), format!("{}", weather_data[1].icon));
    vars.insert("ICON2".to_string(), format!("{}", weather_data[2].icon));
    vars.insert(
        "TZ1_NAME".to_string(),
        format!("{}", &cfg.tz1.split("/").last().unwrap()),
    );
    vars.insert("TZ1_TIME".to_string(), get_time(&cfg.tz1));
    vars.insert(
        "TZ2_NAME".to_string(),
        format!("{}", &cfg.tz2.split("/").last().unwrap()),
    );
    vars.insert("TZ2_TIME".to_string(), get_time(&cfg.tz2));
    vars.insert("DAY0".to_string(), format!("{}", now.format("%a")));
    let day1 = now + Duration::days(1);
    vars.insert("DAY1".to_string(), format!("{}", day1.format("%a")));
    let day2 = now + Duration::days(2);
    vars.insert("DAY2".to_string(), format!("{}", day2.format("%a")));
    let (sunrise, sunset) = sun::sun_rise_set_get(
        &cfg.heweather_key,
        &cfg.longtitude,
        &cfg.latitude,
    );
    vars.insert("SUNRISE".to_string(), sunrise);
    vars.insert("SUNSET".to_string(), sunset);
    let (fund0_name, fund0_value, fund0_rate) = fund::fund_get(&cfg.fund0);
    let (fund1_name, fund1_value, fund1_rate) = fund::fund_get(&cfg.fund1);
    let (fund2_name, fund2_value, fund2_rate) = fund::fund_get(&cfg.fund2);
    vars.insert("FUND0_NAME".to_string(), fund0_name);
    vars.insert("FUND0_VALUE".to_string(), fund0_value);
    vars.insert("FUND0_RATE".to_string(), fund0_rate);
    vars.insert("FUND1_NAME".to_string(), fund1_name);
    vars.insert("FUND1_VALUE".to_string(), fund1_value);
    vars.insert("FUND1_RATE".to_string(), fund1_rate);
    vars.insert("FUND2_NAME".to_string(), fund2_name);
    vars.insert("FUND2_VALUE".to_string(), fund2_value);
    vars.insert("FUND2_RATE".to_string(), fund2_rate);
    let svg_data = strfmt(KINDLE_WEATHER_SVG, &vars).unwrap();
    let mut svg_fd =
        File::create(TMP_SVG_FILE_PATH).expect("Failed to create svg file");
    svg_fd.write_all(svg_data.as_bytes()).unwrap();
    Command::new("rsvg-convert")
        .args(&[
            "--background-color=white",
            "-o",
            TMP_PNG_FILE_PATH,
            TMP_SVG_FILE_PATH,
        ])
        .status()
        .expect("Failed to run rsvg-convert");
    Command::new("pngcrush")
        .args(&["-c", "0", "-ow", TMP_PNG_FILE_PATH, TMP_PNG_FILE_PATH2])
        .status()
        .expect("Failed to run pngcrush");
    let rotation_degree: &str = match cfg.rotation.as_ref() {
        "right" => "270",
        "left" => "90",
        "no" => "0",
        _ => panic!("Invalid rotation configuration"),
    };
    Command::new("convert")
        .args(&[
            TMP_PNG_FILE_PATH,
            "-rotate",
            rotation_degree,
            &cfg.output_file,
        ])
        .status()
        .expect("Failed to run convert");
    println!("{}", &cfg.output_file);
}

fn default_rotation() -> String {
    "no".to_string()
}
