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

static _API_URL: &str = "https://devapi.qweather.com/v7/astronomy/sun";

pub fn sun_rise_set_get(api_key: &str, longtitude: &str, latitude: &str) -> (String, String) {
    let url = format!(
        "{API_URL}?location={LON},{LAT}&key={KEY}&lang=en&date={DAY}",
        API_URL = _API_URL,
        LON = longtitude,
        LAT = latitude,
        KEY = api_key,
        DAY = chrono::offset::Local::today().format("%Y%m%d")
    );

    let ret: Map<String, Value> = serde_json::from_str(&http_get(&url)).unwrap();
    (
        qweather_time_to_hour_minute(ret["sunrise"].as_str().unwrap()),
        qweather_time_to_hour_minute(ret["sunset"].as_str().unwrap()),
    )
}

fn qweather_time_to_hour_minute(ts: &str) -> String {
    format!(
        "{}",
        chrono::DateTime::parse_from_str(ts, "%FT%H:%M%:z")
            .unwrap()
            .format("%H:%M")
    )
}
