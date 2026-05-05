use peak_alloc::PeakAlloc;

#[global_allocator]
pub static PEAK_ALLOC: PeakAlloc = PeakAlloc;
