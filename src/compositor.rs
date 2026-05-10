use smithay::reexports::calloop::EventLoop;

pub struct DarkBox {
    pub start_time: std::time::Instant,
    pub focused: usize,
}

impl DarkBox {
    pub fn new(event_loop: &mut EventLoop<Self>) -> Self {
        println!("Initializing Wayland compositor...");
        
        let start_time = std::time::Instant::now();
        
        // Запуск первого терминала
        let _ = std::process::Command::new("foot")
            .spawn();
        
        DarkBox {
            start_time,
            focused: 0,
        }
    }
}