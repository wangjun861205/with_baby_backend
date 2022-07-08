use libh3_sys::{degsToRads, geoToH3, kRing, GeoCoord, H3Index};

pub fn index(latitude: f64, longitude: f64, resolution: i32) -> H3Index {
    unsafe {
        let lat = degsToRads(latitude);
        let lon = degsToRads(longitude);
        geoToH3(&GeoCoord { lat, lon }, resolution)
    }
}

fn vicinity_factor(k: i32) -> usize {
    if k == 0 {
        return 0;
    }
    vicinity_factor(k - 1) + k as usize
}

fn vicinity_number(k: i32) -> usize {
    vicinity_factor(k) * 6 + 1
}

pub fn k_ring(origin: H3Index, k: i32) -> Vec<u64> {
    unsafe {
        let mut out: Vec<u64> = vec![0; vicinity_number(k)];
        kRing(origin, k, (&mut out[0]) as *mut u64);
        return out;
    }
}

pub fn k_ring_from_lat_lng(latitude: f64, longitude: f64, resolution: i32, k: i32) -> Vec<u64> {
    let ori = index(latitude, longitude, resolution);
    k_ring(ori, k)
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

    // 1, 3, 6, 10, 15, 21, 28

    #[test]
    fn test_k_ring() {
        let index = super::index(36.1111, 117.1111, 13);
        println!("{:?}", super::k_ring(index, 1).into_iter().filter(|v| v > &0).count());
        println!("{:?}", super::k_ring(index, 2).into_iter().filter(|v| v > &0).count());
        println!("{:?}", super::k_ring(index, 3).into_iter().filter(|v| v > &0).count());
        println!("{:?}", super::k_ring(index, 4).into_iter().filter(|v| v > &0).count());
        println!("{:?}", super::k_ring(index, 5).into_iter().filter(|v| v > &0).count());
        println!("{:?}", super::k_ring(index, 6).into_iter().filter(|v| v > &0).count());
        println!("{:?}", super::k_ring(index, 7).into_iter().filter(|v| v > &0).count());
    }

    #[test]
    fn test_vicinity_number() {
        assert!(super::vicinity_number(0) == 1);
        assert!(super::vicinity_number(1) == 7);
        assert!(super::vicinity_number(2) == 19);
        assert!(super::vicinity_number(3) == 37);
        assert!(super::vicinity_number(4) == 61);
        assert!(super::vicinity_number(5) == 91);
        assert!(super::vicinity_number(6) == 127);
        assert!(super::vicinity_number(7) == 169);
    }
}
