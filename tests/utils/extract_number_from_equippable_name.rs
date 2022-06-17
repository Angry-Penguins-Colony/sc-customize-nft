use customize_nft::utils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

#[test]
fn test_get_number_from_penguin_name() {
    DebugApi::dummy();

    assert_eq!(
        utils::extract_number_from_equippable_name(&ManagedBuffer::<DebugApi>::new_from_bytes(
            b"Penguin #1"
        )),
        Some(1)
    );

    assert_eq!(
        utils::extract_number_from_equippable_name(&ManagedBuffer::<DebugApi>::new_from_bytes(
            b"Penguin #15"
        )),
        Some(15)
    );
}
