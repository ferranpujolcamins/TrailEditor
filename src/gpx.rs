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

extern crate gpx;
use crate::model::*;

impl<'a> From<&'a gpx::Waypoint> for WayPoint {
    fn from(item: &'a gpx::Waypoint) -> Self {
        WayPoint {
            elevation: item.elevation,
            coordinates: item.point(),
            time: item.time
        }
    }
}

impl<'a> From<&'a gpx::TrackSegment> for TrackSegment {
    fn from(item: &'a gpx::TrackSegment) -> Self {
        TrackSegment(item.points.
            iter()
            .map(|x: &gpx::Waypoint| WayPoint::from(x))
            .collect()
        )
    }
}

impl<'a> From<&'a gpx::Track> for Track<&'a str> {
    fn from(item: &'a gpx::Track) -> Self {
        Track {
            name: item.name.as_deref(),
            comment: item.comment.as_deref(),
            description: item.description.as_deref(),
            source: item.source.as_deref(),
            number: item.number,
            track_type: item._type.as_deref(),
            segments: item.segments.iter().map(|x: &gpx::TrackSegment| TrackSegment::from(x)).collect()
        }
    }
}