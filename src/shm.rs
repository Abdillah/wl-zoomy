extern crate mmap;

use std::os::unix::io::RawFd;

use wayland_server::{EventLoopHandle, GlobalHandler, Client};
use wayland_server::protocol::wl_shm;
use wayland_server::protocol::wl_shm::WlShm;
use wayland_server::protocol::wl_shm_pool::WlShmPool;


pub struct ShmData {}

impl ShmData {
    pub fn new() -> Self {
        ShmData {}
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

        println!("shm: create_pool: mmap-ing {}", size);
        let map = mmap::MemoryMap::new(size as usize, &moption).expect("shm: create_pool: Failed mmap-ing");
        println!("shm: create_pool: Success mmap-ing");

        // Create ShmPoolData
        let shmpooldata = ::shm_pool::ShmPoolData::new(fd, map, size);
        let shmpool_hand_id = evqh.add_handler(shmpooldata);
        evqh.register::<WlShmPool, ::shm_pool::ShmPoolData>(&id, shmpool_hand_id);
    }
}

unsafe impl ::std::marker::Send for ShmData {}

declare_handler!(ShmData, wl_shm::Handler, wl_shm::WlShm);
