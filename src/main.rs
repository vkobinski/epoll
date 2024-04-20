use std::f32::consts::E;
use std::ffi::{c_char, c_uint, c_ulong, c_void};
use std::{clone, mem, thread};
use std::{os::raw::c_int, ptr::null};

//union epoll_data {
//    void     *ptr;
//    int       fd;
//    uint32_t  u32;
//    uint64_t  u64;
//};

#[repr(C)]
struct sigset_t {
    sig: *mut c_ulong,
}

#[repr(C)]
union epoll_data {
    void: *const c_void,
    fd: c_int,
    u32: std::os::raw::c_uint,
    u64: std::os::raw::c_uint,
}

#[repr(C)]
struct EpollEvent {
    events: std::os::raw::c_uint,
    data: epoll_data,
}

extern "C" {

    fn epoll_create(size: c_int) -> c_int;
    fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, epoll_event: *const EpollEvent) -> c_int;
    fn epoll_wait(epfd: c_int, events: *mut EpollEvent, maxevents: c_int, timeout: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: c_int) -> c_int;
    fn signalfd(fd: c_int,mask : *const sigset_t, flags: c_int) -> c_int;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    //fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
}

fn write_to_db(fd: c_int) {
    println!("Printing to DB");

    unsafe {
        let buf: *const c_void = "1".as_ptr() as *const c_void;
        let result = write(fd, buf, 10);
        println!("{:?}", result);
    }
}

fn print_when_ready(fd: c_int, memfd: c_int) {
    unsafe {
        let mut event = EpollEvent {
            events: 1,
            data: epoll_data { fd: memfd },
        };

        epoll_wait(fd, &mut event as *mut EpollEvent, 1, -1);
    }

    println!("Ready!");
}

fn main() {
    let epoll_fd: c_int;
    let event: EpollEvent;

    unsafe {
        epoll_fd = epoll_create(1);

        println!("memfd {:?}", memfd);

        event = EpollEvent {
            events: 1,
            data: epoll_data { fd: memfd },
        };

        epoll_ctl(epoll_fd, 1, eventfd, &event as *const EpollEvent);
    }

    thread::spawn(move || {
        print_when_ready(epoll_fd, eventfd);
    });

    thread::spawn(move || {
        write_to_db(memfd);
    });

    loop {}
}
