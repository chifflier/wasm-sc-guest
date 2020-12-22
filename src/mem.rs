// use std::ffi;
// use std::mem::size_of;
// use std::alloc::{
//     Layout,
//     alloc as underlying_alloc,
//     dealloc as underlying_dealloc,
// };

#[no_mangle]
pub fn sc_allocate(sz: u32) -> *mut u8 {
    // unsafe{ print_str("sc_allocate") };
    let v: Vec<u8> = vec![0; sz as usize];
    let b = v.into_boxed_slice();
    // unsafe{ print_str(&format!("b.len {}", b.len())) };
    let ptr = Box::into_raw(b);
    ptr as *mut _
}

/// # Safety
/// `ptr` must have been allocated using `sc_allocate`
#[no_mangle]
pub unsafe fn sc_free(ptr: *mut u8) {
    // print_str("sc_free");
    // {
    //     let mut len = 0;
    //     while *ptr.offset(len) != 0 {
    //         len += 1;
    //     }
    //     let slice = std::slice::from_raw_parts(ptr, len as usize);
    //     print_str(&format!("{:x?}", slice));
    // }
    let _b = Box::from_raw(ptr);
}

// // This is basically a struct holding the arguments
// // we need to pass to `dealloc`. I think it is possible
// // to do this while storing less, but this is trivial to
// // implement and proves the point.
// #[derive(Copy, Clone)]
// struct AllocInfo {
//     layout: Layout,
//     ptr: *mut u8,
// }
// 
// unsafe fn wrapped_alloc(layout: Layout) -> *mut u8 {
//     // Compute a layout sufficient to store `AllocInfo`
//     // immediately before it.
//     let header_layout = Layout::new::<AllocInfo>();
// 
//     let (to_request, offset) = header_layout.extend(layout)
//         .expect("real code should probably return null");
// 
//     let orig_ptr = underlying_alloc(to_request);
//     if orig_ptr.is_null() {
//         return orig_ptr;
//     }
// 
//     let result_ptr = orig_ptr.add(offset);
//     // Write `AllocInfo` immediately prior to the pointer we return.
//     // This way, we always know where to get it for passing to
//     // `underlying_dealloc`.
//     let info_ptr = result_ptr.sub(size_of::<AllocInfo>()) as *mut AllocInfo;
//     info_ptr.write_unaligned(AllocInfo {
//         layout: to_request,
//         ptr: orig_ptr,
//     });
//     result_ptr
// }
// 
// unsafe fn wrapped_dealloc(ptr: *mut u8) {
//     assert!(!ptr.is_null());
//     // Simply read the AllocInfo we wrote in `alloc`, and pass it into dealloc.
//     let info_ptr = ptr.sub(size_of::<AllocInfo>()) as *const AllocInfo;
//     let info = info_ptr.read_unaligned();
//     underlying_dealloc(info.ptr, info.layout);
// }
// 
// #[no_mangle]
// pub extern "C" fn sc_allocate(sz: u32) -> *mut u8 {
//     unsafe{ wrapped_alloc(Layout::from_size_align_unchecked(sz as usize, 4)) }
// }
// 
// /// # Safety
// /// `ptr` must have been allocated using `sc_allocate`
// #[no_mangle]
// pub unsafe extern "C" fn sc_free(ptr: *mut u8) {
//     wrapped_dealloc(ptr)
// }
