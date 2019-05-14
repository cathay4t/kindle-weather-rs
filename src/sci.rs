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

static _API_URL: &str = "http://hq.sinajs.cn/list=sh000001";

pub fn sci_get() -> [f32; 2] {
    let txt = http_get(_API_URL);
    let info: Vec<&str> = txt.split(',').collect();
    let cur:f32 = info[3].parse().unwrap();
    let pre:f32 = info[2].parse().unwrap();
    let inc:f32 = cur/pre - 1f32;
    [cur, inc]
}
