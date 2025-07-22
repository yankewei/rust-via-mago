use mago_php_version::PHPVersion;
use std::str::FromStr;

fn main() {
    // let version = PHPVersion::new(8, 4, 0);
    let version = PHPVersion::from_str("8.4.0").unwrap();
    println!("Major version: {}", version.major());
    println!("Minor version: {}", version.minor());
    println!("Patch version: {}", version.patch());
    println!("Version ID: {}", version.to_version_id());
    println!("Version string: {}", version.to_string());
}
