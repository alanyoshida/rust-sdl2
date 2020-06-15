extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::env;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

mod client;
mod server;
mod player;

use client::{Client};
use server::{Server};
use crate::player::{Player};

pub fn main() {
  // Preparing Channels
  let (tx, rx): (Sender<Player>, Receiver<Player>) = mpsc::channel();
  let mut client: Client = Client {
    server_address: String::from("localhost"),
    server_port: String::from("5555"),
  };

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      if args[1] == "client" {
        println!(
          "Executing as client\n Connecting to {} in port {} !",
          client.server_address, client.server_port
        );
      }
    }
    3 => {
      if args[1] == "client" {
        client.server_address = String::from(&args[2]);
        println!(
          "Executing as client\n Connecting to {} in port {} !",
          client.server_address, client.server_port
        );
      }
    }
    4 => {
      if args[1] == "client" {
        client.server_address = String::from(&args[2]);
        client.server_port = String::from(&args[3]);
        println!(
          "Executing as client\n Connecting to {} in port {} !",
          client.server_address, client.server_port
        );
      }
    }
    _ => {
      println!("Executing with embeded server !");
      let sender = tx.clone();

      thread::spawn(|| {
        let server: Server = Server::new();
        server.start(sender);
      });
    }
  }

  thread::spawn(move || {
    client.send_to_server(rx);
  });

  // Player Initial State
  let player: Player = Player {
    x: 50,
    y: 50,
    width: 20,
    height: 20,
  };

  producer(&player);

  thread::spawn(move || {
    game_loop(player);
  });
}

pub fn producer(_player: &Player) {
  // CHANNEL SEND
}

pub fn game_loop(mut player: Player) {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();

  let window = video_subsystem
    .window("rust-sdl2 demo", 800, 600)
    .position_centered()
    .build()
    .unwrap();

  let mut canvas = window.into_canvas().build().unwrap();

  // This handles the events from keyboard
  let mut event_pump = sdl_context.event_pump().unwrap();

  // GAME LOOP
  'running: loop {
    // CLEAR WINDOW EACH FRAME
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw player
    canvas.set_draw_color(Color::RGB(255, 210, 0));
    canvas
      .fill_rect(Rect::new(player.x, player.y, player.width, player.height))
      .unwrap_or_else(|error| {
        panic!("ERROR: {:?}", error);
      });

    // Capture events from keyboard
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        Event::KeyDown {
          keycode: Some(Keycode::Up),
          ..
        } => {
          println!("Up");
          player.y -= 10;
          producer(&player);
        }
        Event::KeyDown {
          keycode: Some(Keycode::Down),
          ..
        } => {
          println!("Down");
          player.y += 10;
          producer(&player);
        }
        Event::KeyDown {
          keycode: Some(Keycode::Left),
          ..
        } => {
          println!("Left");
          player.x -= 10;
          producer(&player);
        }
        Event::KeyDown {
          keycode: Some(Keycode::Right),
          ..
        } => {
          println!("Right");
          player.x += 10;
          producer(&player);
        }
        _ => {}
      }
    }
    // The rest of the game loop goes here...

    // DRAW CANVAS ON WINDOW
    canvas.present();
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
  }
}
