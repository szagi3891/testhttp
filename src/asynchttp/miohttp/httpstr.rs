use std::cmp;
use std::str;

pub const METHOD_NAME_LENGTH  : usize =   16;
pub const PATH_LENGTH         : usize = 4096;
pub const HEADER_NAME_LENGTH  : usize =   32;
pub const HEADER_VALUE_LENGTH : usize =  256;

pub type MethodName = [u8; METHOD_NAME_LENGTH];
pub type Path = [u8; PATH_LENGTH];
pub type HeaderName = [u8; HEADER_NAME_LENGTH];
pub type HeaderValue = [u8; HEADER_VALUE_LENGTH];


/// Copy slices of bytes
pub fn copy(src: &[u8], dst: &mut [u8]) -> usize {
    let len = cmp::min(src.len(), dst.len());
    dst[..len].clone_from_slice(&src[..len]);
    len
}

/// Check two slices for euqality 
pub fn eq(one: &[u8], two: &[u8]) -> bool {
    let len = cmp::min(one.len(), two.len());
    for i in 0..len {
        if one[i] != two[i] {
            return false; 
        }
        if one[i] == 0 && two[i] == 0 {
           return true;
        }
    }
    return true;
}

/// Convert to regular string for logging, debuging, etc.
pub fn to_str(src: &[u8]) -> &str {
    let mut len = 0usize;
    for i in 0..src.len() {
        len += 1;
        if src[i] == 0 {
            break
        };
    };
    let wholestr = str::from_utf8(src).unwrap_or("<Invalid bytes>");
    
    unsafe {
        wholestr.slice_unchecked(0, len-1)
    }
}
