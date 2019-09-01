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
use serde_json::{Map, Value};

static _API_URL: &str = "http://api.waqi.info/feed";

pub fn aqi_get(
    api_key: &str,
    longtitude: &str,
    latitude: &str,
) -> (u32, String) {
    let url = format!(
        "{API_URL}/geo:{LAT};{LON}/?token={KEY}",
        API_URL = _API_URL,
        LON = longtitude,
        LAT = latitude,
        KEY = api_key,
    );
    let html_data = http_get(&url);
    match serde_json::from_str::<Map<String, Value>>(&html_data) {
        Ok(ret) => (
            ret["data"]["aqi"].as_u64().unwrap() as u32,
            format!("{}", ret["data"]["dominentpol"].as_str().unwrap()),
        ),
        Err(e) => {
            println!("AQI retrieve failure, error {}\n{}", e, &html_data);
            (0u32, format!("N/A"))
        }
    }
}
