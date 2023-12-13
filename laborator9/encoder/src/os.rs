#[cfg(target_os = "windows")]
pub fn get_os_name() -> &'static str {
    "Windows"
}
#[cfg(target_os = "linux")]
pub fn get_os_name() -> &'static str {
    "Linux"
}
#[cfg(target_os = "mac")]
pub fn get_os_name() -> &'static str {
    "Mac"
}
