use std::fmt::Debug;

use free_wrap::Wrap;

// Test simple wrapper
Wrap!(String, StringWrapper, derive = { Debug, Clone });

// Test generic wrapper
Wrap!(Vec<T: Debug>, VecWrapper, derive = { Debug });

// // Test lifetime wrapper
// wrap!(std::slice::Iter<'a, T>, SliceIterWrapper);

#[test]
fn test_simple_wrapper() {
    // Test String wrapper
    let original = String::from("hello");
    let wrapper: StringWrapper = original.clone().into();

    // Test deref
    assert_eq!(wrapper.len(), 5);
    assert_eq!(&*wrapper, "hello");

    // Test conversion back
    let back: String = wrapper.into();
    assert_eq!(back, "hello");
}

#[test]
fn test_generic_wrapper() {
    // Test Vec wrapper
    let original = vec![1, 2, 3];
    let wrapper: VecWrapper<i32> = original.clone().into();

    // Test deref
    assert_eq!(wrapper.len(), 3);
    assert_eq!(&*wrapper, &vec![1, 2, 3]);

    // Test conversion back
    let back: Vec<i32> = wrapper.into();
    assert_eq!(back, vec![1, 2, 3]);
}

// #[test]
// fn test_lifetime_wrapper() {
//     let data = vec![1, 2, 3];
//     let iter = data.iter();
//     let wrapper: SliceIterWrapper<i32> = iter.into();

//     // Test deref and iteration
//     let collected: Vec<&i32> = wrapper.copied().collect();
//     assert_eq!(collected, vec![&1, &2, &3]);
// }
