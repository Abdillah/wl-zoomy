use wayland_server::{EventLoopHandle, GlobalHandler, Client};
use wayland_server::protocol::wl_compositor;
use wayland_server::protocol::wl_compositor::WlCompositor;
use wayland_server::protocol::wl_surface::WlSurface;
use wayland_server::protocol::wl_region::WlRegion;

pub struct CompositorData {}

impl CompositorData {
    pub fn new() -> Self {
        CompositorData {}
    }
}

impl GlobalHandler<WlCompositor> for CompositorData {
    fn bind(&mut self, evqh: &mut EventLoopHandle, client: &Client, global: WlCompositor) {
        println!("global: Compositor registered");

        let compositor_hand_id = evqh.add_handler(CompositorData::new());
        evqh.register::<WlCompositor, CompositorData>(&global, compositor_hand_id);
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
        println!("compositor: create surface");

        let surface_id = evqh.add_handler(::surface::SurfaceData::new());
        evqh.register::<WlSurface, ::surface::SurfaceData>(&id, surface_id);
    }

    fn create_region(
        &mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlCompositor,
        id: WlRegion
    ) {
        println!("compositor: create region");
    }
}

declare_handler!(CompositorData, wl_compositor::Handler, wl_compositor::WlCompositor);
