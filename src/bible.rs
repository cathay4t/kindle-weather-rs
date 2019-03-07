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
pub struct BibleVerse {
    script: String,
    version: String,
    reference: String,
}

static _API_URL: &str =
    "https://beta.ourmanna.com/api/v1/get/?format=json&order=random";

pub fn bible_get() -> BibleVerse {
    let txt: Value = serde_json::from_str(&http_get(_API_URL)).unwrap();
    let detail = txt.get("verse").unwrap().get("details").unwrap();
    let script = detail.get("text").unwrap().as_str().unwrap();
    let version = detail.get("version").unwrap().as_str().unwrap();
    let reference = detail.get("reference").unwrap().as_str().unwrap();
    BibleVerse {
        script: script.into(),
        version: version.into(),
        reference: reference.into(),
    }
}
