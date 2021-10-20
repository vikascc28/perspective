// extern crate gcc;

fn main() {
    // println!("test");
    println!("cargo:rustc-link-search=native=/Users/v625154/work/perspective/cpp/perspective/dist/release/");
    println!("cargo:rustc-link-lib=static=perspectivecpp");
}