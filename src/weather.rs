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
use serde_json::Value;

#[derive(Debug)]
pub struct WeatherData {
    condition: String, // Changed to enum
    temp_max: i32,
    temp_min: i32,
}

#[derive(Debug, Deserialize)]
pub struct HeWeatherRet {
}

static _API_URL: &str = "https://free-api.heweather.com/s6/weather/forecast";

pub fn weather_get(
    api_key: &str,
    longtitude: &str,
    latitude: &str,
) -> [WeatherData; 3] {
    let url = format!(
        "{API_URL}?location={LON},{LAT}&key={KEY}&lang=en",
        API_URL = _API_URL,
        LON = longtitude,
        LAT = latitude,
        KEY = api_key,
    );

    let ret: Value = serde_json::from_str(&http_get(&url)).unwrap();
    println!("url: {}", url);
    println!("{:?}", ret);
    [
        WeatherData {
            condition: "clear sky".into(),
            temp_max: 0,
            temp_min: 0,
        },
        WeatherData {
            condition: "clear sky".into(),
            temp_max: 0,
            temp_min: 0,
        },
        WeatherData {
            condition: "clear sky".into(),
            temp_max: 0,
            temp_min: 0,
        },
    ]
}
