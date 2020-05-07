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

pub static KINDLE_WEATHER_SVG: &str = r###"<svg
xmlns="http://www.w3.org/2000/svg" height="600" width="800" version="1.1"
     xmlns:xlink="http://www.w3.org/1999/xlink">
    <g font-family="DejaVu Sans" style="text-anchor:start;">
        <g>
           <text font-size="40px" x="250" y="40" style="text-anchor:middle;">
           {TIME}
           </text>
           <text font-size="40px" x="250" y="80" style="text-anchor:middle;">
           🌅{SUNRISE}/{SUNSET}🌛
           </text>
           <text font-size="40px" x="10" y="120">
           AQI:
           </text>
           <text text-anchor="end" font-size="40px" x="470" y="120">
           ({AQI_MAIN}) {AQI}
           </text>
           <text font-size="40px" x="10" y="170">
           SCI:
           </text>
           <text text-anchor="end" font-size="40px" x="470" y="170">
           {SCI} {SCHG}
           </text>
           <text font-size="40px" x="10" y="220">
           {TZ1_NAME}:
           </text>
           <text text-anchor="end" font-size="40px" x="470" y="220">
           {TZ1_TIME}
           </text>
           <text font-size="40px" x="10" y="270">
           {TZ2_NAME}:
           </text>
           <text text-anchor="end" font-size="40px" x="470" y="270">
           {TZ2_TIME}
           </text>
           <text font-size="40px" x="10" y="320">
           {FUND0_NAME}
           </text>
           <text text-anchor="start" font-size="40px" x="10" y="370">
           {FUND0_VALUE}
           </text>
           <text text-anchor="end" font-size="40px" x="470" y="370">
           {FUND0_RATE}
           </text>
           <text font-size="40px" x="10" y="420">
           {FUND1_NAME}
           </text>
           <text text-anchor="start" font-size="40px" x="10" y="470">
           {FUND1_VALUE}
           </text>
           <text text-anchor="end" font-size="40px" x="470" y="470">
           {FUND1_RATE}
           </text>
           <text font-size="40px" x="10" y="520">
           {FUND2_NAME}
           </text>
           <text text-anchor="start" font-size="40px" x="10" y="570">
           {FUND2_VALUE}
           </text>
           <text text-anchor="end" font-size="40px" x="470" y="570">
           {FUND2_RATE}
           </text>
        </g>
        <g transform="translate(500, 10)">
            <text font-size="40px" x="10" y="40">
            {DAY0}
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="40">
            {C0}
            </text>
            <text text-anchor="middle" font-size="120px" x="60" y="160">
            {ICON0}
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="110">
            {H0}°C
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="170">
            {L0}°C
            </text>
        </g>
        <g transform="translate(500, 200)">
            <text font-size="40px" x="10" y="40">
            {DAY1}
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="40">
            {C1}
            </text>
            <text text-anchor="middle" font-size="120px" x="60" y="160">
            {ICON1}
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="110">
            {H1}°C
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="170">
            {L1}°C
            </text>
        </g>
        <g transform="translate(500, 390)">
            <text font-size="40px" x="10" y="40">
            {DAY2}
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="40">
            {C2}
            </text>
            <text text-anchor="middle" font-size="120px" x="60" y="160">
            {ICON2}
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="110">
            {H2}°C
            </text>
            <text text-anchor="end" font-size="40px" x="250" y="170">
            {L2}°C
            </text>
        </g>
    </g>
</svg>
"###;
