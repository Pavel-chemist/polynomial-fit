// common functions to be used in multiple modules
pub struct PixCoord {
    pub x: f64,
    pub y: f64,
}

pub fn exp (n: f64, e: i32) -> f64 {
    let mut res: f64 = 1.0;
    
    if e >= 0 {
        for _i in 0..e {
            res = res * n;
        }
    } else {
        for _i in 0..(-e) {
            res = res / n;
        }
    }
    
    return res;
}