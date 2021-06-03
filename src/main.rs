extern crate gl;
extern crate imgui;
extern crate imgui_opengl_renderer;
extern crate imgui_sdl2;
extern crate sdl2;

use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use pi::PiCache;

mod gui;
mod pi;

#[tokio::main]
async fn main() {
    // ! MISC GRAPHICS ---
    // Sdl/Opengl setup
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 3);
    }

    let window = video
        .window("Pi", 1000, 1000)
        .position_centered()
        .resizable()
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    let _gl_context = window
        .gl_create_context()
        .expect("Couldn't create GL context");
    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    //  Imgui setup
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    // Event handler setup
    let mut event_pump = sdl_context.event_pump().unwrap();

    // Timing setup
    let mut last_frame = Instant::now();

    // * Tokio runtime setup
    let async_runtime = tokio::runtime::Runtime::new().unwrap();

    // ! MISC GRAPHICS END ---

    // * Ui state setup
    let mut ui_state = gui::State::default();
    ui_state.pi_calc_precision = 2000;

    // * Pi cache setup
    let mut pi_cache: PiCache = PiCache::default();
    pi_cache.calculate(2000);

    let pi_cache_mutex = Arc::new(Mutex::new(pi_cache));
    let pi_cache_arc = Arc::clone(&pi_cache_mutex);

    let mut pi_cache_received_temp = PiCache::default();

    // Main loop
    'running: loop {
        use sdl2::event::Event;
        use sdl2::keyboard::Keycode;

        // * Update --
        for event in event_pump.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);
            if imgui_sdl2.ignore_event(&event) {
                continue;
            }

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Ui state update
        if pi_cache_arc.try_lock().is_ok() {
            pi_cache_received_temp = pi_cache_arc.lock().unwrap().clone();
        }

        let temp_calc_precision = ui_state.pi_calc_precision;

        if ui_state.generation_button_clicked {
            let pi_cache_arc = Arc::clone(&pi_cache_mutex);
            let _locked_pi_cache = pi_cache_arc.lock().unwrap();

            let pi_cache_arc = Arc::clone(&pi_cache_mutex);

            async_runtime.spawn(async move {
                let mut locked_pi_cache = pi_cache_arc.lock().unwrap();

                locked_pi_cache.calculate(temp_calc_precision);
            });

            ui_state.generation_button_clicked = false;
            ui_state.status = gui::MESSAGE_STATUS_GENERATING.to_owned();
        }

        if pi_cache_received_temp.calculated {
            ui_state.status = gui::MESSAGE_STATUS_DONE.to_owned();
            ui_state.current_pi_precision = pi_cache_received_temp.precision;
            ui_state.pi_size_bytes = pi_cache_received_temp.get_size_bytes() as u64;

            pi_cache_received_temp.calculated = false;
        }

        // Update current pi cache with temporary cache
        if pi_cache_arc.try_lock().is_ok() {
            let mut a = pi_cache_arc.lock().unwrap();
            *a = pi_cache_received_temp.clone();
        }

        // * Update end --

        // * Render
        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &event_pump.mouse_state());

        // Calculate delta time
        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;
        imgui.io_mut().delta_time = delta_s;

        // Get temporary ref to ui and fill with content
        let ui = imgui.frame();
        gui::draw_gui(&ui, &mut ui_state);

        // Unsafe gl code
        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Ui render
        imgui_sdl2.prepare_render(&ui, &window);
        renderer.render(ui);

        window.gl_swap_window();
        // * Render end

        //std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    async_runtime.shutdown_background();
}
