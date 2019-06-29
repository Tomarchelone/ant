use ggez::*;
use std::collections::HashSet;
use ggez::{nalgebra as na};
use ggez::graphics::{self, DrawMode, MeshBuilder, Mesh};

const PURPLE: [f32; 4] = [0.4, 0.0, 0.2, 1.0];
const PAPER: [f32; 4] = [0.8, 0.8, 0.6, 1.0];
const RED: [f32; 4] = [0.8, 0.0, 0.0, 1.0];
const BLUE: [f32; 4] = [0.2, 0.0, 0.5, 1.0];
const BLACK: [f32; 4] = [0.05, 0.0, 0.0, 1.0];

pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

pub struct Ant {
    coord: (i64, i64),
    or: Orientation
}

struct Assets {
    black_cell: Mesh,
    white_cell: Mesh,
    right_roads: Mesh,
    left_roads: Mesh,
}

pub struct Screen {
    assets: Assets,
    pix_dim: (f32, f32),
    dim: (i64, i64),
    cell_size: f32,
    center_coord: (i64, i64),
    steps_per_frame: u64,
}

pub enum Stage {
    Initial,
    Run,
}

enum Cells {
    Some(Vec<(i64, i64)>),
    All,
}

pub struct Update {
    cells: Cells,
    ant: bool,
}

pub struct State {
    stage: Stage,
    screen: Screen,
    update: Update,
    board: HashSet<(i64, i64)>,
    ant: Ant,
}

impl State {
    pub fn new(ctx: &mut Context) -> Self {
        let window_resolution = (ctx.conf.window_mode.width, ctx.conf.window_mode.height);
        let pix_dim = (window_resolution.0, window_resolution.1);
        let short_dim = 20;
        let long_dim = (short_dim as f32 * (pix_dim.0 / pix_dim.1)) as i64;
        let dim = (long_dim, short_dim);
        let cell_size = pix_dim.0 / dim.0 as f32;
        let center_coord = (0, 0);

        let black_cell = ggez::graphics::MeshBuilder::new()
        .rectangle(
            DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, cell_size, cell_size),
            BLACK.into(),
        )
        .rectangle(
            DrawMode::stroke(2.0),
            graphics::Rect::new(0.0, 0.0, cell_size, cell_size),
            PURPLE.into(),
        )
        .build(ctx).unwrap();

