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

pub static KINDLE_WEATHER_SVG: &str = r###"<svg xmlns="http://www.w3.org/2000/svg" height="600" width="800" version="1.1"
     xmlns:xlink="http://www.w3.org/1999/xlink">
    <g font-family="DejaVu Sans" style="text-anchor:middle;">
        <g>
           <text font-size="30px" x="200" y="40">
           {TIME}
           </text>
           <text font-size="30px" x="200" y="90">
           AQI: {AQI} ({AQI_MAIN})
           </text>
           <text font-size="30px" x="200" y="140">
           SCI: {SCI} {SCHG}
           </text>
        </g>
        <g transform="translate(500, 0)">
            <text font-size="30px" x="150" y="25">
            {D0}
            </text>
            <text font-size="40px" x="10" y="80" style="text-anchor:start;">
            {C0}
            </text>
            <text font-size="40px" x="10" y="140" style="text-anchor:start;">
            {H0}°C
            </text>
            <text font-size="40px" x="150" y="140" style="text-anchor:start;">
            {L0}°C
            </text>
        </g>
        <g transform="translate(500, 170)">
            <text font-size="30px" x="150" y="25">
            {D1}
            </text>
            <text font-size="40px" x="10" y="80" style="text-anchor:start;">
            {C1}
            </text>
            <text font-size="40px" x="10" y="140" style="text-anchor:start;">
            {H1}°C
            </text>
            <text font-size="40px" x="150" y="140" style="text-anchor:start;">
            {L1}°C
            </text>
        </g>
        <g transform="translate(500, 340)">
            <text font-size="30px" x="150" y="25">
            {D2}
            </text>
            <text font-size="40px" x="10" y="80" style="text-anchor:start;">
            {C2}
            </text>
            <text font-size="40px" x="10" y="140" style="text-anchor:start;">
            {H2}°C
            </text>
            <text font-size="40px" x="150" y="140" style="text-anchor:start;">
            {L2}°C
            </text>
        </g>
    </g>
</svg>
"###;
