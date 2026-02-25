pub(super) fn copy_from_same_vec(v: &mut Vec<u8>, start: usize, len: usize) {
    v.reserve(len);

    unsafe {
        let ptr = v.as_mut_ptr();
        let original_len = v.len();

        // memmove-style copy (allows overlap)
        std::ptr::copy(
            ptr.add(start),        // source
            ptr.add(original_len), // destination (end of vec)
            len,                   // number of bytes
        );

        // Update the vector length
        v.set_len(original_len + len);
    }
}
