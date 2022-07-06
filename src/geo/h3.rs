use geo_types::{Coordinate, Point};
use h3ron::{Error, H3Cell};

pub fn index(latitude: f64, longitude: f64, resolution: u8) -> Result<H3Cell, Error> {
    let p = Point::new(latitude, longitude);
    H3Cell::from_point(&p, resolution)
    // H3Cell::from_coordinate(&Coordinate { x: latitude, y: longitude }, 8)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_index() {
        println!("{}", super::index(36.0, 117.1, 8).unwrap().to_string());
        println!("{}", super::index(36.0, 117.11, 8).unwrap().to_string());
        println!("{}", super::index(36.0, 117.111, 8).unwrap().to_string());
        println!("{:?}", super::index(36.0, 117.1111, 13).unwrap());
        println!("{:?}", super::index(36.0, 117.11111, 13).unwrap());
    }
}
