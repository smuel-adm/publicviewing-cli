use instant::Instant;

use anyhow::Context;
use std::time::Duration;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    monitor::MonitorHandle,
    window::{Window, WindowBuilder},
};

use anyhow::Result;
use wry::WebViewBuilder;

use super::Cli;

pub(crate) fn run(args: Cli) -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("PublicViewing - Cli");

    let num_urls = args.urls.len();
    let mut urls = args.urls.clone().into_iter().cycle();
    let start_url = urls.next().unwrap();

    if let Some(monitor) = args.monitor {
        move_window_to_other_monitor(&window, monitor)?;
    }

    if args.maximized {
        window.set_maximized(true);
    }

    if args.fullscreen {
        use tao::window::Fullscreen;
        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
    }

    if args.above {
        window.set_always_on_top(true);
    }

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let builder = WebViewBuilder::new(&window);

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let builder = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        WebViewBuilder::new_gtk(vbox)
    };

    let webview = builder.with_url(&start_url)?.build()?;

    let (cycle_sec, timer_length) = match args.cycle_sec {
        Some(length) => (length, Duration::new(length, 0)),
        None => (0, Duration::new(0, 0)),
    };

    event_loop.run(move |event, _, control_flow| match event {
        Event::NewEvents(StartCause::Init) => {
            *control_flow = ControlFlow::WaitUntil(Instant::now() + timer_length)
        }
        Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
            *control_flow = ControlFlow::WaitUntil(Instant::now() + timer_length);
            let url = urls.next().unwrap();
            let current_url = webview.url();

            if cycle_sec > 0 {
                match num_urls {
                    1 => webview.load_url(current_url.as_ref()),
                    _ => webview.load_url(&url),
                }
            }
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
    });
}

// Multi monitor support
pub(crate) fn move_window_to_other_monitor(window: &Window, i: usize) -> Result<()> {
    let monitors: Vec<MonitorHandle> = window.available_monitors().collect();
    let monitor = monitors
        .get(i)
        .context(format!("No monitor found at index: {}", &i));
    let pos = monitor?.position();
    window.set_outer_position(tao::dpi::PhysicalPosition { x: pos.x, y: pos.y });

    Ok(())
}
