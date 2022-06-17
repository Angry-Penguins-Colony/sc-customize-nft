use customize_nft::structs::{equippable_nft_attributes::EquippableNftAttributes, item::Item};
use elrond_wasm::types::{ManagedBuffer, TokenIdentifier};
use elrond_wasm_debug::DebugApi;

#[test]
fn test_to_kvp_buffer() {
    DebugApi::dummy();

    let slot = &&ManagedBuffer::new_from_bytes(b"hat");
    let attributes = EquippableNftAttributes::<DebugApi>::new(&[(
        slot,
        Item {
            token: TokenIdentifier::from_esdt_bytes(b"HAT-aaaa"),
            nonce: 1,
            name: ManagedBuffer::new_from_bytes(b"pirate hat"),
        },
    )]);

    assert_eq!(
        attributes.to_kvp_buffer(slot),
        ManagedBuffer::new_from_bytes(b"Hat:pirate hat (HAT-aaaa-01)")
    );
}
