extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use std::thread;
use rustc_serialize::json;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};


#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Player {
    x: i32,
    y: i32,
    width: u32,
    height: u32
}

pub fn game_loop(mut player: Player){
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
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
        .unwrap_or_else(|error|{
            panic!("ERROR: {:?}", error);
        });

        // Capture events from keyboard
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    println!("Up");
                    player.y -= 10;
                    producer(&player);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    println!("Down");
                    player.y += 10;
                    producer(&player);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    println!("Left");
                    player.x -= 10;
                    producer(&player);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    println!("Right");
                    player.x += 10;
                    producer(&player);
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        // DRAW CANVAS ON WINDOW
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main() {
    let (tx, rx): (Sender<Player>, Receiver<Player>) = mpsc::channel();
    let sender = tx.clone();
    thread::spawn(|| {
        server(sender);
    });

    let player: Player = Player {
        x: 50,
        y: 50,
        width: 20,
        height: 20
    };
    producer(&player);
    thread::spawn(move || {
        game_loop(player);
    });

     // Received from thread
    loop {
        println!(" ");
        let received: Player = rx.recv().unwrap();
        println!("Receiving another thread: {:?}", received);
    }
}

pub fn server(tx: mpsc::Sender<Player>,){
    println!("Initialing ZeroMQ server ...");
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();

        let decoded: Player = json::decode(msg.as_str().unwrap()).unwrap();
        println!("## SERVER ## Message received from client = {:?}", decoded);
        tx.send(decoded).unwrap();
        thread::sleep(Duration::from_millis(1000));
        responder.send("OK", 0).unwrap();
    }
}

pub fn producer(player: &Player) {
    let mut msg = zmq::Message::new();
    let encoded = json::encode(player).unwrap();

    thread::spawn(move || {
        println!("Connecting to Server...\n");

        let context = zmq::Context::new();
        let requester = context.socket(zmq::REQ).unwrap();

        assert!(requester.connect("tcp://localhost:5555").is_ok());
        println!("## CLIENT ## Sending to server message = {}", encoded);
        requester.send(encoded.as_str(), 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("## CLIENT ## Response from server = {}\n", msg.as_str().unwrap());
    });
}