use ggez::*;
use std::collections::HashSet;
use ggez::{nalgebra as na};
use ggez::graphics::{self, DrawMode, Mesh};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::input::keyboard;


pub trait Walker {
    fn new(resolution: (f32, f32), dim: i64, ctx: &mut Context) -> Self;

    fn step(&mut self);

    fn step_back(&mut self);

    fn draw(&mut self, ctx: &mut Context) -> GameResult;
}

pub enum Mode {
    Stream(u64),
    StepByStep,
}

pub struct Buttons {
    space: bool,
    right: bool,
    left: bool,
}

pub struct State<W: Walker> {
    mode: Mode,
    buttons: Buttons,
    resolution: (f32, f32),
    walker: W,
}


impl<W: Walker> State<W> {
    pub fn new(ctx: &mut Context, dim: i64) -> Self {
        let resolution = (ctx.conf.window_mode.width, ctx.conf.window_mode.height);

        State {
            mode: Mode::StepByStep,
            buttons: Buttons {
                space: false,
                right: false,
                left: false,
            },
            resolution,
            walker: W::new(resolution, dim, ctx),
        }
    }
}

impl<W: Walker> ggez::event::EventHandler for State<W> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 30;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            if !keyboard::is_key_pressed(ctx, KeyCode::Space) && self.buttons.space {
                self.buttons.space = false;
            }

            if !keyboard::is_key_pressed(ctx, KeyCode::Right) && self.buttons.right {
                self.buttons.right = false;
            }

            if !keyboard::is_key_pressed(ctx, KeyCode::Left) && self.buttons.left {
                self.buttons.left = false;
            }

            match self.mode {
                Mode::Stream(steps_per_frame) => {
                    for _ in 0..steps_per_frame {
                        self.walker.step();
                    }

                    if keyboard::is_key_pressed(ctx, KeyCode::Space) && !self.buttons.space {
                        self.buttons.space = true;
                        self.mode = Mode::StepByStep;
                    }
                },
                Mode::StepByStep => {
                    if keyboard::is_key_pressed(ctx, KeyCode::Right) && !self.buttons.right {
                        self.buttons.right = true;
                        self.walker.step();
                    }

                    if keyboard::is_key_pressed(ctx, KeyCode::Left) && !self.buttons.left {
                        self.buttons.left = true;
                        self.walker.step_back();
                    }

                    if keyboard::is_key_pressed(ctx, KeyCode::Space) && !self.buttons.space {
                        self.buttons.space = true;
                        self.mode = Mode::Stream(10);
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.walker.draw(ctx);

        Ok(())
    }
}
