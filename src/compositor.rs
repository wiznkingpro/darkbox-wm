use smithay::{
    backend::{
        renderer::glium::GliumRenderer,
        winit::{self, WinitEvent},
    },
    desktop::{Window, WindowSurfaceType},
    reexports::{
        calloop::{EventLoop, RegistrationToken},
        wayland_server::Display,
    },
    utils::{Logical, Point, Size},
    wayland::{
        compositor::CompositorState,
        output::Output,
        seat::{Capability, Seat},
        shell::xdg::XdgShellState,
        shm::init_shm_global,
    },
};
use std::time::Instant;
use tracing::info;

pub struct DarkBox {
    display: Display,
    pub start_time: Instant,
    compositor_state: CompositorState,
    xdg_shell_state: XdgShellState,
    seat: Seat,
    output: Output,
    // окна терминалов
    windows: Vec<Window>,
    // текущий сфокусированный индекс
    focused: usize,
}

impl DarkBox {
    pub fn new(event_loop: &mut EventLoop<Self>) -> Result<Self, Box<dyn std::error::Error>> {
        info!("Initializing Wayland display...");
        let display: Display = Display::new()?;
        let start_time = Instant::now();
        
        // Запуск Winit-бэкенда (для работы в окне/под X11 во время разработки)
        let (renderer, source) = winit::init_from_builder::<GliumRenderer>(
            event_loop,
            winit::WinitGraphicsBackend::<GliumRenderer>::builder()
                .title("darkbox-wm")
        )?;
        let token = event_loop.handle().insert_source(source, |event, _, compositor| {
            if let WinitEvent::Refresh = event {
                compositor.render();
            }
        })?;
        
        // Глобальные объекты Wayland
        init_shm_global(&display, vec![]);
        let compositor_state = CompositorState::new(&display, |surface, _| { /* поверхность создана */ });
        let xdg_shell_state = XdgShellState::new(&display, |_| { /* окно создано */ });
        let seat = Seat::new(&display, "seat0", None);
        seat.add_capability(Capability::Keyboard);
        seat.add_capability(Capability::Pointer);
        let output = Output::new("eDP-1".into(), display.clone());
        output.set_preferred(Size::from((1024, 600))); // размер как у нетбука
        output.add_mode(Size::from((1024, 600)));
        
        // Запуск первого терминала
        let _ = std::process::Command::new("foot")
            .spawn();
        
        Ok(DarkBox {
            display,
            start_time,
            compositor_state,
            xdg_shell_state,
            seat,
            output,
            windows: Vec::new(),
            focused: 0,
        })
    }
    
    fn render(&mut self) {
        // Здесь будет логика рендеринга всех окон + системный монитор
        // Для простоты показываем только чёрный фон
    }
}