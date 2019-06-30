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

const LINE_THICKNESS: f32 = 6.0;

pub enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

pub struct Engineer {
    coord: (i64, i64),
    or: Orientation
}

struct Assets {
    node: Mesh,
    h_line: Mesh,
    v_line: Mesh,
    left: Mesh,
    right: Mesh,
    up: Mesh,
    down: Mesh,
    h_blank: Mesh,
    v_blank: Mesh,
    left_blank: Mesh,
    right_blank: Mesh,
    up_blank: Mesh,
    down_blank: Mesh,
    screen_blank: Mesh,
}

pub struct Screen {
    assets: Assets,
    resolution: (f32, f32),
    dim: (i64, i64),
    bridge_len: f32,
    center_coord: (i64, i64),
}

enum Nodes {
    Some(Vec<(i64, i64)>),
    All,
}

pub struct Update {
    nodes: Nodes,
    engineer: bool,
}

pub struct EngineerWalker {
    screen: Screen,
    update: Update,
    bridges: HashSet<(i64, i64, i64, i64)>,
    engineer: Engineer,
}

impl EngineerWalker {
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

    fn draw_node(&self, ctx: &mut Context, screen_i: i64, screen_j: i64) -> GameResult {
        let (board_i, board_j) = self.screen_to_board(screen_i, screen_j);
        let bridge_len = self.screen.bridge_len;

        // чистим стрелочку
        graphics::draw(ctx, &self.screen.assets.up_blank, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.left_blank, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.down_blank, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.right_blank, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        // чистим 4 моста
        graphics::draw(ctx, &self.screen.assets.h_blank, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.v_blank, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.h_blank, graphics::DrawParam::default()
        .dest(na::Point2::new((screen_i-1) as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.v_blank, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, (screen_j-1) as f32 * bridge_len)))?;

        // перерисовываем все ноды
        graphics::draw(ctx, &self.screen.assets.node, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.node, graphics::DrawParam::default()
        .dest(na::Point2::new((screen_i + 1) as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.node, graphics::DrawParam::default()
        .dest(na::Point2::new((screen_i-1) as f32 * bridge_len, screen_j as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.node, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, (screen_j-1) as f32 * bridge_len)))?;

        graphics::draw(ctx, &self.screen.assets.node, graphics::DrawParam::default()
        .dest(na::Point2::new(screen_i as f32 * bridge_len, (screen_j+1) as f32 * bridge_len)))?;

        // перерисовываем мосты
        if self.bridges.contains(&(board_i-1, board_j, board_i, board_j))
        || self.bridges.contains(&(board_i, board_j, board_i-1, board_j)) {
            graphics::draw(ctx, &self.screen.assets.h_line, graphics::DrawParam::default()
            .dest(na::Point2::new((screen_i-1) as f32 * bridge_len, screen_j as f32 * bridge_len)))?;
        }

        if self.bridges.contains(&(board_i, board_j-1, board_i, board_j))
        || self.bridges.contains(&(board_i, board_j, board_i, board_j-1)) {
            graphics::draw(ctx, &self.screen.assets.v_line, graphics::DrawParam::default()
            .dest(na::Point2::new(screen_i as f32 * bridge_len, (screen_j-1) as f32 * bridge_len)))?;
        }

        if self.bridges.contains(&(board_i, board_j, board_i+1, board_j))
        || self.bridges.contains(&(board_i+1, board_j, board_i, board_j)) {
            graphics::draw(ctx, &self.screen.assets.h_line, graphics::DrawParam::default()
            .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;
        }

        if self.bridges.contains(&(board_i, board_j, board_i, board_j+1))
        || self.bridges.contains(&(board_i, board_j+1, board_i, board_j)) {
            graphics::draw(ctx, &self.screen.assets.v_line, graphics::DrawParam::default()
            .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;
        }

        Ok(())
    }

    fn draw_engineer(&self, ctx: &mut Context) -> GameResult {
        let (board_i, board_j) = self.engineer.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        let bridge_len = self.screen.bridge_len;

        match self.engineer.or {
            Orientation::Up => {
                graphics::draw(ctx, &self.screen.assets.up, graphics::DrawParam::default()
                .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;
            },
            Orientation::Right => {
                graphics::draw(ctx, &self.screen.assets.right, graphics::DrawParam::default()
                .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;
            },
            Orientation::Down => {
                graphics::draw(ctx, &self.screen.assets.down, graphics::DrawParam::default()
                .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;
            },
            Orientation::Left => {
                graphics::draw(ctx, &self.screen.assets.left, graphics::DrawParam::default()
                .dest(na::Point2::new(screen_i as f32 * bridge_len, screen_j as f32 * bridge_len)))?;
            }
        }

        Ok(())
    }
}

impl Walker for EngineerWalker {
    fn new(resolution: (f32, f32), dim: i64, ctx: &mut Context) -> Self {
        let long_dim = (dim as f32 * (resolution.0 / resolution.1)) as i64;
        let dim = (long_dim, dim);
        let bridge_len = resolution.0 / dim.0 as f32; // !
        let center_coord = (0, 0);

        let node = ggez::graphics::MeshBuilder::new()
        .circle(
            DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            8.0,
            4.,
            BLACK.into()
        )
        .build(ctx).unwrap();

        let h_line = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(bridge_len, 0.0)
            ]
            , LINE_THICKNESS
            , BLACK.into()
        ).unwrap()
        .build(ctx).unwrap();

        let v_line = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(0.0, bridge_len)
            ]
            , LINE_THICKNESS
            , BLACK.into()
        ).unwrap()
        .build(ctx).unwrap();

        let up = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(0.0, -bridge_len * 0.5)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, -bridge_len * 0.5),
              na::Point2::new(-bridge_len * 0.25, -bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, -bridge_len * 0.5),
              na::Point2::new(bridge_len * 0.25, -bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let down = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(0.0, bridge_len * 0.5)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, bridge_len * 0.5),
              na::Point2::new(-bridge_len * 0.25, bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, bridge_len * 0.5),
              na::Point2::new(bridge_len * 0.25, bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let left = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(-bridge_len * 0.5, 0.0)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(-bridge_len * 0.5, 0.0),
              na::Point2::new(-bridge_len * 0.25, bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(-bridge_len * 0.5, 0.0),
              na::Point2::new(-bridge_len * 0.25, -bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let right = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(bridge_len * 0.5, 0.0)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(bridge_len * 0.5, 0.0),
              na::Point2::new(bridge_len * 0.25, bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .line(
            &[na::Point2::new(bridge_len * 0.5, 0.0),
              na::Point2::new(bridge_len * 0.25, -bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , RED.into()
        ).unwrap()
        .build(ctx).unwrap();

        let h_blank = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(bridge_len, 0.0)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .build(ctx).unwrap();

        let v_blank = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(0.0, bridge_len)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .build(ctx).unwrap();

        let up_blank = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(0.0, -bridge_len * 0.5)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, -bridge_len * 0.5),
              na::Point2::new(-bridge_len * 0.25, -bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, -bridge_len * 0.5),
              na::Point2::new(bridge_len * 0.25, -bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .build(ctx).unwrap();

        let down_blank = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(0.0, bridge_len * 0.5)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, bridge_len * 0.5),
              na::Point2::new(-bridge_len * 0.25, bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .line(
            &[na::Point2::new(0.0, bridge_len * 0.5),
              na::Point2::new(bridge_len * 0.25, bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .build(ctx).unwrap();

        let left_blank = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(-bridge_len * 0.5, 0.0)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .line(
            &[na::Point2::new(-bridge_len * 0.5, 0.0),
              na::Point2::new(-bridge_len * 0.25, bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .line(
            &[na::Point2::new(-bridge_len * 0.5, 0.0),
              na::Point2::new(-bridge_len * 0.25, -bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .build(ctx).unwrap();

        let right_blank = ggez::graphics::MeshBuilder::new()
        .line(
            &[na::Point2::new(0.0, 0.0),
              na::Point2::new(bridge_len * 0.5, 0.0)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .line(
            &[na::Point2::new(bridge_len * 0.5, 0.0),
              na::Point2::new(bridge_len * 0.25, bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .line(
            &[na::Point2::new(bridge_len * 0.5, 0.0),
              na::Point2::new(bridge_len * 0.25, -bridge_len * 0.25)
            ]
            , LINE_THICKNESS
            , PAPER.into()
        ).unwrap()
        .build(ctx).unwrap();

        let screen_blank = ggez::graphics::MeshBuilder::new()
        .rectangle(
            DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, resolution.0, resolution.1),
            PAPER.into(),
        )
        .build(ctx).unwrap();

        EngineerWalker {
            screen: Screen {
                assets: Assets {
                    node,
                    h_line,
                    v_line,
                    left,
                    right,
                    up,
                    down,
                    h_blank,
                    v_blank,
                    left_blank,
                    right_blank,
                    up_blank,
                    down_blank,
                    screen_blank,
                }
                , resolution
                , dim
                , bridge_len
                , center_coord
            },
            update: Update {
                nodes: Nodes::All,
                engineer: true,
            },
            bridges: HashSet::new(),
            engineer: Engineer {
                coord: (0, 0)
                , or: Orientation::Up
            }
        }
    }

    fn step(&mut self) {
        let (board_i, board_j) = self.engineer.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        match &mut self.update.nodes {
            Nodes::Some(ref mut v) => {
                v.push((screen_i, screen_j));
                v.push((screen_i-1, screen_j));
                v.push((screen_i, screen_j-1));
            },
            _ => {},
        }

        let opposite_node = match self.engineer.or {
            Orientation::Up => {
                (board_i, board_j - 1)
            },
            Orientation::Left => {
                (board_i - 1, board_j)
            },
            Orientation::Down => {
                (board_i, board_j + 1)
            },
            Orientation::Right => {
                (board_i + 1, board_j)
            }
        };

        if self.bridges.contains(&(board_i, board_j, opposite_node.0, opposite_node.1))
        || self.bridges.contains(&(opposite_node.0, opposite_node.1, board_i, board_j)) {
            self.bridges.remove(&(board_i, board_j, opposite_node.0, opposite_node.1));
            self.bridges.remove(&(opposite_node.0, opposite_node.1, board_i, board_j));

            self.engineer.coord = opposite_node;
            self.engineer.or = match self.engineer.or {
                Orientation::Up => Orientation::Left,
                Orientation::Left => Orientation::Down,
                Orientation::Down => Orientation::Right,
                Orientation::Right => Orientation::Up,
            }
        } else {
            self.bridges.insert((board_i, board_j, opposite_node.0, opposite_node.1));

            self.engineer.or = match self.engineer.or {
                Orientation::Up => Orientation::Right,
                Orientation::Right => Orientation::Down,
                Orientation::Down => Orientation::Left,
                Orientation::Left => Orientation::Up,
            }
        }



        let (new_screen_i, new_screen_j) = self.board_to_screen(self.engineer.coord.0, self.engineer.coord.1);
        if new_screen_i < 0 {
            self.screen.center_coord.0 -= 1;
            self.update.nodes = Nodes::All;
        }
        if new_screen_i > self.screen.dim.0 {
            self.screen.center_coord.0 += 1;
            self.update.nodes = Nodes::All;
        }
        if new_screen_j < 0 {
            self.screen.center_coord.1 -= 1;
            self.update.nodes = Nodes::All;
        }
        if new_screen_j > self.screen.dim.1 {
            self.screen.center_coord.1 += 1;
            self.update.nodes = Nodes::All;
        }

        let (board_i, board_j) = self.engineer.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        match &mut self.update.nodes {
            Nodes::Some(ref mut v) => v.push((screen_i, screen_j)),
            _ => {},
        }

        self.update.engineer = true;
    }

    fn step_back(&mut self) {
        /*let (board_i, board_j) = self.engineer.coord;
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
        match &self.engineer.or {
            Orientation::Up => self.engineer.coord.1 += 1,
            Orientation::Left => self.engineer.coord.0 += 1,
            Orientation::Down => self.engineer.coord.1 -= 1,
            Orientation::Right => self.engineer.coord.0 -= 1,
        }

        let (board_i, board_j) = self.engineer.coord;
        if self.board.contains(&(board_i, board_j)) {
            match &self.engineer.or {
                Orientation::Up => self.engineer.or = Orientation::Left,
                Orientation::Left => self.engineer.or = Orientation::Down,
                Orientation::Down => self.engineer.or = Orientation::Right,
                Orientation::Right => self.engineer.or = Orientation::Up,
            }

            self.board.remove(&(board_i, board_j));
        } else {
            match &self.engineer.or {
                Orientation::Up => self.engineer.or = Orientation::Right,
                Orientation::Left => self.engineer.or = Orientation::Up,
                Orientation::Down => self.engineer.or = Orientation::Left,
                Orientation::Right => self.engineer.or = Orientation::Down,
            }

            self.board.insert((board_i, board_j));
        }

        let (new_screen_i, new_screen_j) = self.board_to_screen(self.engineer.coord.0, self.engineer.coord.1);
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

        let (board_i, board_j) = self.engineer.coord;
        let (screen_i, screen_j) = self.board_to_screen(board_i, board_j);
        match &mut self.update.cells {
            Cells::Some(ref mut v) => v.push((screen_i, screen_j)),
            _ => {},
        }

        self.update.engineer = true;*/
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match self.update.nodes {
            Nodes::Some(ref nodes) => {
                for &(i, j) in nodes {
                    self.draw_node(ctx, i, j)?;
                }

                self.update.nodes = Nodes::Some(vec!());
            },
            Nodes::All => {
                graphics::draw(ctx, &self.screen.assets.screen_blank, graphics::DrawParam::default()
                .dest(na::Point2::new(0., 0.)))?;
                for i in 0..self.screen.dim.0 {
                    for j in 0..self.screen.dim.1 {
                        self.draw_node(ctx, i, j)?;
                    }
                }
                self.update.nodes = Nodes::Some(vec!());
            },
        }

        if self.update.engineer {
            self.draw_engineer(ctx)?;
            self.update.engineer = false;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
