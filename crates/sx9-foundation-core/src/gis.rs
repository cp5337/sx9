use geo::{point, Contains, HaversineDistance, Point, Polygon};

pub fn point_m(lat: f64, lon: f64) -> Point { point!(x: lon, y: lat) }

pub fn haversine_m(a: &Point, b: &Point) -> f64 { a.haversine_distance(b) }

pub fn inside(p: &Point, poly: &Polygon) -> bool { poly.contains(p) }
