extern crate mmap;

use wayland_server::{EventLoopHandle, GlobalHandler, Client};
use wayland_server::protocol::wl_shm;
use wayland_server::protocol::wl_shm::WlShm;
use wayland_server::protocol::wl_shm_pool::WlShmPool;

use std::os::unix::io::RawFd;

pub struct ShmData {
    data: Option<mmap::MemoryMap>
}

impl ShmData {
    pub fn new() -> Self {
        ShmData {
            data: None
        }
    }
}

impl GlobalHandler<WlShm> for ShmData {
    fn bind(&mut self, evqh: &mut EventLoopHandle, client: &Client, global: WlShm) {
        println!("global: Shm registered");

        let shm_hand_id = evqh.add_handler(ShmData::new());
        evqh.register::<WlShm, ShmData>(&global, shm_hand_id);
    }
}

impl wl_shm::Handler for ShmData {
    fn create_pool(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlShm,
        id: WlShmPool,
        fd: RawFd,
        size: i32
    ) {
        // Create new MemoryMap
        let moption = [
            mmap::MapOption::MapReadable,
            mmap::MapOption::MapWritable,
            mmap::MapOption::MapFd(fd),
            mmap::MapOption::MapNonStandardFlags(0x01) // MAP_SHARED
        ];

        self.data = mmap::MemoryMap::new(size as usize, &moption).ok();
    }
}

unsafe impl ::std::marker::Send for ShmData {}

declare_handler!(ShmData, wl_shm::Handler, wl_shm::WlShm);
