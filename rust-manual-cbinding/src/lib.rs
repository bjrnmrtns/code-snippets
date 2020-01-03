#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

extern "C" {
    // Redefinition in rust of borc/include/bor.h
    fn bor_main(x: i32) -> i32;
    fn bor_sum(input: *const i32, size: libc::size_t) -> i32;
    fn bor_sum_colors(colors: *const Color, size: libc::size_t) -> i32;
}

pub fn bor_main_rust() -> i32 {
    unsafe { bor_main(42) }
}

pub fn bor_sum_rust(x: &[i32]) -> i32 {
    unsafe { bor_sum(x.as_ptr(), x.len() as libc::size_t) }
}

pub fn bor_sum_colors_rust(colors: &[Color]) -> i32 {
    unsafe { bor_sum_colors(colors.as_ptr(), colors.len() as libc::size_t) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bor_main_rust_test() {
        assert_eq!(42, bor_main_rust());
    }
    #[test]
    fn bor_sum_rust_with_slice_test() {
        let ar: [i32; 2] = [1, 2];
        assert_eq!(3, bor_sum_rust(&ar));
    }
    #[test]
    fn bor_sum_rust_with_vec_test() {
        let v = vec![3, 4];
        assert_eq!(7, bor_sum_rust(&v));
    }
    #[test]
    fn bor_sum_colors_rust_test() {
        let colors = vec![Color { r: 1, g: 2, b: 3, }, Color { r: 4, g: 5, b: 6, }];
        assert_eq!(21, bor_sum_colors_rust(&colors));
    }
}
