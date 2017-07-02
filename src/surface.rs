extern crate mmap;

use wayland_server::{EventLoopHandle, GlobalHandler, Client};
use wayland_server::protocol::wl_surface;
use wayland_server::protocol::wl_surface::WlSurface;
use wayland_server::protocol::wl_buffer::WlBuffer;
use wayland_server::protocol::wl_region::WlRegion;
use wayland_server::protocol::wl_callback::WlCallback;
use wayland_server::protocol::wl_output::Transform;

struct Rec {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

pub struct SurfaceData {
    buffer: Option<WlBuffer>,
    x: i32,
    y: i32,
    damages: Vec<Rec>
}

impl SurfaceData {
    pub fn new() -> Self {
        SurfaceData {
            buffer: None,
            x: 0,
            y: 0
        }
    }
}

impl wl_surface::Handler for SurfaceData {
    fn destroy(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface
    ) {

    }

    fn attach(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface,
        buffer: Option<&WlBuffer>,
        x: i32,
        y: i32
    ) {
        self.buffer = buffer;
        self.x = x;
        self.y = y;
    }

    fn damage(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    ) {
        // Insert damage to be processed
    }

    fn frame(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface,
        callback: WlCallback
    ) {
    }

    fn set_opaque_region(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface,
        region: Option<&WlRegion>
    ) {
    }

    fn set_input_region(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface,
        region: Option<&WlRegion>
    ) {
    }

    fn commit(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface
    ) {
    }

    fn set_buffer_transform(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface,
        transform: Transform
    ) {
    }

    fn set_buffer_scale(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface,
        scale: i32
    ) {
    }

    fn damage_buffer(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlSurface,
        x: i32,
        y: i32,
        width: i32,
        height: i32
    ) {
    }
}

declare_handler!(SurfaceData, wl_surface::Handler, wl_surface::WlSurface);
