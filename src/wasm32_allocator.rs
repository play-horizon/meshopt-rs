//! The allocator behind `operator new` and `operator delete` on wasm32 (`wasm32_allocator.cpp`),
//! where no C++ standard library provides them: the Rust global allocator, each block led by a
//! header holding its size, since `operator delete` isn't given it. The two functions are
//! unmangled symbols, so a wasm32 module built with this crate exports them too.

use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};

/// The size of the header, and the alignment of every block: `alignof(max_align_t)` on wasm32,
/// the alignment `operator new` must give.
const HEADER: usize = 16;

/// Allocates `size` bytes for `operator new`; aborts when they can't be had, as `operator new`
/// may not return null and C++ exceptions are off.
#[unsafe(no_mangle)]
pub extern "C" fn meshopt_rust_allocate(size: usize) -> *mut u8 {
    let Some(layout) = size
        .checked_add(HEADER)
        .and_then(|total| Layout::from_size_align(total, HEADER).ok())
    else {
        handle_alloc_error(Layout::new::<[u8; HEADER]>());
    };
    // SAFETY: `layout` has a non-zero size (at least `HEADER`).
    let block = unsafe { alloc(layout) };
    if block.is_null() {
        handle_alloc_error(layout);
    }
    // SAFETY: the block is `HEADER`-aligned and at least `HEADER` bytes long, so the header fits
    // at its start, aligned for a `usize`, and the bytes after it are the caller's.
    unsafe {
        block.cast::<usize>().write(layout.size());
        block.add(HEADER)
    }
}

/// Frees a block of [`meshopt_rust_allocate`] for `operator delete`; null is ignored.
///
/// # Safety
///
/// `pointer` is null or a block [`meshopt_rust_allocate`] returned and not yet freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn meshopt_rust_deallocate(pointer: *mut u8) {
    if pointer.is_null() {
        return;
    }
    // SAFETY: the caller passes a block of `meshopt_rust_allocate`: its header, `HEADER` bytes
    // before it, holds the size it was allocated with, under that alignment.
    unsafe {
        let block = pointer.sub(HEADER);
        let total = block.cast::<usize>().read();
        dealloc(block, Layout::from_size_align_unchecked(total, HEADER));
    }
}
