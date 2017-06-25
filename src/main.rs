extern crate wayland_server;

use wayland_server::{EventLoopHandle, GlobalHandler, Client};
use wayland_server::protocol::wl_compositor::{WlCompositor, Handler};
use wayland_server::protocol::wl_surface::WlSurface;
use wayland_server::protocol::wl_region::WlRegion;

struct CompositorData {
    // Surface list
}

impl CompositorData {
    pub fn new() -> Self {
        CompositorData {}
    }
}

impl GlobalHandler<WlCompositor> for CompositorData {
    fn bind(&mut self, evqh: &mut EventLoopHandle, client: &Client, global: WlCompositor) {
        unimplemented!()
    }
}

impl wl_compositor::Handler for CompositorData {
    fn create_surface(
        &mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlCompositor,
        id: WlSurface
    ) {
        unimplemented!()
    }

    fn create_region(
        &mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlCompositor,
        id: WlRegion
    ) {
        unimplemented!()
    }
}

fn main() {
    let (mut display, mut event_loop) = wayland_server::create_display();
    display.add_socket_auto().expect("Display socket cannot be used");

    let compositor_hand_id = event_loop.add_handler(CompositorData::new());
    event_loop.register_global::<WlCompositor, CompositorData>(compositor_hand_id, /* version: */ 3);

    loop {
        // flush events to client sockets
        display.flush_clients();
        // receive request from clients and dispatch them
        // blocking if no request is pending for at most
        // 10ms
        event_loop.dispatch(Some(10)).unwrap();
        // then you can check events from other sources if
        // you need to
    }
}