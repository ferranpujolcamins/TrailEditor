// Copyright (C) 2021 Ferran Pujol CAmins
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

use std::ops::Deref;
use geo_types::Point;
use chrono::{DateTime, Utc};

pub struct Track<StringType: Deref<Target = str>> {
    pub name: Option<StringType>,
    pub comment: Option<StringType>,
    pub description: Option<StringType>,
    pub source: Option<StringType>,
    // TODO: links,
    pub number: Option<u32>,
    pub track_type: Option<StringType>,
    pub segments: Vec<TrackSegment>

    // all of this are optional
}

pub struct TrackSegment(pub Vec<WayPoint>);

pub struct WayPoint {
    pub elevation: Option<f64>,
    pub coordinates: Point<f64>,
    pub time: Option<DateTime<Utc>>
    // TODO...
}