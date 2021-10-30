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