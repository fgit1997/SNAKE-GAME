extern crate piston_window;
extern crate rand;

mod draw;
mod snake;
mod game;
extern crate winapi;
extern crate user32;
use std::fs::File;
use std::io::prelude::*;
use std::ptr::null_mut as NULL;
use winapi::um::winuser;
use piston_window::*;
use piston_window::types::Color;

use crate::game::Game;
use crate::draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];



fn main() {
    
    let (width, height) = (20, 20);

    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
// MessageBox
let mut file = File::open("arp.txt")
.expect("File not found");
let mut data = String::new();
file.read_to_string(&mut data)
.expect("Error while reading file");

let p_msg:Vec<u16> =data.encode_utf16().collect();
let p_title: Vec<u16> = "Highest Scores".encode_utf16().collect();
unsafe {
let p_result =  user32::MessageBoxW(NULL(), p_msg.as_ptr(), p_title.as_ptr(), winuser::MB_OK | winuser::MB_ICONINFORMATION);
if p_result==1{
    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt);
            
            
        });
    }
}
}


   
}
