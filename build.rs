extern crate cc;

fn main() {
    cc::Build::new().file("src/epoll.c").compile("epoll");
}