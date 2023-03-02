
/// An array of bytes that is guaranteed to be aligned at a 16 byte boundary in memory.
/// This allows the buffer to be viewed as a buffer of other primitive integer types
/// without risk of mis-aligned memory.
///
/// # Examples
///
/// ```
/// # use raw_reader::aligned_buffer::AlignedBuffer;
/// // Allocate a 32 byte buffer on the stack.
/// let mut stack_buffer: AlignedBuffer<32> = AlignedBuffer::new();
///
/// // Set the 17th byte in the buffer to a non-zero value.
/// stack_buffer[16] = 5;
///
/// // View the buffer as u128s instead of u8s.
/// let u128_view = stack_buffer.view_as::<u128>();
/// assert_eq!(u128_view.len(), 2);
/// assert_eq!(u128_view[0], 0); // Corresponds to bytes 0~15
/// assert_eq!(u128_view[1], 5); // Corresponds to bytes 16~31
/// ```
///
/// Note, that allocating large buffers on the stack may require resizing the stack:
/// https://www.reddit.com/r/rust/comments/872fc4/how_to_increase_the_stack_size/dw9usn1
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[repr(C, align(16))]
pub struct AlignedBuffer<const SIZE: usize>([u8; SIZE]);

impl<const SIZE: usize> AlignedBuffer<SIZE> {
    /// Constructs a new buffer with the specified length (which must be a multiple of 16).
    ///
    /// # Panics
    ///
    /// If the provided length isn't a multiple of 16.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raw_reader_lib::aligned_buffer::AlignedBuffer;
    /// // Allocates a buffer that is 64 bytes long.
    /// let buffer: AlignedBuffer<64> = AlignedBuffer::new();
    /// ```
    pub fn new() -> Self {
        debug_assert!(SIZE % 16 == 0, "buffer length must be a multiple of 16");
        AlignedBuffer([0; SIZE])
    }

    /// Returns a view into the buffer as a slice of the specified type. This slice spans the
    /// entire buffer. Only types with an alignment that divides 16 can be specified.
    /// This is technically platform dependent, but includes all the primitives on most platforms. 
    ///
    /// The byte ordering in the buffer is little-endian, so on little-endian systems, the values
    /// will be the actual in-memory values. On big-endian systems, the values will be reversed.
    ///
    /// # Panics
    ///
    /// If the specified viewing type has an alignment that doesn't divide 16.
    /// Since the buffer is aligned to 16 bytes, this will cause a buffer mis-alignment.
    ///
    /// # Examples
    ///
    /// ```
    /// # use raw_reader_lib::aligned_buffer::AlignedBuffer;
    /// // Allocate a buffer that is 64 bytes long and create various views into it.
    /// let buffer: AlignedBuffer<64> = AlignedBuffer::new();
    ///
    /// let i16_view = buffer.view_as::<i16>();     // View the buffer as &[i16]
    /// assert_eq!(i16_view.len(), 64 / 2);
    ///
    /// let usize_view = buffer.view_as::<usize>(); // View the buffer as &[usize]
    /// assert_eq!(usize_view.len(), 64 / std::mem::size_of::<usize>());
    /// 
    /// let u64_view: &[u64] = buffer.view_as();    // Alternate syntax. Views the buffer as &[u64]
    /// assert_eq!(u64_view.len(), 64 / 8);
    /// ```
    pub fn view_as<T>(&self) -> &[T] {
        debug_assert!(16 % std::mem::align_of::<T>() == 0, "type must have an alignment that divides 16");

        // This is safe because the buffer is aligned at 64 bits, so it's also aligned to all the
        // unsigned integer types (u8, u16, u32, u64, and usize).
        unsafe {
            std::slice::from_raw_parts(
                std::mem::transmute::<*const u8, *const T>(self.0.as_ptr()),
                self.0.len() / std::mem::size_of::<T>()
            )
        }
    }
}

// Allows the compiler to implicitly convert this to an `&[u8]`.
impl<const SIZE: usize> std::ops::Deref for AlignedBuffer<SIZE> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Allows the compiler to implicitly convert this to an `&mut [u8]`.
impl<const SIZE: usize> std::ops::DerefMut for AlignedBuffer<SIZE> {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_buffer_view_lengths {
        ($buffer_length:expr, $alignment_type:ty, $expected_length:expr) => {{
            // Allocate a new buffer with the specified length.
            let buffer: AlignedBuffer<$buffer_length> = AlignedBuffer::new();

            // View the buffer as the specified type.
            let view = buffer.view_as::<$alignment_type>();

            // Assert that the expected number of elements is contained in the view.
            assert_eq!(view.len(), $expected_length);
        }}
    }

