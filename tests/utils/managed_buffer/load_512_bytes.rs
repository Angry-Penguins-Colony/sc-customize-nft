use customize_nft::utils::managed_buffer_utils::ManagedBufferUtils;
use elrond_wasm::types::ManagedBuffer;
use elrond_wasm_debug::DebugApi;

use crate::testing_utils;

#[test]
fn should_throw_error() {
    let bytes = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam quis porta ligula. Ut mollis dignissim rhoncus. Proin rutrum arcu volutpat mi tincidunt dignissim. In mauris leo, luctus et mollis porttitor, cursus eu tortor. Suspendisse suscipit aliquet suscipit. Pellentesque nunc quam, iaculis nec vulputate sit amet, sodales fermentum libero. Duis ac velit urna. Pellentesque eget mattis nulla. Nulla ultrices erat ac ultricies egestas. Aenean nec sollicitudin ex. Morbi sapien risus, blandit quis est nec, viverra pharetra nulla. Praesent tempor eget nibh at bibendum. Fusce cursus tristique lacus, vel dignissim nisl accumsan nec. Duis laoreet lacinia augue sed iaculis. Nam viverra tempus ligula at accumsan. Vivamus consequat enim volutpat, fringilla nunc in, aliquam mi. Curabitur interdum vulputate ante, vel vulputate urna consectetur eget. Nam dignissim nulla in nisi porta, a finibus lectus pharetra. Vivamus sed accumsan urna, tempus vulputate sem. Nulla facilisi. Etiam vehicula, mi in sollicitudin dignissim, orci tellus sollicitudin libero, ac commodo eros urna vel odio. Vestibulum diam velit, interdum vitae massa a, pharetra tristique ante.";

    let mut setup = testing_utils::setup(customize_nft::contract_obj);

    setup
        .blockchain_wrapper
        .execute_query(&setup.cf_wrapper, |_| {
            ManagedBuffer::<DebugApi>::new_from_bytes(bytes).load_512_bytes();
        })
        .assert_user_error("ManagedBuffer is too big")
}
