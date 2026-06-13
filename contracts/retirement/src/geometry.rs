use soroban_sdk::Vec;
use crate::types::Point;

pub fn point_in_polygon(
    point: Point,
    polygon: Vec<Point>,
) -> bool {
    let mut inside = false;
    let n = polygon.len();
    if n < 3 { return false; }
    
    let mut j = n - 1;
    for i in 0..n {
        let pi = polygon.get(i).unwrap();
        let pj = polygon.get(j).unwrap();
        
        if ((pi.lon > point.lon) != (pj.lon > point.lon))
            && (point.lat < (pj.lat - pi.lat) * (point.lon - pi.lon)
                / (pj.lon - pi.lon) + pi.lat)
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}
