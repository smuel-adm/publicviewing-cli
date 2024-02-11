// SPDX-License-Identifier: GPLv3
use anyhow::{Context, Result};
use clap::Parser;
use instant::Instant;
use std::time::Duration;
use tao::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    monitor::MonitorHandle,
    window::{Window, WindowBuilder},
};
use wry::WebViewBuilder;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional url(s) to open, space separated
    #[arg(default_values_t = vec!["https://google.com".to_string()])]
    urls: Vec<String>,

    /// window will always be above other windows
    #[arg(long, short)]
    above: bool,

    /// cycle time between site reloads
    ///     if more then one URL was given
    ///     these URL's are cycled after that time
    #[arg(long, short, verbatim_doc_comment, default_value_t = 10)]
    cycle_sec: u64,

    /// open window in fullscreen
    #[arg(long, short, group = "options")]
    fullscreen: bool,

    /// open window maximized
    #[arg(long, short, group = "options")]
    maximized: bool,

    /// monitor number on which the window should open
    ///     This has no effect if you have only one monitor!
    ///     Android / Linux(Wayland): Unsupported
    #[arg(long, verbatim_doc_comment)]
    monitor: Option<usize>,
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

fn run(args: Cli) -> Result<()> {
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

    let timer_length = Duration::new(args.cycle_sec, 0);

    event_loop.run(move |event, _, control_flow| {
        // If we have only one url no control flows (timers and such) are required.
        // This disables the match event block below complete.
        if num_urls == 1 {
            *control_flow = ControlFlow::Wait;
        }

        match event {
            Event::NewEvents(StartCause::Init) => {
                *control_flow = ControlFlow::WaitUntil(Instant::now() + timer_length)
            }
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                *control_flow = ControlFlow::WaitUntil(Instant::now() + timer_length);
                if num_urls > 1 {
                    let url = urls.next().unwrap();
                    webview.load_url(&url);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Err(err) = run(args) {
        println!("{:?}", err);
        std::process::exit(1);
    }

    Ok(())
}
