//! # 全局属性
//! - `#![no_std]`
//!   禁用标准库
#![no_std]
//!
//! - `#![no_main]`
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口
#![no_main]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(llvm_asm)]`
//!   内嵌汇编
#![feature(llvm_asm)]
#![feature(core_intrinsics)]
#[macro_use]
extern crate redos;

use core::mem::size_of;

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle]
pub extern "C" fn rust_main() {
    println!("Hello frame allocate!");
    let mut allocator = redos::memory::frame::FrameAllocator::default();
    let mut pn = allocator.alloc().unwrap();
    for _ in 1..allocator.frame_total() {
        let new_pn = allocator.alloc().unwrap();
        pn = new_pn;
    }
    assert!(allocator.alloc().is_err());
    println!("{}", size_of::<usize>());

    panic!("end");
}
