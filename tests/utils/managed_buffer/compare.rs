use customize_nft::utils::managed_buffer_utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

macro_rules! assert_compare_gt {
    ($a: expr, $b: expr) => {{
        assert!($a.compare(&$a).is_eq());
        assert!($b.compare(&$b).is_eq());

        assert!($a.compare(&$b).is_gt());
        assert!($b.compare(&$a).is_lt());
    }};
}

#[test]
fn returns_zero() {
    DebugApi::dummy();

    let a = ManagedBuffer::<DebugApi>::new_from_bytes(b"hello world");
    let b = ManagedBuffer::<DebugApi>::new_from_bytes(b"hello world");

    assert!(a.compare(&b).is_eq());
}

#[test]
fn gt_lowercase() {
    DebugApi::dummy();

    let greater = ManagedBuffer::<DebugApi>::new_from_bytes(b"ccc");
    let minus = ManagedBuffer::<DebugApi>::new_from_bytes(b"bbb");

    assert_compare_gt!(greater, minus);
}

#[test]
fn gt_number() {
    DebugApi::dummy();

    let greater = ManagedBuffer::<DebugApi>::new_from_bytes(b"5");
    let minus = ManagedBuffer::<DebugApi>::new_from_bytes(b"1");

    assert_compare_gt!(greater, minus);
}

#[test]
fn gt_uppercase() {
    DebugApi::dummy();

    let greater = ManagedBuffer::<DebugApi>::new_from_bytes(b"B");
    let minus = ManagedBuffer::<DebugApi>::new_from_bytes(b"A");

    assert_compare_gt!(greater, minus);
}

#[test]
fn uppercase_first_bis() {
    DebugApi::dummy();

    let greater = ManagedBuffer::<DebugApi>::new_from_bytes(b"A");
    let minus = ManagedBuffer::<DebugApi>::new_from_bytes(b"b");

    assert_compare_gt!(minus, greater);
}

#[test]
fn uppercase_first() {
    DebugApi::dummy();

    let greater = ManagedBuffer::<DebugApi>::new_from_bytes(b"B");
    let minus = ManagedBuffer::<DebugApi>::new_from_bytes(b"a");

    assert_compare_gt!(minus, greater);
}
