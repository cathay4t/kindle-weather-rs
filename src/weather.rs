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

use super::http::http_get;
use chrono::Local;
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct WeatherData {
    pub condition: String,
    pub temp_max: i32,
    pub temp_min: i32,
    pub icon: String,
}

impl WeatherData {
    fn new_from_he_forcast(forcast: &Value, is_night: bool) -> WeatherData {
        let condition_string = match is_night {
            true => "textNight",
            false => "textDay",
        };
        let icon_code_str = match is_night {
            true => "iconNight",
            false => "iconDay",
        };
        WeatherData {
            condition: format!("{}", forcast[condition_string].as_str().unwrap()),
            temp_max: forcast["tempMax"].as_str().unwrap().parse::<i32>().unwrap(),
            temp_min: forcast["tempMin"].as_str().unwrap().parse::<i32>().unwrap(),
            icon: get_icon(forcast[icon_code_str].as_str().unwrap()),
        }
    }
}

fn get_icon(code_str: &str) -> String {
    let mut icon_map: HashMap<u32, String> = HashMap::new();
    icon_map.insert(100, "☀".into());
    icon_map.insert(101, "⛅".into());
    icon_map.insert(102, "⛅".into());
    icon_map.insert(103, "⛅".into());
    icon_map.insert(104, "☁".into());

    icon_map.insert(200, "🍃".into());
    icon_map.insert(201, "🎐".into());
    icon_map.insert(202, "🎐".into());
    icon_map.insert(203, "🎐".into());
    icon_map.insert(204, "🍃".into());
    icon_map.insert(205, "🌬".into());
    icon_map.insert(206, "🌬".into());
    icon_map.insert(207, "🌪".into());
    icon_map.insert(208, "🌪".into());
    icon_map.insert(209, "🌪".into());
    icon_map.insert(210, "🌪".into());
    icon_map.insert(211, "🌪".into());
    icon_map.insert(212, "🌪".into());
    icon_map.insert(213, "🌪".into());

    icon_map.insert(300, "🌦".into());
    icon_map.insert(301, "🌦".into());
    icon_map.insert(302, "⛈".into());
    icon_map.insert(303, "⛈".into());
    icon_map.insert(303, "⛈".into());
    icon_map.insert(304, "⛈".into());
    icon_map.insert(305, "☔".into());
    icon_map.insert(306, "🌧".into());
    icon_map.insert(307, "🌧".into());
    icon_map.insert(308, "🌊".into());
    icon_map.insert(309, "💧".into());
    icon_map.insert(310, "⛈️".into());
    icon_map.insert(311, "⛈️".into());
    icon_map.insert(312, "⛈️".into());
    icon_map.insert(313, "🌨️".into());
    icon_map.insert(314, "🌧".into());
    icon_map.insert(315, "🌧".into());
    icon_map.insert(316, "🌧".into());
    icon_map.insert(317, "🌊".into());
    icon_map.insert(318, "🌊".into());
    icon_map.insert(350, "🌦️".into());
    icon_map.insert(351, "🌦️".into());
    icon_map.insert(399, "☔".into());

    icon_map.insert(400, "❄️".into());
    icon_map.insert(401, "❄️".into());
    icon_map.insert(402, "☃".into());
    icon_map.insert(403, "☃".into());
    icon_map.insert(404, "☃".into());
    icon_map.insert(405, "☃".into());
    icon_map.insert(406, "☃".into());
    icon_map.insert(407, "☃".into());
    icon_map.insert(408, "☃".into());
    icon_map.insert(409, "☃".into());
    icon_map.insert(410, "☃".into());
    icon_map.insert(499, "☃".into());

    icon_map.insert(500, "🌁".into());
    icon_map.insert(501, "🌁".into());
    icon_map.insert(502, "🌫".into());
    icon_map.insert(503, "🌫".into());
    icon_map.insert(504, "🌫".into());
    icon_map.insert(505, "🌫".into());
    icon_map.insert(506, "🌫".into());
    icon_map.insert(507, "🌫".into());
    icon_map.insert(508, "🌫".into());
    icon_map.insert(509, "🌫".into());
    icon_map.insert(510, "🌫".into());
    icon_map.insert(511, "🌫".into());
    icon_map.insert(512, "🌫".into());
    icon_map.insert(513, "🌫".into());
    icon_map.insert(514, "🌫".into());
    icon_map.insert(515, "🌫".into());

    icon_map.insert(900, "🌡️".into());
    icon_map.insert(901, "☃️".into());

    if let Ok(code) = code_str.parse::<u32>() {
        if let Some(icon) = icon_map.get(&code) {
            icon.clone()
        } else {
            format!("{}", code)
        }
    } else {
        "❓".into()
    }
}

static _API_URL: &str = "https://devapi.qweather.com/v7/weather/3d";

pub fn weather_get(api_key: &str, longtitude: &str, latitude: &str) -> [WeatherData; 3] {
    let url = format!(
        "{API_URL}?location={LON},{LAT}&key={KEY}&lang=zh",
        API_URL = _API_URL,
        LON = longtitude,
        LAT = latitude,
        KEY = api_key,
    );

    let ret: Map<String, Value> = serde_json::from_str(&http_get(&url)).unwrap();
    let forcasts = ret["daily"].as_array().unwrap();
    let now = Local::now();
    let night = Local::today().and_hms(17, 0, 0);

    [
        WeatherData::new_from_he_forcast(&forcasts[0], now > night),
        WeatherData::new_from_he_forcast(&forcasts[1], false),
        WeatherData::new_from_he_forcast(&forcasts[2], false),
    ]
}
