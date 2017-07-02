extern crate mmap;

use std::string::String;
use std::os::unix::io::RawFd;

use wayland_server::{EventLoopHandle, Resource, Client};
use wayland_server::protocol::wl_shm_pool;
use wayland_server::protocol::wl_shm_pool::WlShmPool;
use wayland_server::protocol::wl_buffer::WlBuffer;
use wayland_server::protocol::wl_shm::{Format, Error as ShmError};

pub struct ShmPoolData {
    fd: RawFd,
    data: Option<mmap::MemoryMap>,
    size: i32
}

impl ShmPoolData {
    pub fn new(fd: RawFd, data: mmap::MemoryMap, size: i32) -> Self {
        ShmPoolData {
            fd: fd,
            data: Some(data),
            size: size
        }
    }
}

impl wl_shm_pool::Handler for ShmPoolData {
    fn create_buffer(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlShmPool,
        id: WlBuffer,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: Format
    ) {
        let map = match self.data {
            Some(ref map) => map,
            None => {
                // Create new MemoryMap
                let moption = [
                    mmap::MapOption::MapReadable,
                    mmap::MapOption::MapWritable,
                    mmap::MapOption::MapFd(self.fd),
                    mmap::MapOption::MapNonStandardFlags(0x01) // MAP_SHARED
                ];

                println!("shm_pool: create_buffer: mmap-ing");
                let map = mmap::MemoryMap::new(self.size as usize, &moption).expect("shm_pool: create_buffer: Failed mmap-ing");
                println!("shm_pool: create_buffer: Success mmap-ing");

                self.data = Some(map);
                self.data.as_ref().unwrap()
            }
        };

        // Verify size
        let requested_size: usize = (height * stride) as usize;
        if map.len() < requested_size {
            // Stride invalid
            let msg = String::from(format!("SHM size ({}) less than requested buffer size ({}).", map.len(), requested_size));
            resource.post_error(ShmError::InvalidStride.to_raw(), msg);
        }
    }

    fn destroy(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlShmPool
    ) {
    }

    fn resize(&mut self,
        evqh: &mut EventLoopHandle,
        client: &Client,
        resource: &WlShmPool,
        size: i32
    ) {
        // Create new MemoryMap
        let moption = [
            mmap::MapOption::MapReadable,
            mmap::MapOption::MapWritable,
            mmap::MapOption::MapFd(self.fd),
            mmap::MapOption::MapNonStandardFlags(0x01) // MAP_SHARED
        ];

        println!("shm_pool: resize: mmap-ing");
        let map = mmap::MemoryMap::new(size as usize, &moption).expect("shm_pool: resize: Failed mmap-ing");
        println!("shm_pool: resize: Success mmap-ing");

        self.data = Some(map);
    }
}

unsafe impl ::std::marker::Send for ShmPoolData {}

declare_handler!(ShmPoolData, wl_shm_pool::Handler, wl_shm_pool::WlShmPool);
