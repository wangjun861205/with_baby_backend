use libh3_sys::{degsToRads, geoToH3, GeoCoord};

pub fn index(latitude: f64, longitude: f64, resolution: i32) -> String {
    unsafe {
        let lat = degsToRads(latitude);
        let lon = degsToRads(longitude);
        format!("{:016x}", geoToH3(&GeoCoord { lat, lon }, resolution))
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_index() {
        println!("{}", super::index(36.0, 117.1, 8));
        println!("{}", super::index(36.0, 117.11, 8));
        println!("{}", super::index(36.0, 117.111, 8));
        println!("{}", super::index(36.0, 117.1111, 15));
        println!("{}", super::index(36.0, 117.11111, 15));
        println!("{}", super::index(36.0, 117.111111, 15));
        println!("{}", super::index(36.0, 117.1111111, 15));
    }
}
