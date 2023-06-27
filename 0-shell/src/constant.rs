#[cfg(target_family = "windows")]
pub const USER: &str = "USERNAME";
#[cfg(target_family = "unix")]
pub const USER: &str = "USER";

#[cfg(target_family = "windows")]
pub const HOME: &str = "HOMEPATH";
#[cfg(target_family = "unix")]
pub const HOME: &str = "HOME";
