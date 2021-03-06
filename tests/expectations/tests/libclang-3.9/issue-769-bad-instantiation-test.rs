#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Rooted<T> {
        pub member: T,
        pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    }
    impl<T> Default for Rooted<T> {
        fn default() -> Self {
            unsafe { ::std::mem::zeroed() }
        }
    }
    #[test]
    fn __bindgen_test_layout_Rooted_open0_int_close0_instantiation() {
        assert_eq!(
            ::std::mem::size_of::<root::Rooted<::std::os::raw::c_int>>(),
            4usize,
            concat!(
                "Size of template specialization: ",
                stringify!(root::Rooted<::std::os::raw::c_int>)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<root::Rooted<::std::os::raw::c_int>>(),
            4usize,
            concat!(
                "Alignment of template specialization: ",
                stringify!(root::Rooted<::std::os::raw::c_int>)
            )
        );
    }
    #[test]
    fn __bindgen_test_layout_Rooted_open0_int_close0_instantiation_1() {
        assert_eq!(
            ::std::mem::size_of::<root::Rooted<::std::os::raw::c_int>>(),
            4usize,
            concat!(
                "Size of template specialization: ",
                stringify!(root::Rooted<::std::os::raw::c_int>)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<root::Rooted<::std::os::raw::c_int>>(),
            4usize,
            concat!(
                "Alignment of template specialization: ",
                stringify!(root::Rooted<::std::os::raw::c_int>)
            )
        );
    }
}
