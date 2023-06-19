use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Room {
    pub position: Vec2,
    pub w: usize,
    pub h: usize,
}

impl Room {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self {
            position: Vec2::new(x as f32, y as f32),
            w,
            h,
        }
    }

    pub fn center(&self) -> Vec2 {
        let Vec2 { x, y } = self.position;
        let w = self.w as f32;
        let h = self.h as f32;

        Vec2::new(x + w / 2.0, y + h / 2.0)
    }

    pub fn find_edge(&self, edge: Edge) -> Vec2 {
        let mut pos = Vec2::default();

        match edge {
            Edge::Top => {
                pos.x = self.center().x.floor();
                pos.y = self.position.y + self.h as f32;
            }
            Edge::Bottom => {
                pos.x = self.center().x.floor();
                pos.y = self.position.y;
            }
            Edge::Left => {
                pos.y = self.center().y.floor();
                pos.x = self.position.x;
            }
            Edge::Right => {
                pos.y = self.center().y.floor();
                pos.x = self.position.x + self.w as f32;
            }
        }

        pos
    }
}

pub enum Edge {
    Top,
    Left,
    Right,
    Bottom,
}