        let white_cell = ggez::graphics::MeshBuilder::new()
        .rectangle(
            DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, cell_size, cell_size),
            PAPER.into(),
        )
        .rectangle(
            DrawMode::stroke(2.0),
            graphics::Rect::new(0.0, 0.0, cell_size, cell_size),
            PURPLE.into(),
        )
        .build(ctx).unwrap();

        let right_roads = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, cell_size / 2.0),
              na::Point2::new(cell_size / 2.0, 0.0)
            ]
            , 4.0
            , BLUE.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size / 2.0, cell_size),
              na::Point2::new(cell_size, cell_size / 2.0)
            ]
            , 4.0
            , BLUE.into()
        ).unwrap()
        .build(ctx).unwrap();

        let left_roads = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(cell_size / 2.0, 0.0),
              na::Point2::new(cell_size, cell_size / 2.0)
            ]
            , 4.0
            , BLUE.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, cell_size / 2.0),
              na::Point2::new(cell_size / 2.0, cell_size)
            ]
            , 4.0
            , BLUE.into()
        ).unwrap()
        .build(ctx).unwrap();

        State {
            stage: Stage::Initial,
            screen: Screen {
                assets: Assets {
                    black_cell,
                    white_cell,
                    right_roads,
                    left_roads
                }
                , pix_dim
                , dim
                , cell_size
                , center_coord
                , steps_per_frame: 1
            },
            update: Update {
                cells: Cells::All,
                ant: true,
            },
            board: HashSet::new(),
            ant: Ant {
                coord: (0, 0)
                , or: Orientation::Up
            }
        }
    }

    pub fn pix_dim(&self) -> (f32, f32) {
        self.screen.pix_dim
    }

    fn board_to_screen(&self, board_i: i64, board_j: i64) -> (i64, i64) {
        // кординаты центра доски с точки зрения верхнего левого угла
        let (cx, cy) = ((self.screen.dim.0 / 2) as i64, (self.screen.dim.1 / 2) as i64);
        // координаты центра с точки зрения (0, 0) доски
        let (cent_x, cent_y) = self.screen.center_coord;
        (board_i + cx - cent_x, board_j + cy - cent_y)
    }

    fn screen_to_board(&self, screen_i: i64, screen_j: i64) -> (i64, i64) {
        // кординаты центра доски с точки зрения верхнего левого угла
        let (cx, cy) = ((self.screen.dim.0 / 2) as i64, (self.screen.dim.1 / 2) as i64);
        // координаты центра с точки зрения (0, 0) доски
        let (cent_x, cent_y) = self.screen.center_coord;
        (screen_i - cx + cent_x, screen_j - cy + cent_y)
    }

    fn step(&mut self) {
        let (board_i, board_j) = self.ant.coord;
        if self.board.contains(&(board_i, board_j)) {
            match &self.ant.or {
                Orientation::Up => self.ant.or = Orientation::Left,
                Orientation::Left => self.ant.or = Orientation::Down,
                Orientation::Down => self.ant.or = Orientation::Right,
                Orientation::Right => self.ant.or = Orientation::Up,
            }

            self.board.remove(&(board_i, board_j));
        } else {
            match &self.ant.or {
                Orientation::Up => self.ant.or = Orientation::Right,
                Orientation::Left => self.ant.or = Orientation::Up,
                Orientation::Down => self.ant.or = Orientation::Left,
                Orientation::Right => self.ant.or = Orientation::Down,
            }

            self.board.insert((board_i, board_j));
        }

        // шагаем
        match &self.ant.or {
            Orientation::Up => self.ant.coord.1 -= 1,
            Orientation::Left => self.ant.coord.0 -= 1,
            Orientation::Down => self.ant.coord.1 += 1,
            Orientation::Right => self.ant.coord.0 += 1,
        }

        let (new_screen_i, new_screen_j) = self.board_to_screen(self.ant.coord.0, self.ant.coord.1);
        if new_screen_i < 0 {
            self.screen.center_coord.0 -= 1;
            self.update.cells = Cells::All;
        }
        if new_screen_i > self.screen.dim.0 {
            self.screen.center_coord.0 += 1;
            self.update.cells = Cells::All;
        }
        if new_screen_j < 0 {
            self.screen.center_coord.1 -= 1;
            self.update.cells = Cells::All;
        }
        if new_screen_j > self.screen.dim.1 {
            self.screen.center_coord.1 += 1;
            self.update.cells = Cells::All;
        }

        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        match &mut self.update.cells {
            Cells::Some(ref mut v) => {v.push((screen_i, screen_j))},
            _ => {},
        }
        self.update.ant = true;
    }

    fn draw_cell(&self, ctx: &mut Context, screen_i: i64, screen_j: i64) -> GameResult {
        let cell_size = self.screen.cell_size;
        let board_idxes = self.screen_to_board(screen_i, screen_j);
        let (board_i, board_j) = self.screen_to_board(screen_i, screen_j);

        let mut roads_switch: i64 = 0;
        roads_switch += (board_i + board_j) % 2;

        if self.board.contains(&board_idxes) {
            roads_switch += 1;
            graphics::draw(ctx, &self.screen.assets.black_cell, graphics::DrawParam::default()
            .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
        } else {
            graphics::draw(ctx, &self.screen.assets.white_cell, graphics::DrawParam::default()
            .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
        }

        if roads_switch % 2 == 0 {
            graphics::draw(ctx, &self.screen.assets.right_roads, graphics::DrawParam::default()
            .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
        } else {
            graphics::draw(ctx, &self.screen.assets.left_roads, graphics::DrawParam::default()
            .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
        }

        Ok(())
    }

    fn draw_ant(&self, ctx: &mut Context) -> GameResult {
        let cell_size = self.screen.cell_size;
        let (board_i, board_j) = self.ant.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        let mesh = ggez::graphics::MeshBuilder::new()
        .rectangle(
            DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, cell_size, cell_size),
            graphics::WHITE,
        )
        .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size))
        .color(RED.into()))?;

        Ok(())
    }
}

impl ggez::event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 30;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            for _ in 0..self.screen.steps_per_frame {
                self.step();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match self.update.cells {
            Cells::Some(ref cells) => {
                for &(i, j) in cells {
                    self.draw_cell(ctx, i, j)?;
                }

                self.update.cells = Cells::Some(vec!());
            },
            Cells::All => {
                for i in 0..self.screen.dim.0 {
                    for j in 0..self.screen.dim.1 {
                        self.draw_cell(ctx, i, j)?;
                    }
                }
                self.update.cells = Cells::Some(vec!());
            },
        }

        if self.update.ant {
            self.draw_ant(ctx)?;
            self.update.ant = false;
        }


        graphics::present(ctx)?;
        Ok(())
    }


}