use ggez::*;
use ggez::conf::{FullscreenType};

pub mod lib_ant;

use lib_ant::*;

pub fn main() {
    // hardcoded for now TODO: get it from somewhere!
    let screen_resolution = (1920.0, 1080.0);

    let mut c = conf::Conf::new();
    let mut window_mode = c.window_mode;

    let (w, h) = (screen_resolution.0, screen_resolution.1);;
    window_mode.width = w;
    window_mode.height = h;
    window_mode.fullscreen_type = FullscreenType::True;
    c = c.window_mode(window_mode);
    //c.window_setup.vsync = true;

    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("Ant", "Tomarchelone")
        .conf(c).build().unwrap();
    let state = &mut State::new(ctx);

    event::run(ctx, event_loop, state).unwrap();

}
