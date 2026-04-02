use pixels::{Pixels, SurfaceTexture};
use std::io::{self, Write};
use std::net::SocketAddr;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use common::font::draw_text;

const WIDTH: u32  = 800;
const HEIGHT: u32 = 800;

fn prompt(msg: &str) -> String {
    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        if !trimmed.is_empty() {
            return trimmed.to_string();
        }

        println!("Input cannot be empty");
    }
}

fn prompt_addr() -> SocketAddr {
    loop {
        let raw = prompt("Enter IP Address: ");
        if let Ok(addr) = raw.parse() {
            return addr;
        }
        println!("Invalid address, expected format 192.168.1.1:34254");
    }
}

fn prompt_username() -> String {
    loop {
        let name = prompt("Enter Name: ");
        if name.len() >= 4 && name.len() <= 16 && name.chars().all(|c| c.is_alphanumeric()) {
            return name;
        }
        println!("Invalid username, must be alphanumeric and between 4 and 16 characters");
    }
}

fn main() {
    let server_addr = prompt_addr();
    let username    = prompt_username();
    println!("Starting...");

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Maze War")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let mut pixels = Pixels::new(WIDTH, HEIGHT, SurfaceTexture::new(WIDTH, HEIGHT, &window)).unwrap();

    let mut frames = 0;
    let mut last   = Instant::now();
    let mut fps    = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => window.request_redraw(),

            Event::RedrawRequested(_) => {
                frames += 1;
                if last.elapsed().as_secs_f32() >= 1.0 {
                    fps    = frames;
                    frames = 0;
                    last   = Instant::now();
                }

                let frame = pixels.frame_mut();
                frame.fill(0);
                draw_text(frame, WIDTH as usize, &format!("FPS:{}", fps), 4, 4, [255, 255, 255, 255], 1);

                pixels.render().unwrap();
            }

            Event::WindowEvent { event, .. } => {
                if let WindowEvent::CloseRequested = event {
                    *control_flow = ControlFlow::Exit;
                }
            }

            _ => {}
        }

        let _ = (&server_addr, &username);
    });
}