use imgui::*;

use std::time::Duration;

pub const MESSAGE_STATUS_GENERATING: &str = "Generating...";
pub const MESSAGE_STATUS_DONE: &str = "Ready";

/// Main GUI draw function
pub fn draw_gui<'a>(ui: &Ui<'a>, state: &mut State) {
    draw_generation_window(ui, state);
    draw_info_window(ui, state);
}

fn draw_generation_window(ui: &Ui, state: &mut State) {
    let window = Window::new(im_str!("Generation"))
        .size([300.0, 150.0], Condition::Always)
        .position([500.0, 100.0], Condition::Appearing);

    window.build(&ui, || {
        ui.text_wrapped(im_str!("Please specify the digits of precision. Beware that large values may significantly slow down the generation."));

        ui.separator();

        Slider::new(im_str!("Precision"))
            .range(100..= 10000000)
            .build(&ui, &mut state.pi_calc_precision);

        ui.separator();

        state.generation_button_clicked = ui.button(im_str!("Generate"), [75.0, 25.0]);
    });
}

fn draw_info_window(ui: &Ui, state: &mut State) {
    use num_format::{Locale, ToFormattedString};
    use pretty_bytes::converter::convert;

    let window = Window::new(im_str!("Info"))
        .size([200.0, 350.0], Condition::Always)
        .position([0.0, 0.0], Condition::Always);

    window.build(&ui, || {
        ui.text(im_str!("Status: {}", state.status));
        ui.text(im_str!(
            "Digits: {}",
            state.current_pi_precision.to_formatted_string(&Locale::en)
        ));
        ui.text(im_str!("Size: {}", convert(state.pi_size_bytes as f64)));
    });
}

#[derive(Default, Debug, Clone)]
pub struct State {
    pub update_time: Duration,
    pub render_time: Duration,
    pub vert_build_time: Duration,
    pub total_frame_time: Duration,

    pub pi_calc_precision: u32,
    pub current_pi_precision: u32,
    pub pi_size_bytes: u64,

    pub generation_button_clicked: bool,
    pub generation_finished: bool,
    pub status: String,
    pub debug: bool,
}

impl State {}
