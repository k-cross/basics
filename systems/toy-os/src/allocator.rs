use linked::LinkedListAllocator;
//use bump::BumpAllocator;
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB, mapper::MapToError,
    },
};

pub mod bump;
pub mod linked;

// address to start allocating virtual memory to heap, physical memory is still
// mapped using the frame allocators
pub const HEAP_START: usize = 0x_4444_4444_0000;
// size is 100 KiB
pub const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
//static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let heap_start = VirtAddr::new(HEAP_START as u64);
    let page_range = {
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

// allign on the page size, get address of next page of memory if it doesn't
// already fall on the boundary
fn align_up(addr: usize, align: usize) -> usize {
    // use bitwise operations for speed (flip all bits of not align - 1 to set
    // the next address)
    (addr + align - 1) & !(align - 1)
}