    #[test]
    fn ensure_u16_views_have_correct_lengths() {
        test_buffer_view_lengths!(16, u16, 16 / 2);
        test_buffer_view_lengths!(32, u16, 32 / 2);
        test_buffer_view_lengths!(64, u16, 64 / 2);

        test_buffer_view_lengths!(80, u16, 80 / 2);

        test_buffer_view_lengths!(240, u16, 240 / 2);
        test_buffer_view_lengths!(256, u16, 256 / 2);
        test_buffer_view_lengths!(272, u16, 272 / 2);

        test_buffer_view_lengths!(0x0fff0, u16, 0x0fff0 / 2);
        test_buffer_view_lengths!(0x10000, u16, 0x10000 / 2);
        test_buffer_view_lengths!(0x10010, u16, 0x10010 / 2);
    }

    #[test]
    fn ensure_u32_views_have_correct_lengths() {
        test_buffer_view_lengths!(16, u32, 16 / 4);
        test_buffer_view_lengths!(32, u32, 32 / 4);
        test_buffer_view_lengths!(64, u32, 64 / 4);

        test_buffer_view_lengths!(80, u32, 80 / 4);

        test_buffer_view_lengths!(240, u32, 240 / 4);
        test_buffer_view_lengths!(256, u32, 256 / 4);
        test_buffer_view_lengths!(272, u32, 272 / 4);

        test_buffer_view_lengths!(0x0fff0, u32, 0x0fff0 / 4);
        test_buffer_view_lengths!(0x10000, u32, 0x10000 / 4);
        test_buffer_view_lengths!(0x10010, u32, 0x10010 / 4);
    }

    #[test]
    fn ensure_u64_views_have_correct_lengths() {
        test_buffer_view_lengths!(16, u64, 16 / 8);
        test_buffer_view_lengths!(32, u64, 32 / 8);
        test_buffer_view_lengths!(64, u64, 64 / 8);

        test_buffer_view_lengths!(80, u64, 80 / 8);

        test_buffer_view_lengths!(240, u64, 240 / 8);
        test_buffer_view_lengths!(256, u64, 256 / 8);
        test_buffer_view_lengths!(272, u64, 272 / 8);

        test_buffer_view_lengths!(0x0fff0, u64, 0x0fff0 / 8);
        test_buffer_view_lengths!(0x10000, u64, 0x10000 / 8);
        test_buffer_view_lengths!(0x10010, u64, 0x10010 / 8);
    }

    #[test]
    fn ensure_u128_views_have_correct_lengths() {
        test_buffer_view_lengths!(16, u128, 16 / 16);
        test_buffer_view_lengths!(32, u128, 32 / 16);
        test_buffer_view_lengths!(64, u128, 64 / 16);

        test_buffer_view_lengths!(80, u128, 80 / 16);

        test_buffer_view_lengths!(240, u128, 240 / 16);
        test_buffer_view_lengths!(256, u128, 256 / 16);
        test_buffer_view_lengths!(272, u128, 272 / 16);

        test_buffer_view_lengths!(0x0fff0, u128, 0x0fff0 / 16);
        test_buffer_view_lengths!(0x10000, u128, 0x10000 / 16);
        test_buffer_view_lengths!(0x10010, u128, 0x10010 / 16);
    }

    #[test]
    fn ensure_usize_views_have_correct_lengths() {
        const USIZE_LENGTH: usize = std::mem::size_of::<usize>();

        test_buffer_view_lengths!(16, usize, 16 / USIZE_LENGTH);
        test_buffer_view_lengths!(32, usize, 32 / USIZE_LENGTH);
        test_buffer_view_lengths!(64, usize, 64 / USIZE_LENGTH);

        test_buffer_view_lengths!(80, usize, 80 / USIZE_LENGTH);

        test_buffer_view_lengths!(240, usize, 240 / USIZE_LENGTH);
        test_buffer_view_lengths!(256, usize, 256 / USIZE_LENGTH);
        test_buffer_view_lengths!(272, usize, 272 / USIZE_LENGTH);

        test_buffer_view_lengths!(0x0fff0, usize, 0x0fff0 / USIZE_LENGTH);
        test_buffer_view_lengths!(0x10000, usize, 0x10000 / USIZE_LENGTH);
        test_buffer_view_lengths!(0x10010, usize, 0x10010 / USIZE_LENGTH);
    }

    macro_rules! test_buffer_alignment {
        ($buffer_length:expr, $alignment_type:ty, $expected_length:expr) => {{
            // Allocate a new buffer with the specified length.
            let buffer: AlignedBuffer<$buffer_length> = AlignedBuffer::new();

            // Align the buffer to the specified type.
            let (prefix, aligned, suffix) = unsafe {
                (&buffer.0).align_to::<$alignment_type>()
            };

            // Assert that the buffer was already aligned; meaning the prefix and suffix were empty.
            assert_eq!(prefix.len(), 0);
            assert_eq!(suffix.len(), 0);
            // Assert that the expected number of elements is contained in the resultant slice.
            assert_eq!(aligned.len(), $expected_length);
        }}
    }

