use bor_sys::bor_main;

pub fn bor_main_rust() -> i32 {
    unsafe { bor_main(42) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bor_main_rust_test() {
        assert_eq!(42, bor_main_rust());
    }
}
