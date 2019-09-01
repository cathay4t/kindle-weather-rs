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

use reqwest;

pub fn http_get(url: &str) -> String {
    let hc = reqwest::Client::new();
    let ret = hc
        .get(url)
        .header("user-agent", "Mozilla/5.0")
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("url {}", url);
    ret.to_string()
}
