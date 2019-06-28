/*******************************************************************************
 * Copyright (c) 2019 Jens Reimann
 *
 * This program and the accompanying materials are made available under the
 * terms of the Eclipse Public License 2.0 which is available at
 * http://www.eclipse.org/legal/epl-2.0
 *
 * SPDX-License-Identifier: EPL-2.0
 *******************************************************************************/

#![no_std]

use core::alloc::{GlobalAlloc, Layout};

pub struct EspIdfAllocator;

const MALLOC_CAP_8BIT: u32 = 4;

extern "C" {
    fn heap_caps_malloc(size: usize, caps: u32) -> *mut core::ffi::c_void;
    fn heap_caps_free(ptr: *mut core::ffi::c_void);
    fn heap_caps_realloc(
        ptr: *mut core::ffi::c_void,
        size: usize,
        caps: u32,
    ) -> *mut core::ffi::c_void;
}

unsafe impl GlobalAlloc for EspIdfAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        heap_caps_malloc(layout.size(), MALLOC_CAP_8BIT) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        heap_caps_free(ptr as *mut core::ffi::c_void)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        heap_caps_realloc(ptr as *mut core::ffi::c_void, new_size, MALLOC_CAP_8BIT) as *mut u8
    }
}
