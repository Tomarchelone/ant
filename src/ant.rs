use ggez::*;
use std::collections::HashSet;
use ggez::{nalgebra as na};
use ggez::graphics::{self, DrawMode, Mesh};
use ggez::event::{EventHandler, KeyCode, KeyMods};
use ggez::input::keyboard;

use crate::lib::*;

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
    down_left: Mesh,
    down_right: Mesh,
    up_left: Mesh,
    up_right: Mesh,
    left_down: Mesh,
    left_up: Mesh,
    right_down: Mesh,
    right_up: Mesh,
}

pub struct Screen {
    assets: Assets,
    resolution: (f32, f32),
    dim: (i64, i64),
    cell_size: f32,
    center_coord: (i64, i64),
}

enum Cells {
    Some(Vec<(i64, i64)>),
    All,
}

pub struct Update {
    cells: Cells,
    ant: bool,
}

pub struct AntWalker {
    screen: Screen,
    update: Update,
    board: HashSet<(i64, i64)>,
    ant: Ant,
}

impl AntWalker {
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
        // чётные - приходим сверху-снизу, уходим вправо-влево
        // нечётные - приходим справа-слева, уходим вверх-вниз
        if !self.board.contains(&(board_i, board_j)) {
            // белый
            match self.ant.or {
                Orientation::Up => {
                    graphics::draw(ctx, &self.screen.assets.down_right, graphics::DrawParam::default()
                    .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
                },
                Orientation::Right => {
                    graphics::draw(ctx, &self.screen.assets.left_down, graphics::DrawParam::default()
                    .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
                },
                Orientation::Down => {
                    graphics::draw(ctx, &self.screen.assets.up_left, graphics::DrawParam::default()
                    .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
                },
                Orientation::Left => {
                    graphics::draw(ctx, &self.screen.assets.right_up, graphics::DrawParam::default()
                    .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
                }
            }
        } else {
            // чёрный
            match self.ant.or {
                Orientation::Up => {
                    graphics::draw(ctx, &self.screen.assets.down_left, graphics::DrawParam::default()
                    .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
                },
                Orientation::Right => {
                    graphics::draw(ctx, &self.screen.assets.left_up, graphics::DrawParam::default()
                    .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
                },
                Orientation::Down => {
                    graphics::draw(ctx, &self.screen.assets.up_right, graphics::DrawParam::default()
                    .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
                },
                Orientation::Left => {
                    graphics::draw(ctx, &self.screen.assets.right_down, graphics::DrawParam::default()
                    .dest(na::Point2::new(screen_i as f32 * cell_size, screen_j as f32 * cell_size)))?;
                }
            }
        }

        Ok(())
    }
}

impl Walker for AntWalker {
    fn new(resolution: (f32, f32), dim: i64, ctx: &mut Context) -> Self {
        let long_dim = (dim as f32 * (resolution.0 / resolution.1)) as i64;
        let dim = (long_dim, dim);
        let cell_size = resolution.0 / dim.0 as f32;
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

        let down_left = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(cell_size / 2.0, cell_size),
              na::Point2::new(0.0, cell_size / 2.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, cell_size * 0.5),
              na::Point2::new(cell_size * 0.25, cell_size * 0.5)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, cell_size * 0.5),
              na::Point2::new(0.0, cell_size * 0.75)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let down_right = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(cell_size / 2.0, cell_size),
              na::Point2::new(cell_size, cell_size / 2.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.75, cell_size / 2.0),
              na::Point2::new(cell_size, cell_size / 2.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size, cell_size * 0.75),
              na::Point2::new(cell_size, cell_size / 2.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let up_right = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(cell_size / 2.0, 0.0),
              na::Point2::new(cell_size, cell_size / 2.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size, cell_size * 0.5),
              na::Point2::new(cell_size, cell_size * 0.25)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size, cell_size * 0.5),
              na::Point2::new(cell_size * 0.75, cell_size * 0.5)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let up_left = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(cell_size / 2.0, 0.0),
              na::Point2::new(0.0, cell_size / 2.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, cell_size * 0.5),
              na::Point2::new(0.0, cell_size * 0.25)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, cell_size * 0.5),
              na::Point2::new(cell_size * 0.25, cell_size * 0.5)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let left_down = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, cell_size * 0.5),
              na::Point2::new(cell_size * 0.5, cell_size)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.5, cell_size),
              na::Point2::new(cell_size * 0.25, cell_size)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.5, cell_size),
              na::Point2::new(cell_size * 0.5, cell_size * 0.75)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let left_up = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, cell_size * 0.5),
              na::Point2::new(cell_size * 0.5, 0.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.5, 0.0),
              na::Point2::new(cell_size * 0.25, 0.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.5, 0.0),
              na::Point2::new(cell_size * 0.5, cell_size * 0.25)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let right_down = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(cell_size, cell_size * 0.5),
              na::Point2::new(cell_size * 0.5, cell_size)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.5, cell_size),
              na::Point2::new(cell_size * 0.75, cell_size)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.5, cell_size),
              na::Point2::new(cell_size * 0.5, cell_size * 0.75)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let right_up = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(cell_size, cell_size * 0.5),
              na::Point2::new(cell_size * 0.5, 0.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.5, 0.0),
              na::Point2::new(cell_size * 0.75, 0.0)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(cell_size * 0.5, 0.0),
              na::Point2::new(cell_size * 0.5, cell_size * 0.25)
            ]
            , 4.0
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        AntWalker {
            screen: Screen {
                assets: Assets {
                    black_cell,
                    white_cell,
                    right_roads,
                    left_roads,
                    down_left,
                    down_right,
                    up_left,
                    up_right,
                    left_down,
                    left_up,
                    right_down,
                    right_up,
                }
                , resolution
                , dim
                , cell_size
                , center_coord
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

    fn step(&mut self) {
        let (board_i, board_j) = self.ant.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        match &mut self.update.cells {
            Cells::Some(ref mut v) => v.push((screen_i, screen_j)),
            _ => {},
        }

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

        let (board_i, board_j) = self.ant.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        match &mut self.update.cells {
            Cells::Some(ref mut v) => v.push((screen_i, screen_j)),
            _ => {},
        }

        self.update.ant = true;
    }

    fn step_back(&mut self) {
        let (board_i, board_j) = self.ant.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        match &mut self.update.cells {
            Cells::Some(ref mut v) => {
                v.push((screen_i, screen_j));
                v.push((screen_i - 1, screen_j));
                v.push((screen_i + 1, screen_j));
                v.push((screen_i, screen_j - 1));
                v.push((screen_i, screen_j + 1));
            },
            _ => {},
        }

        // шагаем
        match &self.ant.or {
            Orientation::Up => self.ant.coord.1 += 1,
            Orientation::Left => self.ant.coord.0 += 1,
            Orientation::Down => self.ant.coord.1 -= 1,
            Orientation::Right => self.ant.coord.0 -= 1,
        }

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

        let (board_i, board_j) = self.ant.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        match &mut self.update.cells {
            Cells::Some(ref mut v) => v.push((screen_i, screen_j)),
            _ => {},
        }

        self.update.ant = true;
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
