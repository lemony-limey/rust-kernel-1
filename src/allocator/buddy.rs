use super::{align_up, Locked};

use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};

pub struct BuddyAllocator
{

}

impl BuddyAllocator
{
    pub const fn new() -> Self
    {

    }

    pub unsafe fn init()
    {

    }
}

unsafe impl GlobalAlloc for Locked<BuddyAllocator>
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        // Get a mutable reference to self with mutual exclusion
        let mut allocator = self.lock();

    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
    {
        // Get a mutable reference to self with mutual exclusion
        let mut allocator = self.lock();

    }
}
