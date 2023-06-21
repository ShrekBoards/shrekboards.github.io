use shrek_superslam::Console;

/// Turn an integer value from the JS into a Console enum.
pub fn console_from_value(console: i32) -> Console {
    match console {
        0 => Console::Gamecube,
        1 => Console::PC,
        2 => Console::PS2,
        3 => Console::Xbox,
        _ => panic!("uh oh")
    }
}