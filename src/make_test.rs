#[macro_export]
macro_rules! test {
    {
        $func:path
    } => {
        #[test_case]
        #[allow(unused_imports)]
        fn _test() {
            $crate::__private::_run_test(core::any::type_name_of_val(&$func), $func);
        }
    };
}
