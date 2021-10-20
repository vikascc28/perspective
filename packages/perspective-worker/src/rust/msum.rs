extern "C" {
    fn cadd(x: i32, y: i32) -> i32;
}

#[no_mangle]
pub  extern "C" fn rsum(x: i32, y: i32, z: i32) -> i32 {
    x + unsafe { cadd(y, z) }
}

// pub fn main() {

// }