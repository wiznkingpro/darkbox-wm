use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

pub fn spawn_system_monitor() -> Result<(), Box<dyn std::error::Error>> {
    // Запускаем легковесный монитор (btm или самодельный)
    let mut child = Command::new("btm")
        .arg("--basic")
        .stdout(Stdio::piped())
        .spawn()?;
    
    // В будущем здесь будет вывод на экран через текстовый слой
    Ok(())
}