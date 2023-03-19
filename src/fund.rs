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

use std::collections::HashMap;

use serde::Deserialize;

use crate::http::http_post;

const API_URL: &str = "https://fund.sina.com.cn/fund/api/fundDetail";

const FUND_NAME_MAX_LEN: usize = 14;

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct SinaFundReply {
    code: i32,
    msg: String,
    data: SinaFundData,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct SinaFundData {
    market: SinaFundMarket,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct SinaFundMarket {
    base_info: SinaFundBaseInfo,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct SinaFundBaseInfo {
    fundname: String,
    netvalue: String,
    dayincratio: String,
}

pub fn fund_get(fund_id: &str) -> (String, String, String) {
    let html_txt = http_post(API_URL, &format!("fundcode={fund_id}&type=4"));

    let reply: SinaFundReply = serde_json::from_str(&html_txt).unwrap();
    if reply.code != 0 {
        eprintln!(
            "Failed to get fund {fund_id} info: {}, {}",
            reply.code, reply.msg
        );
        (fund_id.to_string(), "N/A".into(), "N/A".into())
    } else {
        (
            reply.data.market.base_info.fundname,
            reply.data.market.base_info.netvalue.to_string(),
            format!(
                "{}%",
                reply
                    .data
                    .market
                    .base_info
                    .dayincratio
                    .parse::<f64>()
                    .unwrap()
                    * 100.0
            ),
        )
    }
}
