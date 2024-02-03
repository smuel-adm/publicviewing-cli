// SPDX-License-Identifier: GPLv2

use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};
use wry::WebViewBuilder;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional url to open, default https://google.com
    #[arg(default_value = "https://google.com")]
    url: String,
    
    // Open window in fullscreen
    #[arg(long, short, group = "options")]
    fullscreen: bool,
    
    // Open window maximized
    #[arg(long, short, group = "options")]
    maximized: bool,
}

fn main() -> wry::Result<()> {
  let cli = Cli::parse();

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  window.set_title("[Astemo] PublicViewing - Cli");
  
  if cli.maximized {
    window.set_maximized(true);
  }

  if cli.fullscreen {
    use tao::window::Fullscreen;
    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
  }
   
  let builder = WebViewBuilder::new(&window);



  let _webview = builder
    .with_url(&cli.url)?
    .build()?;
  
  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    if let Event::WindowEvent {
      event: WindowEvent::CloseRequested,
      ..
    } = event
    {
      *control_flow = ControlFlow::Exit
    }
  });
}
