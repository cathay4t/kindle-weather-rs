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
xmlns="http://www.w3.org/2000/svg" height="768" width="1024" version="1.1"
     xmlns:xlink="http://www.w3.org/1999/xlink">
    <g font-family="DejaVu Sans" style="text-anchor:start;">
        <g transform="translate(10, 10)">
            <g>
               <text font-size="55px" x="300" y="60" style="text-anchor:middle;">
               {TIME}
               </text>
               <text font-size="55px" x="300" y="120" style="text-anchor:middle;">
               🌅{SUNRISE}/{SUNSET}🌛
               </text>
               <text font-size="55px" x="10" y="180">
               AQI:
               </text>
               <text text-anchor="end" font-size="55px" x="600" y="180">
               ({AQI_MAIN}) {AQI}
               </text>
               <text font-size="55px" x="10" y="240">
               SCI:
               </text>
               <text text-anchor="end" font-size="55px" x="600" y="240">
               {SCI} {SCHG}
               </text>
               <text font-size="55px" x="10" y="300">
               {TZ1_NAME}:
               </text>
               <text text-anchor="end" font-size="55px" x="600" y="300">
               {TZ1_TIME}
               </text>
               <text font-size="55px" x="10" y="360">
               {TZ2_NAME}:
               </text>
               <text text-anchor="end" font-size="55px" x="600" y="360">
               {TZ2_TIME}
               </text>
               <text font-size="55px" x="10" y="420">
               {FUND0_NAME}
               </text>
               <text text-anchor="start" font-size="55px" x="10" y="480">
               {FUND0_VALUE}
               </text>
               <text text-anchor="end" font-size="55px" x="600" y="480">
               {FUND0_RATE}
               </text>
               <text font-size="55px" x="10" y="540">
               {FUND1_NAME}
               </text>
               <text text-anchor="start" font-size="55px" x="10" y="600">
               {FUND1_VALUE}
               </text>
               <text text-anchor="end" font-size="55px" x="600" y="600">
               {FUND1_RATE}
               </text>
               <text font-size="55px" x="10" y="660">
               {FUND2_NAME}
               </text>
               <text text-anchor="start" font-size="55px" x="10" y="720">
               {FUND2_VALUE}
               </text>
               <text text-anchor="end" font-size="55px" x="600" y="720">
               {FUND2_RATE}
               </text>
            </g>
        </g>
        <g transform="translate(660, 20)">
            <text font-size="55px" x="10" y="50">
            {DAY0}
            </text>
            <text text-anchor="end" font-size="40px" x="300" y="50">
            {C0}
            </text>
            <text text-anchor="middle" font-size="140px" x="60" y="190">
            {ICON0}
            </text>
            <text text-anchor="end" font-size="55px" x="300" y="120">
            {H0}°C
            </text>
            <text text-anchor="end" font-size="55px" x="300" y="190">
            {L0}°C
            </text>
        </g>
        <g transform="translate(660, 270)">
            <text font-size="55px" x="10" y="50">
            {DAY1}
            </text>
            <text text-anchor="end" font-size="40px" x="300" y="50">
            {C1}
            </text>
            <text text-anchor="middle" font-size="140px" x="60" y="190">
            {ICON1}
            </text>
            <text text-anchor="end" font-size="55px" x="300" y="120">
            {H1}°C
            </text>
            <text text-anchor="end" font-size="55px" x="300" y="190">
            {L1}°C
            </text>
        </g>
        <g transform="translate(660, 520)">
            <text font-size="55px" x="10" y="50">
            {DAY2}
            </text>
            <text text-anchor="end" font-size="40px" x="300" y="50">
            {C2}
            </text>
            <text text-anchor="middle" font-size="140px" x="60" y="190">
            {ICON2}
            </text>
            <text text-anchor="end" font-size="55px" x="300" y="120">
            {H2}°C
            </text>
            <text text-anchor="end" font-size="55px" x="300" y="190">
            {L2}°C
            </text>
        </g>
    </g>
</svg>
"###;