    #[test]
    fn ensure_u16_slices_are_aligned() {
        test_buffer_alignment!(16, u16, 16 / 2);
        test_buffer_alignment!(32, u16, 32 / 2);
        test_buffer_alignment!(64, u16, 64 / 2);

        test_buffer_alignment!(80, u16, 80 / 2);

        test_buffer_alignment!(240, u16, 240 / 2);
        test_buffer_alignment!(256, u16, 256 / 2);
        test_buffer_alignment!(272, u16, 272 / 2);

        test_buffer_alignment!(0x0fff0, u16, 0x0fff0 / 2);
        test_buffer_alignment!(0x10000, u16, 0x10000 / 2);
        test_buffer_alignment!(0x10010, u16, 0x10010 / 2);
    }

    #[test]
    fn ensure_u32_slices_are_aligned() {
        test_buffer_alignment!(16, u32, 16 / 4);
        test_buffer_alignment!(32, u32, 32 / 4);
        test_buffer_alignment!(64, u32, 64 / 4);

        test_buffer_alignment!(80, u32, 80 / 4);

        test_buffer_alignment!(240, u32, 240 / 4);
        test_buffer_alignment!(256, u32, 256 / 4);
        test_buffer_alignment!(272, u32, 272 / 4);

        test_buffer_alignment!(0x0fff0, u32, 0x0fff0 / 4);
        test_buffer_alignment!(0x10000, u32, 0x10000 / 4);
        test_buffer_alignment!(0x10010, u32, 0x10010 / 4);
    }

    #[test]
    fn ensure_u64_slices_are_aligned() {
        test_buffer_alignment!(16, u64, 16 / 8);
        test_buffer_alignment!(32, u64, 32 / 8);
        test_buffer_alignment!(64, u64, 64 / 8);

        test_buffer_alignment!(80, u64, 80 / 8);

        test_buffer_alignment!(240, u64, 240 / 8);
        test_buffer_alignment!(256, u64, 256 / 8);
        test_buffer_alignment!(272, u64, 272 / 8);

        test_buffer_alignment!(0x0fff0, u64, 0x0fff0 / 8);
        test_buffer_alignment!(0x10000, u64, 0x10000 / 8);
        test_buffer_alignment!(0x10010, u64, 0x10010 / 8);
    }

    #[test]
    fn ensure_u128_slices_are_aligned() {
        test_buffer_alignment!(16, u128, 16 / 16);
        test_buffer_alignment!(32, u128, 32 / 16);
        test_buffer_alignment!(64, u128, 64 / 16);

        test_buffer_alignment!(80, u128, 80 / 16);

        test_buffer_alignment!(240, u128, 240 / 16);
        test_buffer_alignment!(256, u128, 256 / 16);
        test_buffer_alignment!(272, u128, 272 / 16);

        test_buffer_alignment!(0x0fff0, u128, 0x0fff0 / 16);
        test_buffer_alignment!(0x10000, u128, 0x10000 / 16);
        test_buffer_alignment!(0x10010, u128, 0x10010 / 16);
    }

    #[test]
    fn ensure_usize_slices_are_aligned() {
        const USIZE_LENGTH: usize = std::mem::size_of::<usize>();

        test_buffer_alignment!(16, usize, 16 / USIZE_LENGTH);
        test_buffer_alignment!(32, usize, 32 / USIZE_LENGTH);
        test_buffer_alignment!(64, usize, 64 / USIZE_LENGTH);

        test_buffer_alignment!(80, usize, 80 / USIZE_LENGTH);

        test_buffer_alignment!(240, usize, 240 / USIZE_LENGTH);
        test_buffer_alignment!(256, usize, 256 / USIZE_LENGTH);
        test_buffer_alignment!(272, usize, 272 / USIZE_LENGTH);

        test_buffer_alignment!(0x0fff0, usize, 0x0fff0 / USIZE_LENGTH);
        test_buffer_alignment!(0x10000, usize, 0x10000 / USIZE_LENGTH);
        test_buffer_alignment!(0x10010, usize, 0x10010 / USIZE_LENGTH);
    }

    #[test]
    fn ensure_primitive_types_are_correctly_aligned() {
        let buffer: AlignedBuffer<64> = AlignedBuffer::new();

        // These methods will panic if the types are not correctly aligned to 16 bytes.
        buffer.view_as::<bool>();
        buffer.view_as::<u8>();
        buffer.view_as::<i8>();
        buffer.view_as::<u16>();
        buffer.view_as::<i16>();
        buffer.view_as::<u32>();
        buffer.view_as::<i32>();
        buffer.view_as::<u64>();
        buffer.view_as::<i64>();
        buffer.view_as::<u128>();
        buffer.view_as::<i128>();
        buffer.view_as::<usize>();
        buffer.view_as::<isize>();
        buffer.view_as::<f32>();
        buffer.view_as::<f64>();
        buffer.view_as::<char>();
    }
}
