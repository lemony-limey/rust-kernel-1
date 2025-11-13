use super::{align_up, Locked};

use alloc::alloc::{GlobalAlloc, Layout};
use core::{mem, ptr};
use x86_64::structures::paging::mapper::MappedFrame::Size4KiB;
use x86_64::structures::paging::{Page, PageTableFlags};
use crate::println;

const HIGHEST_ORDER: usize = 16;
const NUMBER_OF_ORDERS: usize = HIGHEST_ORDER + 1;

// Page size is 4KiB in this case.
// Order specifies the number of pages we need.

struct ListNode
{
    next: Option<&'static mut ListNode>,
}

pub struct BuddyAllocator
{
    free_list: [Option<&'static mut ListNode>; NUMBER_OF_ORDERS],

}

impl BuddyAllocator
{
    pub const fn new() -> Self
    {
        const EMPTY: Option<&'static mut ListNode> = None;

        Self
        {
            // Everything in the free list is empty initially
            free_list: [EMPTY; NUMBER_OF_ORDERS],
        }
    }

    /// Unsafe because it can only be called once.
    pub unsafe fn init()
    {

        todo!()
    }

    pub fn dump(&self)
    {
        println!("*** buddy page allocator - free list ***");

        for i in 0..=HIGHEST_ORDER
        {

        }
        todo!()
    }

    fn insert_pages(&self, range_start: &Page, page_count: usize)
    {
        todo!()

    }

    fn remove_pages(&self, range_start: &Page, page_count: usize)
    {
        todo!()
    }

    fn insert_free_block(&mut self, order: usize, block_start: &Page)
    {
        todo!()
    }

    fn remove_free_block(&mut self, order: usize, block_start: &Page)
    {
        todo!()
    }

    fn allocate_pages(&mut self, order: usize, page_allocation_flags: Option<PageTableFlags>) -> *mut u8
    {
        todo!()
    }

    fn free_pages(&mut self, order: usize, block_start: &Page)
    {

        todo!()
    }

    fn split_block(&self, order: usize, block_start: &Page)
    {
        todo!()

    }

    fn merge_buddies(&self, order: usize, buddy: &Page)
    {

        todo!()
    }


    fn buddy_exists(&self, order: usize, buddy: &Page) -> bool
    {
        todo!()
    }

    fn block_exists_in_free_list(&self, order: usize, target_block: &Page) -> bool
    {
        todo!()
    }

    fn is_page_within_block(
        &self,
        order: usize,
        block_page_frame_number: usize,
        target_page_frame_number: usize,
    ) -> bool
    {

        todo!()
    }
}

/// Locked allows us to enforce mututal exclusion.
unsafe impl GlobalAlloc for Locked<BuddyAllocator>
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        // Layout contains requested size and alignment.
        // Calculate the next power of 2 value greater than or equal to the requested size.
        let order = find_order_for_given_size(layout.size());

        // Get a mutable reference to self with mutual exclusion
        let mut allocator = self.lock();

        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
    {
        // Get a mutable reference to self with mutual exclusion
        let mut allocator = self.lock();

        todo!()
    }
}

/// Adapted from:
/// stacsos/kernel/inc/stacsos/kernel/mem/page-allocator-buddy.h
const fn pages_per_block(order: usize) -> usize
{
    1 << order
}

/// Adapted from:
/// stacsos/kernel/inc/stacsos/kernel/mem/page-allocator-buddy.h
const fn is_block_aligned(order: usize, page_frame_number: usize) -> bool
{
    !(page_frame_number & (pages_per_block(order) - 1)) != 0
}

/// Code adapted from:
/// https://stackoverflow.com/questions/1322510/given-an-integer-how-do-i-find-the-next-largest-power-of-two-using-bit-twiddlin#1322548
const fn find_order_for_given_size(mut size: usize) -> usize
{
    if size == 0
    {
        return 1usize;
    }

    size -= 1;

    size |= size >> 1;
    size |= size >> 2;
    size |= size >> 4;
    size |= size >> 8;
    size |= size >> 16;
    size |= size >> 32;

    size += 1;

    size
}
