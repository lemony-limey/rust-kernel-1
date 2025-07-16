use super::Locked;

use alloc::alloc::{GlobalAlloc, Layout};
use core::{
    mem, 
    ptr::{self, NonNull},
};

/// The block sizes to use.
///
/// The sizes must each be power of 2 because they are also used as
/// the block alignment, and alignments must always be powers of 2
/// also.
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];


struct ListNode
{
    next: Option<&'static mut ListNode>,
}


pub struct FixedSizeBlockAllocator
{
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,  // Temporary placeholder
}

impl FixedSizeBlockAllocator
{
    /// Creates an empty FixedSizeBlockAllocator.
    pub const fn new() -> Self
    {
        const EMPTY: Option<&'static mut ListNode> = None;

        Self
        {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    /// Initialise the allocator with the given heap bounds.
    ///
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize)
    {
        unsafe { self.fallback_allocator.init(heap_start as *mut u8, heap_size); }
    }

    /// Allocates using the fallback allocator.
    fn fallback_alloc(&mut self, layout: Layout) -> *mut u8
    {
        match self.fallback_allocator.allocate_first_fit(layout)
        {
            Ok(ptr) => ptr.as_ptr(),
            Err(_) => ptr::null_mut(), // Indicate failed allocation
        }
    }
}


/// Choose an appropriate block size for the given layout.
///
/// Returns an index into the 'BLOCK_SIZES' array.
fn list_index(layout: &Layout) -> Option<usize>
{
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES.iter().position(|&s| s >= required_block_size)
}


unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator>
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        // Get a mutable reference to the wrapped allocator
        let mut allocator = self.lock();

        match list_index(&layout)
        {
            Some(index) => {
                // Try to get the first node from the free list
                match allocator.list_heads[index].take()
                {
                    // Free list has available block
                    Some(node) => {
                        // Head pointer of free list now points to node after
                        // newly allocated one.
                        allocator.list_heads[index] = node.next.take();

                        node as *mut ListNode as *mut u8
                    },

                    // Free list was empty, so create new ones
                    None => {
                        // No block exists in list -> allocate new block
                        let block_size = BLOCK_SIZES[index];

                        // Only works if all block sizes are a power of 2
                        let block_align = block_size;
                        let layout = Layout::from_size_align(block_size, block_align)
                            .unwrap();

                        allocator.fallback_alloc(layout)
                    },
                }
            },

            // No block size fits for the requested allocation
            None => allocator.fallback_alloc(layout),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
    {
        // Get a mutable reference to the wrapped allocator
        let mut allocator = self.lock();

        match list_index(&layout)
        {
            Some(index) => {
                let new_node = ListNode {
                    next: allocator.list_heads[index].take(),
                };

                // Verify that block has size and alignment required for storing node
                assert!(mem::size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(mem::align_of::<ListNode>() <= BLOCK_SIZES[index]);

                let new_node_ptr = ptr as *mut ListNode;

                unsafe
                {
                    new_node_ptr.write(new_node);
                    allocator.list_heads[index] = Some(&mut *new_node_ptr);
                }
            },

            // No fitting block size exists in BLOCK_SIZES
            None => {
                let ptr = NonNull::new(ptr).unwrap();

                unsafe
                {
                    allocator.fallback_allocator.deallocate(ptr, layout);
                }
            }
        }
    }
}
