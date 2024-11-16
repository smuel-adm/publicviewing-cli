use anyhow::Context;
use std::time::Duration;
use std::time::Instant;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    monitor::MonitorHandle,
    window::{Window, WindowBuilder},
};

use anyhow::anyhow;
use anyhow::Result;
use wry::WebViewBuilder;

use super::Cli;

pub(crate) fn run(args: Cli) -> Result<()> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop)?;
    window.set_title("PublicViewing - Cli");

    let num_urls = args.urls.len();
    let mut urls = args.urls.clone().into_iter().cycle();
    let start_url = urls
        .next()
        .ok_or(anyhow!("`start_url` could not read as String"))?;

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

    let builder = WebViewBuilder::new().with_url(&start_url);

    #[cfg(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    ))]
    let webview = builder.build(&window)?;

    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        target_os = "android"
    )))]
    let webview = {
        use tao::platform::unix::WindowExtUnix;
        use wry::WebViewBuilderExtUnix;
        let vbox = window.default_vbox().unwrap();
        builder.build_gtk(vbox)?
    };

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
            let current_url = webview.url().unwrap();

            if cycle_sec > 0 {
                match num_urls {
                    1 => webview
                        .load_url(current_url.as_ref())
                        .expect("Load `current_url`"),
                    _ => webview.load_url(&url).expect("load `url`"),
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
fn move_window_to_other_monitor(window: &Window, i: usize) -> Result<()> {
    let monitors: Vec<MonitorHandle> = window.available_monitors().collect();
    let monitor = monitors
        .get(i)
        .context(format!("No monitor found at index: {}", &i));
    let pos = monitor?.position();
    window.set_outer_position(tao::dpi::PhysicalPosition { x: pos.x, y: pos.y });

    Ok(())
}
