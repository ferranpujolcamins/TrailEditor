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