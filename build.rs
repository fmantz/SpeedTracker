fn main() {
    println!("cargo:rustc-link-search=./SpeedTest/");
    println!("cargo:rustc-link-lib=speedtest")
}