

// slices the array of bytes into 512 byte long chunks
pub fn slice_into_512(buffer: &[u8]) -> Vec<Vec<u8>>  {
    return buffer.chunks(512).map(|s| s.into()).collect();
}