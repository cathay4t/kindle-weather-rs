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
use select::document::Document;
use select::predicate::Name;
use std::collections::HashMap;
use strfmt::strfmt;

const BASE_URL: &str = "http://stocks.sina.cn/fund/?code={FUND_ID}&vt=4";

const FUND_NAME_MAX_LEN: usize = 14;

pub fn fund_get(fund_id: &str) -> (String, String, String) {
    let mut vars = HashMap::new();
    vars.insert("FUND_ID".to_string(), format!("{}", fund_id));
    let url = strfmt(BASE_URL, &vars).unwrap();
    let html_txt = http_get(&url);
    let html_doc = Document::from(html_txt.as_ref());
    let mut fund_name = String::new();
    let mut fund_val = String::new();
    let mut fund_rat = String::new();
    //    let mut fund_real = String::new();

    for node in html_doc.find(Name("span")) {
        if (!fund_name.is_empty())
            && (!fund_val.is_empty())
            && (!fund_rat.is_empty())
        //            && (!fund_real.is_empty())
        {
            break;
        }
        if node.attr("class") == None {
            continue;
        }
        if node.attr("class").unwrap() == "fund_name" {
            fund_name = node.text().to_string();
            continue;
        }
        if &(node.attr("class").unwrap()[.."j_fund_value".len()])
            == "j_fund_value"
        {
            fund_val = node.text().to_string();
            continue;
        }
        if &(node.attr("class").unwrap()[.."j_fund_valExt".len()])
            == "j_fund_valExt"
        {
            fund_rat = match node.text().as_str() {
                "0.00%" => "+0.00%".into(),
                _ => node.text().into(),
            };
            continue;
        }
        //        if &(node.attr("class").unwrap()[.."j_premium_rate".len()])
        //            == "j_premium_rate"
        //        {
        //            fund_real = node.text().to_string();
        //            continue;
        //        }
    }
    (
        fund_name
            .chars()
            .into_iter()
            .take(FUND_NAME_MAX_LEN)
            .collect(),
        fund_val,
        fund_rat,
    )
}
