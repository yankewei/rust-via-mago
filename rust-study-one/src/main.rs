use mago_php_version::PHPVersion;

fn main() {
    let version = PHPVersion::new(8, 4, 0);
    println!("Major version: {}", version.major());
    println!("Minor version: {}", version.minor());
    println!("Patch version: {}", version.patch());
    println!("Version ID: {}", version.to_version_id());
    println!("Version string: {}", version.to_string());
}