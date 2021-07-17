use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop},
    window::WindowBuilder,
};
use async_std::{task};

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let events = event_loop.as_async();

    let events_task = task::spawn(async {

        loop {
            let event = events.await;
            println!("{:?}", event);

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => break,
                Event::MainEventsCleared => {
                    window.request_redraw();
                }
                _ => (),
            }
        }
    });

    task::block_on(events_task);
}
