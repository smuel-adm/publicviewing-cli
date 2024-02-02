// SPDX-License-Identifier: GPLv2

use tao::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
  window::Fullscreen,
};
use wry::WebViewBuilder;

fn main() -> wry::Result<()> {
  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  // window.set_fullscreen(Some(Fullscreen::Borderless(None)));
  window.set_title("PublicViewing");
  window.set_maximized(true);
 
  let builder = WebViewBuilder::new(&window);

  let _webview = builder
    .with_url("http://v-sx-app01:1337/line1-machine-status.php")?
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
