use std::collections::LinkedList;
use piston_window::{ Context, G2d };
use piston_window::types::Color;

use crate::draw::draw_block;

const PONG_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
pub enum Direction {
    Left,
    Right,
}
pub struct Block {
    pub x: i32,
    pub y: i32,
}

pub struct Pong {
    pub body: LinkedList<Block>,
}

impl Pong {
    pub fn new(x: i32, y: i32) -> Pong {
        let mut body: LinkedList<Block> = LinkedList::new();
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        Pong { body }
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(PONG_COLOR, block.x, block.y, con, g);
        }
    }
}
