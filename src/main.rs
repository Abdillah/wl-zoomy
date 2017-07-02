#[macro_use]
extern crate wayland_server;

use wayland_server::Client;
use wayland_server::protocol::wl_compositor::WlCompositor;
use wayland_server::protocol::wl_shm::WlShm;
use wayland_server::protocol::wl_surface::WlSurface;
use wayland_server::protocol::wl_region::WlRegion;

mod compositor;
mod shm;
mod surface;
mod shm_pool;

fn main() {
    let (mut display, mut event_loop) = wayland_server::create_display();
    display.add_socket_auto().expect("Display socket cannot be used");

    let compositor_id = event_loop.add_handler(compositor::CompositorData::new());
    event_loop.register_global::<WlCompositor, compositor::CompositorData>(compositor_id, /* version: */ 3);

    let shm_id = event_loop.add_handler(shm::ShmData::new());
    event_loop.register_global::<WlShm, shm::ShmData>(shm_id, /* version: */ 1);

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