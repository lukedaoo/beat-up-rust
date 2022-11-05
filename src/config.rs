pub struct Config {
    pub width: i32,
    pub height: i32,
}
impl Default for Config {
    fn default() -> Self {
        Config {
            width: 1920,
            height: 800,
        }
    }
}
