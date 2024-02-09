// SPDX-License-Identifier: GPLv3
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
    /// Optional url(s) to open,
    #[arg(default_values_t = vec!["https://google.com".to_string()])]
    urls: Vec<String>,

    // Monitor number on which the window should open, no effect if you have only one monitor!
    #[arg(
        long,
        help = "Monitor number on which the window should open, no effect if you have only one monitor! (only on Windows OS)"
    )]
    monitor: Option<usize>,

    // Open window in fullscreen
    #[arg(long, short, help = "Open window in fullscreen", group = "options")]
    fullscreen: bool,

    // Open window maximized
    #[arg(long, short, help = "Open window maximized", group = "options")]
    maximized: bool,

    // Cycle time between site reloads (if more then one URL was given), in seconds
    #[arg(
        long,
        short,
        default_value_t = 10,
        help = "Cycle time between site reloads (if more then one URL was given), in seconds"
    )]
    cycle_sec: u64,
}

// Multi monitor support
fn move_window_to_other_monitor(window: &Window, i: usize) -> wry::Result<()> {
    let monitors: Vec<MonitorHandle> = window.available_monitors().collect();
    let monitor = monitors.get(i).expect("Monitor not found");
    let pos = monitor.position();
    window.set_outer_position(tao::dpi::PhysicalPosition { x: pos.x, y: pos.y });

    Ok(())
}

fn main() -> wry::Result<()> {
    let cli = Cli::parse();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("PublicViewing - Cli");

    let num_urls = cli.urls.len();
    let mut urls = cli.urls.clone().into_iter().cycle();
    let start_url = urls.next().unwrap();

    if let Some(monitor) = cli.monitor {
        move_window_to_other_monitor(&window, monitor)?;
    }

    if cli.maximized {
        window.set_maximized(true);
    }

    if cli.fullscreen {
        use tao::window::Fullscreen;
        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
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

    let timer_length = Duration::new(cli.cycle_sec, 0);

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
