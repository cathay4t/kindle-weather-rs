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

pub(crate) fn http_get(url: &str) -> String {
    http_get_with_referer(url, None)
}

pub(crate) fn http_get_with_referer(
    url: &str,
    reference: Option<&str>,
) -> String {
    let mut headers = reqwest::header::HeaderMap::new();
    if let Some(reference) = reference {
        headers.insert("Referer", reference.parse().unwrap());
    }

    let client = reqwest::blocking::Client::builder()
        .gzip(true)
        .timeout(std::time::Duration::from_secs(10))
        .default_headers(headers)
        .build()
        .unwrap();

    let ret = client.get(url).send().unwrap().text().unwrap();
    println!("url {}", url);
    let s = ret.to_string();
    s
}
