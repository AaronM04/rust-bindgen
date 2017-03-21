/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct A {
    pub foo: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(::std::mem::size_of::<A>() , 4usize , concat ! (
               "Size of: " , stringify ! ( A ) ));
    assert_eq! (::std::mem::align_of::<A>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( A ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const A ) ) . foo as * const _ as usize } ,
                0usize , concat ! (
                "Alignment of field: " , stringify ! ( A ) , "::" , stringify
                ! ( foo ) ));
}
impl Clone for A {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
pub struct B__bindgen_vtable {
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct B {
    pub vtable_: *const B__bindgen_vtable,
    pub bar: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(::std::mem::size_of::<B>() , 16usize , concat ! (
               "Size of: " , stringify ! ( B ) ));
    assert_eq! (::std::mem::align_of::<B>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( B ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const B ) ) . bar as * const _ as usize } ,
                8usize , concat ! (
                "Alignment of field: " , stringify ! ( B ) , "::" , stringify
                ! ( bar ) ));
}
impl Clone for B {
    fn clone(&self) -> Self { *self }
}
impl Default for B {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
pub struct C__bindgen_vtable {
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C {
    pub vtable_: *const C__bindgen_vtable,
    pub baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 16usize , concat ! (
               "Size of: " , stringify ! ( C ) ));
    assert_eq! (::std::mem::align_of::<C>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( C ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const C ) ) . baz as * const _ as usize } ,
                8usize , concat ! (
                "Alignment of field: " , stringify ! ( C ) , "::" , stringify
                ! ( baz ) ));
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
impl Default for C {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct D {
    pub _base: C,
    pub _base_1: B,
    pub bazz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_D() {
    assert_eq!(::std::mem::size_of::<D>() , 40usize , concat ! (
               "Size of: " , stringify ! ( D ) ));
    assert_eq! (::std::mem::align_of::<D>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( D ) ));
}
impl Clone for D {
    fn clone(&self) -> Self { *self }
}
impl Default for D {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
