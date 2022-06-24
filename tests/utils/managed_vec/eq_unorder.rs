use customize_nft::utils::managed_vec_utils::ManagedVecUtils;
use elrond_wasm::types::ManagedVec;
use elrond_wasm_debug::DebugApi;

macro_rules! managed_vec [
    ($vec_type: tt, $($e:expr),*) => ({
        let mut _temp = ::std::vec::Vec::<$vec_type>::new();
        $(_temp.push($e);)*
        ManagedVec::<DebugApi, u64>::from(_temp)
    })
];

macro_rules! assert_eq_unorder {
    ($a: expr, $b: expr) => {
        assert!($a.eq_unorder(&$b));
        assert!($b.eq_unorder(&$a));
    };
}

macro_rules! assert_ne_unorder {
    ($a: expr, $b: expr) => {
        assert_eq!($a.eq_unorder(&$b), false);
        assert_eq!($b.eq_unorder(&$a), false);
    };
}

#[test]
fn true_if_both_empty() {
    DebugApi::dummy();

    let a = ManagedVec::<DebugApi, u64>::new();
    let b = ManagedVec::<DebugApi, u64>::new();

    assert_eq_unorder!(a, b);
}

#[test]
fn true_if_same_size_and_same_order() {
    DebugApi::dummy();

    let a = managed_vec!(u64, 0u64, 3u64);
    let b = managed_vec!(u64, 0u64, 3u64);

    assert_eq_unorder!(a, b);
}

#[test]
fn false_if_same_size_but_different_order() {
    DebugApi::dummy();

    let a = managed_vec!(u64, 1u64, 3u64);
    let b = managed_vec!(u64, 0u64, 3u64);

    assert_ne_unorder!(a, b);
}

#[test]
fn false_if_same_size_but_different_order_plus_contains_duplicate() {
    DebugApi::dummy();

    let a = managed_vec!(u64, 1u64, 1u64, 3u64);
    let b = managed_vec!(u64, 1u64, 1u64, 5u64);

    assert_ne_unorder!(a, b);
}

#[test]
fn false_if_different_size_and_different_order() {
    DebugApi::dummy();

    let a = managed_vec!(u64, 3u64, 0u64, 1u64);
    let b = managed_vec!(u64, 0u64, 3u64);

    assert_ne_unorder!(a, b);
}

#[test]
fn false_if_different_size() {
    DebugApi::dummy();

    let a = ManagedVec::<DebugApi, u64>::new();
    let b = managed_vec!(u64, 0u64);

    assert_ne_unorder!(a, b);
}
