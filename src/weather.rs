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

#[derive(Debug)]
pub struct WeatherData {
    pub condition: String,
    pub temp_max: i32,
    pub temp_min: i32,
}

impl WeatherData {
    fn new_from_he_forcast(forcast: &Value, is_night: bool) -> WeatherData {
        let condition_string = match is_night {
            true => "cond_txt_n",
            false => "cond_txt_d",
        };
        WeatherData {
            condition: format!(
                "{}",
                forcast[condition_string].as_str().unwrap()
            ),
            temp_max: forcast["tmp_max"]
                .as_str()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
            temp_min: forcast["tmp_min"]
                .as_str()
                .unwrap()
                .parse::<i32>()
                .unwrap(),
        }
    }
}

static _API_URL: &str = "https://free-api.heweather.com/s6/weather/forecast";

pub fn weather_get(
    api_key: &str,
    longtitude: &str,
    latitude: &str,
) -> [WeatherData; 3] {
    let url = format!(
        "{API_URL}?location={LON},{LAT}&key={KEY}&lang=zh",
        API_URL = _API_URL,
        LON = longtitude,
        LAT = latitude,
        KEY = api_key,
    );

    let ret: Map<String, Value> =
        serde_json::from_str(&http_get(&url)).unwrap();
    let forcasts = ret["HeWeather6"][0]["daily_forecast"].as_array().unwrap();
    let now = Local::now();
    let night = Local::today().and_hms(17, 0, 0);

    [
        WeatherData::new_from_he_forcast(&forcasts[0], now > night),
        WeatherData::new_from_he_forcast(&forcasts[1], false),
        WeatherData::new_from_he_forcast(&forcasts[2], false),
    ]
}
