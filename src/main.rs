extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use std::thread;


pub fn main() {

    thread::spawn(|| {
        consumer();
    });
    thread::spawn(|| {
        producer();
    });

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    // This handles the events from keyboard
    let mut event_pump = sdl_context.event_pump().unwrap();

    struct Player {
        x: i32,
        y: i32,
        width: u32,
        height: u32
    };
    let mut player = Player {
        x: 50,
        y: 50,
        width: 20,
        height: 20
    };

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
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    println!("Down");
                    player.y += 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    println!("Left");
                    player.x -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    println!("Right");
                    player.x += 10;
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

pub fn consumer(){
    println!("Initialing ZeroMQ server ...");
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();

    assert!(responder.bind("tcp://*:5555").is_ok());

    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("## SERVER ## Message received from client = {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("World", 0).unwrap();
    }
}

pub fn producer() {
    println!("Connecting to Server...\n");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..10 {
        let message = "Hello";
        println!("## CLIENT ## Sending to server message = {}", message);
        requester.send(message, 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("## CLIENT ## Response from server = {}\n", msg.as_str().unwrap());
    }
}