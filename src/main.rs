use smithay::reexports::calloop::EventLoop;

mod compositor;
mod config;
mod keybindings;
mod monitor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("DarkBox WM starting...");
    
    let mut event_loop: EventLoop<compositor::DarkBox> = EventLoop::try_new()?;
    let mut compositor = compositor::DarkBox::new(&mut event_loop);
    
    monitor::spawn_system_monitor();
    
    event_loop.run(None, &mut compositor, |_| {})?;
    
    Ok(())
}