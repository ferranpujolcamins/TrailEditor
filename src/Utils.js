// Copyright (C) 2021 Ferran Pujol Camins
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

var padTo = function (str, l, s) {
    var n = l - str.length
    for (var i = 0; i < n; ++i) {
        str = s + str
    }
    return str
}

exports.formatGeo = function (posIndicator) {
    return function (negIndicator) {
        return function(gc) {
            var indicator = posIndicator
            if (gc < 0) {
                gc = -gc
                indicator = negIndicator
            }
            var degrees = Math.floor(gc)
            var minutesRaw = (gc - degrees) * 60
            var minutes = Math.floor(minutesRaw)
            var seconds = (minutesRaw - minutes) * 60

            return padTo(degrees.toFixed(0), 3, ' ') + "°" +
                padTo(minutes.toFixed(0), 2, '0') + "'" +
                padTo(seconds.toFixed(4), 7, '0') + '"' + indicator
        }
    }
}
