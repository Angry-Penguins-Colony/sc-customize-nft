#![feature(prelude_import)]
#![no_std]
#![no_main]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use elrond_wasm::{
    api::{
        BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl, CryptoApi,
        CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi, LogApiImpl,
        ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
    },
    arrayvec::ArrayVec,
    contract_base::{ContractBase, ProxyObjBase},
    elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
    err_msg,
    esdt::*,
    io::*,
    non_zero_usize,
    non_zero_util::*,
    require, require_old, sc_error, sc_panic, sc_print,
    storage::mappers::*,
    types::{
        SCResult::{Err, Ok},
        *,
    },
    Box, Vec,
};
use elrond_wasm::{
    derive::{ManagedVecItem, TypeAbi},
    elrond_codec,
    elrond_codec::elrond_codec_derive::{
        NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode, TopEncodeOrDefault,
    },
};
extern crate alloc;
pub mod libs {
    pub mod penguin_mint {
        use crate::structs::{item_slot::ItemSlot, penguin_attributes::PenguinAttributes};
        use alloc::string::ToString;
        use core::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        };
        use elrond_wasm::{
            api::{
                BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl,
                CryptoApi, CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi,
                LogApiImpl, ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
            },
            arrayvec::ArrayVec,
            contract_base::{ContractBase, ProxyObjBase},
            elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
            err_msg,
            esdt::*,
            io::*,
            non_zero_usize,
            non_zero_util::*,
            require, require_old, sc_error, sc_panic, sc_print,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
            Box, Vec,
        };
        use elrond_wasm::{
            derive::{ManagedVecItem, TypeAbi},
            elrond_codec,
            elrond_codec::elrond_codec_derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
        };
        use elrond_wasm::{
            elrond_codec::TopEncode,
            types::{ManagedBuffer, ManagedByteArray, ManagedVec, SCResult},
        };
        pub trait MintPenguin:
            elrond_wasm::contract_base::ContractBase
            + Sized
            + super::storage::StorageModule
            + super::penguin_parse::ParsePenguin
        {
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn update_penguin(
                &self,
                penguin_id: &elrond_wasm::types::TokenIdentifier<Self::Api>,
                penguin_nonce: u64,
                attributes: &PenguinAttributes<Self::Api>,
            ) -> SCResult<u64> {
                let caller = self.blockchain().get_caller();
                let token_nonce =
                    self.mint_penguin(attributes, &self.get_penguin_name(penguin_nonce))?;
                self.send().esdt_local_burn(
                    &penguin_id,
                    penguin_nonce,
                    &elrond_wasm::types::BigUint::<Self::Api>::from(1u32),
                );
                self.send().direct(
                    &caller,
                    &penguin_id,
                    token_nonce,
                    &elrond_wasm::types::BigUint::<Self::Api>::from(1u32),
                    &[],
                );
                return SCResult::Ok(token_nonce);
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn get_penguin_name(&self, penguin_nonce: u64) -> ManagedBuffer<Self::Api> {
                let nft_data = self.blockchain().get_esdt_token_data(
                    &self.blockchain().get_sc_address(),
                    &self.penguins_identifier().get(),
                    penguin_nonce,
                );
                return nft_data.name;
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn mint_penguin(
                &self,
                attributes: &PenguinAttributes<Self::Api>,
                name: &elrond_wasm::types::ManagedBuffer<Self::Api>,
            ) -> SCResult<u64> {
                let penguin_id = self.penguins_identifier().get();
                let mut uris = ManagedVec::new();
                uris.push(self.build_url(&attributes)?);
                let token_nonce = self.send().esdt_nft_create::<PenguinAttributes<Self::Api>>(
                    &penguin_id,
                    &elrond_wasm::types::BigUint::<Self::Api>::from(1u32),
                    &name,
                    &elrond_wasm::types::BigUint::<Self::Api>::zero(),
                    &self.calculate_hash(&attributes)?,
                    &attributes,
                    &uris,
                );
                return SCResult::Ok(token_nonce);
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn calculate_hash(
                &self,
                _attributes: &PenguinAttributes<Self::Api>,
            ) -> SCResult<elrond_wasm::types::ManagedBuffer<Self::Api>> {
                return SCResult::Ok(elrond_wasm::types::ManagedBuffer::<Self::Api>::new());
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn build_url(
                &self,
                attributes: &PenguinAttributes<Self::Api>,
            ) -> SCResult<ManagedBuffer<Self::Api>> {
                if attributes.get_fill_count() == 0 {
                    return SCResult::Ok(self.get_full_unequiped_penguin_uri());
                }
                let mut expected = elrond_wasm::types::ManagedBuffer::<Self::Api>::new();
                expected.append(&self.uri().get());
                let mut is_first_item = true;
                for slot in ItemSlot::VALUES.iter() {
                    if let Some(item) = attributes.get_item(slot) {
                        let token_data = self.parse_item_attributes(&item.token, item.nonce);
                        let slot_type = token_data.item_id;
                        let slot_id = slot.to_bytes::<Self::Api>();
                        if is_first_item == false {
                            expected.append_bytes(b"+");
                        }
                        expected.append(
                            &elrond_wasm::types::ManagedBuffer::<Self::Api>::new_from_bytes(
                                slot_id,
                            ),
                        );
                        expected.append_bytes(b"_");
                        expected.append(&slot_type);
                        is_first_item = false;
                    }
                }
                expected.append_bytes(b"/image.png");
                return SCResult::Ok(expected);
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn get_full_unequiped_penguin_uri(&self) -> ManagedBuffer<Self::Api> {
                let mut uri = elrond_wasm::types::ManagedBuffer::<Self::Api>::new();
                uri.append(&self.uri().get());
                uri.append_bytes(b"empty/image.png");
                return uri;
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn get_next_penguin_name(&self) -> elrond_wasm::types::ManagedBuffer<Self::Api> {
                let penguin_id = self.penguins_identifier().get();
                let index = self
                    .blockchain()
                    .get_current_esdt_nft_nonce(&self.blockchain().get_sc_address(), &penguin_id)
                    + 1;
                let mut full_token_name = elrond_wasm::types::ManagedBuffer::<Self::Api>::new();
                let token_name_from_storage =
                    elrond_wasm::types::ManagedBuffer::<Self::Api>::new_from_bytes(b"Penguin");
                let hash_sign =
                    elrond_wasm::types::ManagedBuffer::<Self::Api>::new_from_bytes(" #".as_bytes());
                let token_index = elrond_wasm::types::ManagedBuffer::<Self::Api>::new_from_bytes(
                    index.to_string().as_bytes(),
                );
                full_token_name.append(&token_name_from_storage);
                full_token_name.append(&hash_sign);
                full_token_name.append(&token_index);
                return full_token_name;
            }
        }
        pub trait AutoImpl: elrond_wasm::contract_base::ContractBase {}
        impl<C> MintPenguin for C where
            C: AutoImpl + super::storage::StorageModule + super::penguin_parse::ParsePenguin
        {
        }
        pub trait EndpointWrappers:
            elrond_wasm::contract_base::ContractBase
            + MintPenguin
            + super::storage::EndpointWrappers
            + super::penguin_parse::EndpointWrappers
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                if match fn_name {
                    b"callBack"
                        if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                            elrond_wasm::abi::EndpointLocationAbi::MainContract,
                        ) =>
                    {
                        self::EndpointWrappers::callback(self);
                        return true;
                    }
                    b"init"
                        if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                            elrond_wasm::abi::EndpointLocationAbi::ViewContract,
                        ) =>
                    {
                        elrond_wasm::external_view_contract::external_view_contract_constructor::<
                            Self::Api,
                        >();
                        return true;
                    }
                    other => false,
                } {
                    return true;
                }
                if super::storage::EndpointWrappers::call(self, fn_name) {
                    return true;
                }
                if super::penguin_parse::EndpointWrappers::call(self, fn_name) {
                    return true;
                }
                false
            }
            fn callback_selector(
                &self,
                mut ___cb_closure___: elrond_wasm::types::CallbackClosureForDeser<Self::Api>,
            ) -> elrond_wasm::types::CallbackSelectorResult<Self::Api> {
                let mut ___call_result_loader___ =
                    elrond_wasm::io::EndpointDynArgLoader::<Self::Api>::new();
                let ___cb_closure_matcher___ = ___cb_closure___.matcher::<32usize>();
                if ___cb_closure_matcher___.matches_empty() {
                    return elrond_wasm::types::CallbackSelectorResult::Processed;
                }
                match super::storage::EndpointWrappers::callback_selector(self, ___cb_closure___) {
                    elrond_wasm::types::CallbackSelectorResult::Processed => {
                        return elrond_wasm::types::CallbackSelectorResult::Processed;
                    }
                    elrond_wasm::types::CallbackSelectorResult::NotProcessed(
                        recovered_cb_closure,
                    ) => {
                        ___cb_closure___ = recovered_cb_closure;
                    }
                }
                match super::penguin_parse::EndpointWrappers::callback_selector(
                    self,
                    ___cb_closure___,
                ) {
                    elrond_wasm::types::CallbackSelectorResult::Processed => {
                        return elrond_wasm::types::CallbackSelectorResult::Processed;
                    }
                    elrond_wasm::types::CallbackSelectorResult::NotProcessed(
                        recovered_cb_closure,
                    ) => {
                        ___cb_closure___ = recovered_cb_closure;
                    }
                }
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
            }
            fn callback(&self) {
                if let Some(___cb_closure___) =
                    elrond_wasm::types::CallbackClosureForDeser::storage_load_and_clear::<Self::Api>(
                    )
                {
                    if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                        self::EndpointWrappers::callback_selector(self, ___cb_closure___)
                    {
                        elrond_wasm::api::ErrorApiImpl::signal_error(
                            &Self::Api::error_api_impl(),
                            err_msg::CALLBACK_BAD_FUNC,
                        );
                    }
                }
            }
        }
        pub struct AbiProvider {}
        impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
            type Api = elrond_wasm::api::uncallable::UncallableApi;
            fn abi() -> elrond_wasm::abi::ContractAbi {
                let mut contract_abi = elrond_wasm :: abi :: ContractAbi { build_info : elrond_wasm :: abi :: BuildInfoAbi { contract_crate : elrond_wasm :: abi :: ContractCrateBuildAbi { name : "equip_penguin" , version : "0.0.0" , git_version : "N/A" , } , framework : elrond_wasm :: abi :: FrameworkBuildAbi :: create () , } , docs : & [] , name : "MintPenguin" , constructors : Vec :: new () , endpoints : Vec :: new () , has_callback : false , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
                contract_abi
            }
        }
        pub struct ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            _phantom: core::marker::PhantomData<A>,
        }
        impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            type Api = A;
        }
        impl<A> super::storage::AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> super::penguin_parse::AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> super::storage::EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> super::penguin_parse::EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> elrond_wasm::contract_base::CallableContract for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
            fn clone_obj(
                &self,
            ) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
                elrond_wasm::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            ContractObj {
                _phantom: core::marker::PhantomData,
            }
        }
        pub struct ContractBuilder;
        impl elrond_wasm::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: elrond_wasm::api::VMApi>(
                &self,
            ) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
                elrond_wasm::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
        pub use super::penguin_parse::endpoints as __endpoints_1__;
        pub use super::storage::endpoints as __endpoints_0__;
        #[allow(non_snake_case)]
        pub mod endpoints {
            use super::EndpointWrappers;
            pub use super::__endpoints_0__::*;
            pub use super::__endpoints_1__::*;
        }
        pub trait ProxyTrait:
            elrond_wasm::contract_base::ProxyObjBase
            + Sized
            + super::storage::ProxyTrait
            + super::penguin_parse::ProxyTrait
        {
        }
    }
    pub mod penguin_parse {
        use crate::structs::{
            item_attributes::ItemAttributes, penguin_attributes::PenguinAttributes,
        };
        use alloc::string::ToString;
        use core::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        };
        use elrond_wasm::{
            api::{
                BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl,
                CryptoApi, CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi,
                LogApiImpl, ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
            },
            arrayvec::ArrayVec,
            contract_base::{ContractBase, ProxyObjBase},
            elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
            err_msg,
            esdt::*,
            io::*,
            non_zero_usize,
            non_zero_util::*,
            require, require_old, sc_error, sc_panic, sc_print,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
            Box, Vec,
        };
        use elrond_wasm::{
            derive::{ManagedVecItem, TypeAbi},
            elrond_codec,
            elrond_codec::elrond_codec_derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
        };
        use elrond_wasm::{
            elrond_codec::TopEncode,
            types::{ManagedBuffer, ManagedByteArray, ManagedVec, SCResult},
        };
        pub trait ParsePenguin: elrond_wasm::contract_base::ContractBase + Sized {
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn parse_penguin_attributes(
                &self,
                penguin_id: &elrond_wasm::types::TokenIdentifier<Self::Api>,
                penguin_nonce: u64,
            ) -> PenguinAttributes<Self::Api> {
                let attributes = self
                    .blockchain()
                    .get_esdt_token_data(
                        &self.blockchain().get_sc_address(),
                        &penguin_id,
                        penguin_nonce,
                    )
                    .decode_attributes::<PenguinAttributes<Self::Api>>();
                return attributes;
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn parse_item_attributes(
                &self,
                id: &elrond_wasm::types::TokenIdentifier<Self::Api>,
                nonce: u64,
            ) -> ItemAttributes<Self::Api> {
                let attributes = self
                    .blockchain()
                    .get_esdt_token_data(&self.blockchain().get_sc_address(), &id, nonce)
                    .attributes;
                return ItemAttributes {
                    item_id: attributes,
                };
            }
        }
        pub trait AutoImpl: elrond_wasm::contract_base::ContractBase {}
        impl<C> ParsePenguin for C where C: AutoImpl {}
        pub trait EndpointWrappers:
            elrond_wasm::contract_base::ContractBase + ParsePenguin
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                if match fn_name {
                    b"callBack"
                        if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                            elrond_wasm::abi::EndpointLocationAbi::MainContract,
                        ) =>
                    {
                        self::EndpointWrappers::callback(self);
                        return true;
                    }
                    b"init"
                        if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                            elrond_wasm::abi::EndpointLocationAbi::ViewContract,
                        ) =>
                    {
                        elrond_wasm::external_view_contract::external_view_contract_constructor::<
                            Self::Api,
                        >();
                        return true;
                    }
                    other => false,
                } {
                    return true;
                }
                false
            }
            fn callback_selector(
                &self,
                mut ___cb_closure___: elrond_wasm::types::CallbackClosureForDeser<Self::Api>,
            ) -> elrond_wasm::types::CallbackSelectorResult<Self::Api> {
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
            }
            fn callback(&self) {}
        }
        pub struct AbiProvider {}
        impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
            type Api = elrond_wasm::api::uncallable::UncallableApi;
            fn abi() -> elrond_wasm::abi::ContractAbi {
                let mut contract_abi = elrond_wasm :: abi :: ContractAbi { build_info : elrond_wasm :: abi :: BuildInfoAbi { contract_crate : elrond_wasm :: abi :: ContractCrateBuildAbi { name : "equip_penguin" , version : "0.0.0" , git_version : "N/A" , } , framework : elrond_wasm :: abi :: FrameworkBuildAbi :: create () , } , docs : & [] , name : "ParsePenguin" , constructors : Vec :: new () , endpoints : Vec :: new () , has_callback : false , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
                contract_abi
            }
        }
        pub struct ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            _phantom: core::marker::PhantomData<A>,
        }
        impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            type Api = A;
        }
        impl<A> AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> elrond_wasm::contract_base::CallableContract for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
            fn clone_obj(
                &self,
            ) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
                elrond_wasm::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            ContractObj {
                _phantom: core::marker::PhantomData,
            }
        }
        pub struct ContractBuilder;
        impl elrond_wasm::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: elrond_wasm::api::VMApi>(
                &self,
            ) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
                elrond_wasm::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
        #[allow(non_snake_case)]
        pub mod endpoints {
            use super::EndpointWrappers;
        }
        pub trait ProxyTrait: elrond_wasm::contract_base::ProxyObjBase + Sized {}
    }
    pub mod storage {
        use crate::structs::item_slot::ItemSlot;
        use core::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        };
        use elrond_wasm::{
            api::{
                BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl,
                CryptoApi, CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi,
                LogApiImpl, ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
            },
            arrayvec::ArrayVec,
            contract_base::{ContractBase, ProxyObjBase},
            elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
            err_msg,
            esdt::*,
            io::*,
            non_zero_usize,
            non_zero_util::*,
            require, require_old, sc_error, sc_panic, sc_print,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
            Box, Vec,
        };
        use elrond_wasm::{
            derive::{ManagedVecItem, TypeAbi},
            elrond_codec,
            elrond_codec::elrond_codec_derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
        };
        pub trait StorageModule: elrond_wasm::contract_base::ContractBase + Sized {
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn items_slot(
                &self,
                token: &elrond_wasm::types::TokenIdentifier<Self::Api>,
            ) -> SingleValueMapper<Self::Api, ItemSlot>;
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn penguins_identifier(
                &self,
            ) -> SingleValueMapper<Self::Api, elrond_wasm::types::TokenIdentifier<Self::Api>>;
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn uri(&self) -> SingleValueMapper<Self::Api, ManagedBuffer<Self::Api>>;
        }
        pub trait AutoImpl: elrond_wasm::contract_base::ContractBase {}
        impl<C> StorageModule for C
        where
            C: AutoImpl,
        {
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn items_slot(
                &self,
                token: &elrond_wasm::types::TokenIdentifier<Self::Api>,
            ) -> SingleValueMapper<Self::Api, ItemSlot> {
                let mut ___key___ =
                    elrond_wasm::storage::StorageKey::<Self::Api>::new(&b"items_types"[..]);
                ___key___.append_item(&token);
                < SingleValueMapper < Self :: Api , ItemSlot > as elrond_wasm :: storage :: mappers :: StorageMapper < Self :: Api > > :: new (___key___)
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn penguins_identifier(
                &self,
            ) -> SingleValueMapper<Self::Api, elrond_wasm::types::TokenIdentifier<Self::Api>>
            {
                let mut ___key___ =
                    elrond_wasm::storage::StorageKey::<Self::Api>::new(&b"penguins_identifier"[..]);
                < SingleValueMapper < Self :: Api , elrond_wasm :: types :: TokenIdentifier < Self :: Api > > as elrond_wasm :: storage :: mappers :: StorageMapper < Self :: Api > > :: new (___key___)
            }
            #[allow(clippy::too_many_arguments)]
            #[allow(clippy::type_complexity)]
            fn uri(&self) -> SingleValueMapper<Self::Api, ManagedBuffer<Self::Api>> {
                let mut ___key___ = elrond_wasm::storage::StorageKey::<Self::Api>::new(&b"uri"[..]);
                < SingleValueMapper < Self :: Api , ManagedBuffer < Self :: Api > > as elrond_wasm :: storage :: mappers :: StorageMapper < Self :: Api > > :: new (___key___)
            }
        }
        pub trait EndpointWrappers:
            elrond_wasm::contract_base::ContractBase + StorageModule
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                if match fn_name {
                    b"callBack"
                        if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                            elrond_wasm::abi::EndpointLocationAbi::MainContract,
                        ) =>
                    {
                        self::EndpointWrappers::callback(self);
                        return true;
                    }
                    b"init"
                        if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                            elrond_wasm::abi::EndpointLocationAbi::ViewContract,
                        ) =>
                    {
                        elrond_wasm::external_view_contract::external_view_contract_constructor::<
                            Self::Api,
                        >();
                        return true;
                    }
                    other => false,
                } {
                    return true;
                }
                false
            }
            fn callback_selector(
                &self,
                mut ___cb_closure___: elrond_wasm::types::CallbackClosureForDeser<Self::Api>,
            ) -> elrond_wasm::types::CallbackSelectorResult<Self::Api> {
                elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
            }
            fn callback(&self) {}
        }
        pub struct AbiProvider {}
        impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
            type Api = elrond_wasm::api::uncallable::UncallableApi;
            fn abi() -> elrond_wasm::abi::ContractAbi {
                let mut contract_abi = elrond_wasm :: abi :: ContractAbi { build_info : elrond_wasm :: abi :: BuildInfoAbi { contract_crate : elrond_wasm :: abi :: ContractCrateBuildAbi { name : "equip_penguin" , version : "0.0.0" , git_version : "N/A" , } , framework : elrond_wasm :: abi :: FrameworkBuildAbi :: create () , } , docs : & [] , name : "StorageModule" , constructors : Vec :: new () , endpoints : Vec :: new () , has_callback : false , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
                contract_abi
            }
        }
        pub struct ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            _phantom: core::marker::PhantomData<A>,
        }
        impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            type Api = A;
        }
        impl<A> AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
        impl<A> elrond_wasm::contract_base::CallableContract for ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            fn call(&self, fn_name: &[u8]) -> bool {
                EndpointWrappers::call(self, fn_name)
            }
            fn clone_obj(
                &self,
            ) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
                elrond_wasm::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
        pub fn contract_obj<A>() -> ContractObj<A>
        where
            A: elrond_wasm::api::VMApi,
        {
            ContractObj {
                _phantom: core::marker::PhantomData,
            }
        }
        pub struct ContractBuilder;
        impl elrond_wasm::contract_base::CallableContractBuilder for self::ContractBuilder {
            fn new_contract_obj<A: elrond_wasm::api::VMApi>(
                &self,
            ) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
                elrond_wasm::Box::new(ContractObj::<A> {
                    _phantom: core::marker::PhantomData,
                })
            }
        }
        #[allow(non_snake_case)]
        pub mod endpoints {
            use super::EndpointWrappers;
        }
        pub trait ProxyTrait: elrond_wasm::contract_base::ProxyObjBase + Sized {}
    }
}
pub mod structs {
    pub mod item {
        #![no_std]
        #![no_main]
        #![allow(unused_attributes)]
        #![allow(unused_imports)]
        use super::item_slot::ItemSlot;
        use alloc::{borrow::ToOwned, format, string::ToString};
        use core::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        };
        use core::str::FromStr;
        use elrond_wasm::{
            api::{
                BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl,
                CryptoApi, CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi,
                LogApiImpl, ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
            },
            arrayvec::ArrayVec,
            contract_base::{ContractBase, ProxyObjBase},
            elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
            err_msg,
            esdt::*,
            io::*,
            non_zero_usize,
            non_zero_util::*,
            require, require_old, sc_error, sc_panic, sc_print,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
            Box, Vec,
        };
        use elrond_wasm::{
            derive::{ManagedVecItem, TypeAbi},
            elrond_codec,
            elrond_codec::elrond_codec_derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
        };
        use elrond_wasm::{elrond_codec::TopDecodeInput, String};
        pub struct Item<M: ManagedTypeApi> {
            pub token: TokenIdentifier<M>,
            pub nonce: u64,
            pub name: ManagedBuffer<M>,
        }
        impl<M: ManagedTypeApi> elrond_codec::TopEncode for Item<M> {
            fn top_encode_or_handle_err<O, H>(
                &self,
                output: O,
                h: H,
            ) -> core::result::Result<(), H::HandledErr>
            where
                O: elrond_codec::TopEncodeOutput,
                H: elrond_codec::EncodeErrorHandler,
            {
                let mut buffer = output.start_nested_encode();
                let dest = &mut buffer;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.token, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.nonce, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.name, dest, h)?;
                output.finalize_nested_encode(buffer);
                core::result::Result::Ok(())
            }
        }
        impl<M: ManagedTypeApi> elrond_codec::TopDecode for Item<M> {
            fn top_decode_or_handle_err<I, H>(
                top_input: I,
                h: H,
            ) -> core::result::Result<Self, H::HandledErr>
            where
                I: elrond_codec::TopDecodeInput,
                H: elrond_codec::DecodeErrorHandler,
            {
                let mut nested_buffer = top_input.into_nested_buffer();
                let result = Item { 
                    token : < TokenIdentifier < M > as elrond_codec :: NestedDecode > :: dep_decode_or_handle_err (& mut nested_buffer , h) ? , 
                    nonce : < u64 as elrond_codec :: NestedDecode > :: dep_decode_or_handle_err (& mut nested_buffer , h) ? , 
                    name : < ManagedBuffer < M > as elrond_codec :: NestedDecode > :: dep_decode_or_handle_err (& mut nested_buffer , h) ? , 
                } ;
                if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                    return core::result::Result::Err(
                        h.handle_error(elrond_codec::DecodeError::INPUT_TOO_LONG),
                    );
                }
                core::result::Result::Ok(result)
            }
        }
        impl<M: ManagedTypeApi> elrond_codec::NestedEncode for Item<M> {
            fn dep_encode_or_handle_err<O, H>(
                &self,
                dest: &mut O,
                h: H,
            ) -> core::result::Result<(), H::HandledErr>
            where
                O: elrond_codec::NestedEncodeOutput,
                H: elrond_codec::EncodeErrorHandler,
            {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.token, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.nonce, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.name, dest, h)?;
                core::result::Result::Ok(())
            }
        }
        impl<M: ManagedTypeApi> elrond_codec::NestedDecode for Item<M> {
            fn dep_decode_or_handle_err<I, H>(
                input: &mut I,
                h: H,
            ) -> core::result::Result<Self, H::HandledErr>
            where
                I: elrond_codec::NestedDecodeInput,
                H: elrond_codec::DecodeErrorHandler,
            {
                core :: result :: Result :: Ok (Item { token : < TokenIdentifier < M > as elrond_codec :: NestedDecode > :: dep_decode_or_handle_err (input , h) ? , nonce : < u64 as elrond_codec :: NestedDecode > :: dep_decode_or_handle_err (input , h) ? , name : < ManagedBuffer < M > as elrond_codec :: NestedDecode > :: dep_decode_or_handle_err (input , h) ? , })
            }
        }
        impl<M: ManagedTypeApi> elrond_wasm::abi::TypeAbi for Item<M> {
            fn type_name() -> elrond_wasm::String {
                "Item".into()
            }
            fn provide_type_descriptions<TDC: elrond_wasm::abi::TypeDescriptionContainer>(
                accumulator: &mut TDC,
            ) {
                let type_name = Self::type_name();
                if !accumulator.contains_type(&type_name) {
                    accumulator.reserve_type_name(type_name.clone());
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "token",
                        field_type: <TokenIdentifier<M>>::type_name(),
                    });
                    <TokenIdentifier<M>>::provide_type_descriptions(accumulator);
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "nonce",
                        field_type: <u64>::type_name(),
                    });
                    <u64>::provide_type_descriptions(accumulator);
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "name",
                        field_type: <ManagedBuffer<M>>::type_name(),
                    });
                    <ManagedBuffer<M>>::provide_type_descriptions(accumulator);
                    accumulator.insert(
                        type_name.clone(),
                        elrond_wasm::abi::TypeDescription {
                            docs: &[],
                            name: type_name,
                            contents: elrond_wasm::abi::TypeContents::Struct(field_descriptions),
                        },
                    );
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<M: ::core::clone::Clone + ManagedTypeApi> ::core::clone::Clone for Item<M> {
            #[inline]
            fn clone(&self) -> Item<M> {
                match *self {
                    Item {
                        token: ref __self_0_0,
                        nonce: ref __self_0_1,
                        name: ref __self_0_2,
                    } => Item {
                        token: ::core::clone::Clone::clone(&(*__self_0_0)),
                        nonce: ::core::clone::Clone::clone(&(*__self_0_1)),
                        name: ::core::clone::Clone::clone(&(*__self_0_2)),
                    },
                }
            }
        }
        impl<M: ManagedTypeApi> ::core::marker::StructuralPartialEq for Item<M> {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<M: ::core::cmp::PartialEq + ManagedTypeApi> ::core::cmp::PartialEq for Item<M> {
            #[inline]
            fn eq(&self, other: &Item<M>) -> bool {
                match *other {
                    Item {
                        token: ref __self_1_0,
                        nonce: ref __self_1_1,
                        name: ref __self_1_2,
                    } => match *self {
                        Item {
                            token: ref __self_0_0,
                            nonce: ref __self_0_1,
                            name: ref __self_0_2,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &Item<M>) -> bool {
                match *other {
                    Item {
                        token: ref __self_1_0,
                        nonce: ref __self_1_1,
                        name: ref __self_1_2,
                    } => match *self {
                        Item {
                            token: ref __self_0_0,
                            nonce: ref __self_0_1,
                            name: ref __self_0_2,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                        }
                    },
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<M: ::core::fmt::Debug + ManagedTypeApi> ::core::fmt::Debug for Item<M> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    Item {
                        token: ref __self_0_0,
                        nonce: ref __self_0_1,
                        name: ref __self_0_2,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "Item");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "token",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "nonce",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "name",
                            &&(*__self_0_2),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<M: ManagedTypeApi> Item<M> {
            fn split_last_occurence(bytes: &[u8], char: u8) -> (&[u8], &[u8]) {
                for i in (0..bytes.len() - 1).rev() {
                    if bytes[i] == char {
                        return bytes.split_at(i);
                    }
                }
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["no occurence of char ", " in bytes "],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&char),
                        ::core::fmt::ArgumentV1::new_debug(&bytes),
                    ],
                ));
            }
            pub fn u64_to_hex(val: &u64) -> ManagedBuffer<M> {
                let hex_val = {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_lower_hex(&val)],
                    ));
                    res
                };
                let bytes = hex_val.as_bytes();
                if &bytes.len() % 2 != 0 {
                    let mut o = ManagedBuffer::<M>::new();
                    o.append_bytes(b"0");
                    o.append_bytes(bytes);
                    return o;
                } else {
                    return ManagedBuffer::<M>::new_from_bytes(bytes);
                }
            }
        }
    }
    pub mod item_attributes {
        #![no_std]
        #![no_main]
        #![allow(unused_attributes)]
        #![allow(unused_imports)]
        use core::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        };
        use elrond_wasm::{
            api::{
                BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl,
                CryptoApi, CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi,
                LogApiImpl, ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
            },
            arrayvec::ArrayVec,
            contract_base::{ContractBase, ProxyObjBase},
            elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
            err_msg,
            esdt::*,
            io::*,
            non_zero_usize,
            non_zero_util::*,
            require, require_old, sc_error, sc_panic, sc_print,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
            Box, Vec,
        };
        use elrond_wasm::{
            derive::{ManagedVecItem, TypeAbi},
            elrond_codec,
            elrond_codec::elrond_codec_derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
        };
        pub struct ItemAttributes<M: ManagedTypeApi> {
            pub item_id: ManagedBuffer<M>,
        }
        impl<M: ManagedTypeApi> elrond_codec::TopEncode for ItemAttributes<M> {
            fn top_encode_or_handle_err<O, H>(
                &self,
                output: O,
                h: H,
            ) -> core::result::Result<(), H::HandledErr>
            where
                O: elrond_codec::TopEncodeOutput,
                H: elrond_codec::EncodeErrorHandler,
            {
                let mut buffer = output.start_nested_encode();
                let dest = &mut buffer;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.item_id, dest, h)?;
                output.finalize_nested_encode(buffer);
                core::result::Result::Ok(())
            }
        }
        impl<M: ManagedTypeApi> elrond_codec::TopDecode for ItemAttributes<M> {
            fn top_decode_or_handle_err<I, H>(
                top_input: I,
                h: H,
            ) -> core::result::Result<Self, H::HandledErr>
            where
                I: elrond_codec::TopDecodeInput,
                H: elrond_codec::DecodeErrorHandler,
            {
                let mut nested_buffer = top_input.into_nested_buffer();
                let result = ItemAttributes {
                    item_id:
                        <ManagedBuffer<M> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                };
                if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                    return core::result::Result::Err(
                        h.handle_error(elrond_codec::DecodeError::INPUT_TOO_LONG),
                    );
                }
                core::result::Result::Ok(result)
            }
        }
        impl<M: ManagedTypeApi> elrond_codec::NestedEncode for ItemAttributes<M> {
            fn dep_encode_or_handle_err<O, H>(
                &self,
                dest: &mut O,
                h: H,
            ) -> core::result::Result<(), H::HandledErr>
            where
                O: elrond_codec::NestedEncodeOutput,
                H: elrond_codec::EncodeErrorHandler,
            {
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.item_id, dest, h)?;
                core::result::Result::Ok(())
            }
        }
        impl<M: ManagedTypeApi> elrond_codec::NestedDecode for ItemAttributes<M> {
            fn dep_decode_or_handle_err<I, H>(
                input: &mut I,
                h: H,
            ) -> core::result::Result<Self, H::HandledErr>
            where
                I: elrond_codec::NestedDecodeInput,
                H: elrond_codec::DecodeErrorHandler,
            {
                core::result::Result::Ok(ItemAttributes {
                    item_id:
                        <ManagedBuffer<M> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            input, h,
                        )?,
                })
            }
        }
        impl<M: ManagedTypeApi> ::core::marker::StructuralPartialEq for ItemAttributes<M> {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<M: ::core::cmp::PartialEq + ManagedTypeApi> ::core::cmp::PartialEq for ItemAttributes<M> {
            #[inline]
            fn eq(&self, other: &ItemAttributes<M>) -> bool {
                match *other {
                    ItemAttributes {
                        item_id: ref __self_1_0,
                    } => match *self {
                        ItemAttributes {
                            item_id: ref __self_0_0,
                        } => (*__self_0_0) == (*__self_1_0),
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &ItemAttributes<M>) -> bool {
                match *other {
                    ItemAttributes {
                        item_id: ref __self_1_0,
                    } => match *self {
                        ItemAttributes {
                            item_id: ref __self_0_0,
                        } => (*__self_0_0) != (*__self_1_0),
                    },
                }
            }
        }
        impl<M: ManagedTypeApi> elrond_wasm::abi::TypeAbi for ItemAttributes<M> {
            fn type_name() -> elrond_wasm::String {
                "ItemAttributes".into()
            }
            fn provide_type_descriptions<TDC: elrond_wasm::abi::TypeDescriptionContainer>(
                accumulator: &mut TDC,
            ) {
                let type_name = Self::type_name();
                if !accumulator.contains_type(&type_name) {
                    accumulator.reserve_type_name(type_name.clone());
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "item_id",
                        field_type: <ManagedBuffer<M>>::type_name(),
                    });
                    <ManagedBuffer<M>>::provide_type_descriptions(accumulator);
                    accumulator.insert(
                        type_name.clone(),
                        elrond_wasm::abi::TypeDescription {
                            docs: &[],
                            name: type_name,
                            contents: elrond_wasm::abi::TypeContents::Struct(field_descriptions),
                        },
                    );
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<M: ::core::fmt::Debug + ManagedTypeApi> ::core::fmt::Debug for ItemAttributes<M> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    ItemAttributes {
                        item_id: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "ItemAttributes");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "item_id",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<M: ManagedTypeApi> ItemAttributes<M> {
            pub fn random() -> Self {
                ItemAttributes {
                    item_id: ManagedBuffer::new_random(4),
                }
            }
        }
    }
    pub mod item_slot {
        #![no_std]
        #![no_main]
        #![allow(unused_attributes)]
        #![allow(unused_imports)]
        use core::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        };
        use elrond_wasm::{
            api::{
                BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl,
                CryptoApi, CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi,
                LogApiImpl, ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
            },
            arrayvec::ArrayVec,
            contract_base::{ContractBase, ProxyObjBase},
            elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
            err_msg,
            esdt::*,
            io::*,
            non_zero_usize,
            non_zero_util::*,
            require, require_old, sc_error, sc_panic, sc_print,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
            Box, Vec,
        };
        use elrond_wasm::{
            derive::{ManagedVecItem, TypeAbi},
            elrond_codec,
            elrond_codec::elrond_codec_derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
        };
        pub enum ItemSlot {
            None,
            Hat,
            Background,
            Skin,
            Beak,
            Weapon,
            Clothes,
            Eye,
        }
        impl elrond_codec::NestedEncode for ItemSlot {
            fn dep_encode_or_handle_err<O, H>(
                &self,
                dest: &mut O,
                h: H,
            ) -> core::result::Result<(), H::HandledErr>
            where
                O: elrond_codec::NestedEncodeOutput,
                H: elrond_codec::EncodeErrorHandler,
            {
                match self {
                    ItemSlot::None => {
                        elrond_codec::NestedEncode::dep_encode_or_handle_err(&0u8, dest, h)?;
                    }
                    ItemSlot::Hat => {
                        elrond_codec::NestedEncode::dep_encode_or_handle_err(&1u8, dest, h)?;
                    }
                    ItemSlot::Background => {
                        elrond_codec::NestedEncode::dep_encode_or_handle_err(&2u8, dest, h)?;
                    }
                    ItemSlot::Skin => {
                        elrond_codec::NestedEncode::dep_encode_or_handle_err(&3u8, dest, h)?;
                    }
                    ItemSlot::Beak => {
                        elrond_codec::NestedEncode::dep_encode_or_handle_err(&4u8, dest, h)?;
                    }
                    ItemSlot::Weapon => {
                        elrond_codec::NestedEncode::dep_encode_or_handle_err(&5u8, dest, h)?;
                    }
                    ItemSlot::Clothes => {
                        elrond_codec::NestedEncode::dep_encode_or_handle_err(&6u8, dest, h)?;
                    }
                    ItemSlot::Eye => {
                        elrond_codec::NestedEncode::dep_encode_or_handle_err(&7u8, dest, h)?;
                    }
                };
                core::result::Result::Ok(())
            }
        }
        impl elrond_codec::NestedDecode for ItemSlot {
            fn dep_decode_or_handle_err<I, H>(
                input: &mut I,
                h: H,
            ) -> core::result::Result<Self, H::HandledErr>
            where
                I: elrond_codec::NestedDecodeInput,
                H: elrond_codec::DecodeErrorHandler,
            {
                match <u8 as elrond_codec::NestedDecode>::dep_decode_or_handle_err(input, h)? {
                    0u8 => core::result::Result::Ok(ItemSlot::None),
                    1u8 => core::result::Result::Ok(ItemSlot::Hat),
                    2u8 => core::result::Result::Ok(ItemSlot::Background),
                    3u8 => core::result::Result::Ok(ItemSlot::Skin),
                    4u8 => core::result::Result::Ok(ItemSlot::Beak),
                    5u8 => core::result::Result::Ok(ItemSlot::Weapon),
                    6u8 => core::result::Result::Ok(ItemSlot::Clothes),
                    7u8 => core::result::Result::Ok(ItemSlot::Eye),
                    _ => core::result::Result::Err(
                        h.handle_error(elrond_codec::DecodeError::INVALID_VALUE),
                    ),
                }
            }
        }
        impl elrond_codec::TopEncode for ItemSlot {
            fn top_encode_or_handle_err<O, H>(
                &self,
                output: O,
                h: H,
            ) -> core::result::Result<(), H::HandledErr>
            where
                O: elrond_codec::TopEncodeOutput,
                H: elrond_codec::EncodeErrorHandler,
            {
                match self {
                    ItemSlot::None => {
                        elrond_codec::TopEncode::top_encode_or_handle_err(&0u8, output, h)
                    }
                    ItemSlot::Hat => {
                        elrond_codec::TopEncode::top_encode_or_handle_err(&1u8, output, h)
                    }
                    ItemSlot::Background => {
                        elrond_codec::TopEncode::top_encode_or_handle_err(&2u8, output, h)
                    }
                    ItemSlot::Skin => {
                        elrond_codec::TopEncode::top_encode_or_handle_err(&3u8, output, h)
                    }
                    ItemSlot::Beak => {
                        elrond_codec::TopEncode::top_encode_or_handle_err(&4u8, output, h)
                    }
                    ItemSlot::Weapon => {
                        elrond_codec::TopEncode::top_encode_or_handle_err(&5u8, output, h)
                    }
                    ItemSlot::Clothes => {
                        elrond_codec::TopEncode::top_encode_or_handle_err(&6u8, output, h)
                    }
                    ItemSlot::Eye => {
                        elrond_codec::TopEncode::top_encode_or_handle_err(&7u8, output, h)
                    }
                }
            }
        }
        impl elrond_codec::TopDecode for ItemSlot {
            fn top_decode_or_handle_err<I, H>(
                top_input: I,
                h: H,
            ) -> core::result::Result<Self, H::HandledErr>
            where
                I: elrond_codec::TopDecodeInput,
                H: elrond_codec::DecodeErrorHandler,
            {
                if top_input.byte_len() == 0 {
                    return core::result::Result::Ok(ItemSlot::None);
                }
                match <u8 as elrond_codec::TopDecode>::top_decode_or_handle_err(top_input, h)? {
                    0u8 => core::result::Result::Ok(ItemSlot::None),
                    1u8 => core::result::Result::Ok(ItemSlot::Hat),
                    2u8 => core::result::Result::Ok(ItemSlot::Background),
                    3u8 => core::result::Result::Ok(ItemSlot::Skin),
                    4u8 => core::result::Result::Ok(ItemSlot::Beak),
                    5u8 => core::result::Result::Ok(ItemSlot::Weapon),
                    6u8 => core::result::Result::Ok(ItemSlot::Clothes),
                    7u8 => core::result::Result::Ok(ItemSlot::Eye),
                    _ => core::result::Result::Err(
                        h.handle_error(elrond_codec::DecodeError::INVALID_VALUE),
                    ),
                }
            }
        }
        impl elrond_wasm::abi::TypeAbi for ItemSlot {
            fn type_name() -> elrond_wasm::String {
                "ItemSlot".into()
            }
            fn provide_type_descriptions<TDC: elrond_wasm::abi::TypeDescriptionContainer>(
                accumulator: &mut TDC,
            ) {
                let type_name = Self::type_name();
                if !accumulator.contains_type(&type_name) {
                    accumulator.reserve_type_name(type_name.clone());
                    let mut variant_descriptions = elrond_wasm::Vec::new();
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
                        docs: &[],
                        discriminant: 0usize,
                        name: "None",
                        fields: field_descriptions,
                    });
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
                        docs: &[],
                        discriminant: 1usize,
                        name: "Hat",
                        fields: field_descriptions,
                    });
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
                        docs: &[],
                        discriminant: 2usize,
                        name: "Background",
                        fields: field_descriptions,
                    });
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
                        docs: &[],
                        discriminant: 3usize,
                        name: "Skin",
                        fields: field_descriptions,
                    });
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
                        docs: &[],
                        discriminant: 4usize,
                        name: "Beak",
                        fields: field_descriptions,
                    });
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
                        docs: &[],
                        discriminant: 5usize,
                        name: "Weapon",
                        fields: field_descriptions,
                    });
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
                        docs: &[],
                        discriminant: 6usize,
                        name: "Clothes",
                        fields: field_descriptions,
                    });
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    variant_descriptions.push(elrond_wasm::abi::EnumVariantDescription {
                        docs: &[],
                        discriminant: 7usize,
                        name: "Eye",
                        fields: field_descriptions,
                    });
                    accumulator.insert(
                        type_name.clone(),
                        elrond_wasm::abi::TypeDescription {
                            docs: &[],
                            name: type_name,
                            contents: elrond_wasm::abi::TypeContents::Enum(variant_descriptions),
                        },
                    );
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for ItemSlot {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match (&*self,) {
                    (&ItemSlot::None,) => ::core::fmt::Formatter::write_str(f, "None"),
                    (&ItemSlot::Hat,) => ::core::fmt::Formatter::write_str(f, "Hat"),
                    (&ItemSlot::Background,) => ::core::fmt::Formatter::write_str(f, "Background"),
                    (&ItemSlot::Skin,) => ::core::fmt::Formatter::write_str(f, "Skin"),
                    (&ItemSlot::Beak,) => ::core::fmt::Formatter::write_str(f, "Beak"),
                    (&ItemSlot::Weapon,) => ::core::fmt::Formatter::write_str(f, "Weapon"),
                    (&ItemSlot::Clothes,) => ::core::fmt::Formatter::write_str(f, "Clothes"),
                    (&ItemSlot::Eye,) => ::core::fmt::Formatter::write_str(f, "Eye"),
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for ItemSlot {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for ItemSlot {
            #[inline]
            fn eq(&self, other: &ItemSlot) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for ItemSlot {
            #[inline]
            fn clone(&self) -> ItemSlot {
                match (&*self,) {
                    (&ItemSlot::None,) => ItemSlot::None,
                    (&ItemSlot::Hat,) => ItemSlot::Hat,
                    (&ItemSlot::Background,) => ItemSlot::Background,
                    (&ItemSlot::Skin,) => ItemSlot::Skin,
                    (&ItemSlot::Beak,) => ItemSlot::Beak,
                    (&ItemSlot::Weapon,) => ItemSlot::Weapon,
                    (&ItemSlot::Clothes,) => ItemSlot::Clothes,
                    (&ItemSlot::Eye,) => ItemSlot::Eye,
                }
            }
        }
        impl ItemSlot {
            pub const VALUES: [ItemSlot; 7] = [
                Self::Hat,
                Self::Background,
                Self::Skin,
                Self::Beak,
                Self::Weapon,
                Self::Clothes,
                Self::Eye,
            ];
            pub fn to_bytes<M: ManagedTypeApi>(&self) -> &[u8] {
                match self {
                    Self::Hat => return b"hat",
                    Self::Background => return b"background",
                    Self::Skin => return b"skin",
                    Self::Beak => return b"beak",
                    Self::Weapon => return b"weapon",
                    Self::Clothes => return b"clothes",
                    Self::Eye => return b"eyes",
                    Self::None => return b"none",
                }
            }
            /// To_bytes but with caps to the first character
            pub fn to_title_bytes<M: ManagedTypeApi>(&self) -> &[u8] {
                match self {
                    Self::Hat => return b"Hat",
                    Self::Background => return b"Background",
                    Self::Skin => return b"Skin",
                    Self::Beak => return b"Beak",
                    Self::Weapon => return b"Weapon",
                    Self::Clothes => return b"Clothes",
                    Self::Eye => return b"Eyes",
                    Self::None => return b"None",
                }
            }
        }
    }
    pub mod penguin_attributes {
        #![no_std]
        #![no_main]
        #![allow(unused_attributes)]
        #![allow(unused_imports)]
        use super::{item::Item, item_slot::ItemSlot};
        use alloc::{borrow::ToOwned, format};
        use core::ops::{
            Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
            DivAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
            SubAssign,
        };
        use elrond_wasm::{
            api::{
                BigIntApi, BlockchainApi, BlockchainApiImpl, CallValueApi, CallValueApiImpl,
                CryptoApi, CryptoApiImpl, EllipticCurveApi, ErrorApi, ErrorApiImpl, LogApi,
                LogApiImpl, ManagedTypeApi, PrintApi, PrintApiImpl, SendApi, SendApiImpl,
            },
            arrayvec::ArrayVec,
            contract_base::{ContractBase, ProxyObjBase},
            elrond_codec::{multi_types::*, DecodeError, NestedDecode, NestedEncode, TopDecode},
            err_msg,
            esdt::*,
            io::*,
            non_zero_usize,
            non_zero_util::*,
            require, require_old, sc_error, sc_panic, sc_print,
            storage::mappers::*,
            types::{
                SCResult::{Err, Ok},
                *,
            },
            Box, Vec,
        };
        use elrond_wasm::{
            derive::{ManagedVecItem, TypeAbi},
            elrond_codec,
            elrond_codec::elrond_codec_derive::{
                NestedDecode, NestedEncode, TopDecode, TopDecodeOrDefault, TopEncode,
                TopEncodeOrDefault,
            },
        };
        use elrond_wasm::{
            elrond_codec::{TopDecodeInput, TopEncode},
            String,
        };
        pub struct PenguinAttributes<M: ManagedTypeApi> {
            pub hat: Option<Item<M>>,
            pub background: Option<Item<M>>,
            pub skin: Option<Item<M>>,
            pub beak: Option<Item<M>>,
            pub weapon: Option<Item<M>>,
            pub clothes: Option<Item<M>>,
            pub eye: Option<Item<M>>,
        }
        impl<M: ManagedTypeApi> elrond_codec::TopDecode for PenguinAttributes<M> {
            fn top_decode_or_handle_err<I, H>(
                top_input: I,
                h: H,
            ) -> core::result::Result<Self, H::HandledErr>
            where
                I: elrond_codec::TopDecodeInput,
                H: elrond_codec::DecodeErrorHandler,
            {
                let mut nested_buffer = top_input.into_nested_buffer();
                let result = PenguinAttributes {
                    hat: <Option<Item<M>> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        &mut nested_buffer,
                        h,
                    )?,
                    background:
                        <Option<Item<M>> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                    skin:
                        <Option<Item<M>> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                    beak:
                        <Option<Item<M>> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                    weapon:
                        <Option<Item<M>> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                    clothes:
                        <Option<Item<M>> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                            &mut nested_buffer,
                            h,
                        )?,
                    eye: <Option<Item<M>> as elrond_codec::NestedDecode>::dep_decode_or_handle_err(
                        &mut nested_buffer,
                        h,
                    )?,
                };
                if !elrond_codec::NestedDecodeInput::is_depleted(&nested_buffer) {
                    return core::result::Result::Err(
                        h.handle_error(elrond_codec::DecodeError::INPUT_TOO_LONG),
                    );
                }
                core::result::Result::Ok(result)
            }
        }
        impl<M: ManagedTypeApi> elrond_codec::TopEncode for PenguinAttributes<M> {
            fn top_encode_or_handle_err<O, H>(
                &self,
                output: O,
                h: H,
            ) -> core::result::Result<(), H::HandledErr>
            where
                O: elrond_codec::TopEncodeOutput,
                H: elrond_codec::EncodeErrorHandler,
            {
                let mut buffer = output.start_nested_encode();
                let dest = &mut buffer;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.hat, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.background, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.skin, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.beak, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.weapon, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.clothes, dest, h)?;
                elrond_codec::NestedEncode::dep_encode_or_handle_err(&self.eye, dest, h)?;
                output.finalize_nested_encode(buffer);
                core::result::Result::Ok(())
            }
        }
        impl<M: ManagedTypeApi> ::core::marker::StructuralPartialEq for PenguinAttributes<M> {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<M: ::core::cmp::PartialEq + ManagedTypeApi> ::core::cmp::PartialEq for PenguinAttributes<M> {
            #[inline]
            fn eq(&self, other: &PenguinAttributes<M>) -> bool {
                match *other {
                    PenguinAttributes {
                        hat: ref __self_1_0,
                        background: ref __self_1_1,
                        skin: ref __self_1_2,
                        beak: ref __self_1_3,
                        weapon: ref __self_1_4,
                        clothes: ref __self_1_5,
                        eye: ref __self_1_6,
                    } => match *self {
                        PenguinAttributes {
                            hat: ref __self_0_0,
                            background: ref __self_0_1,
                            skin: ref __self_0_2,
                            beak: ref __self_0_3,
                            weapon: ref __self_0_4,
                            clothes: ref __self_0_5,
                            eye: ref __self_0_6,
                        } => {
                            (*__self_0_0) == (*__self_1_0)
                                && (*__self_0_1) == (*__self_1_1)
                                && (*__self_0_2) == (*__self_1_2)
                                && (*__self_0_3) == (*__self_1_3)
                                && (*__self_0_4) == (*__self_1_4)
                                && (*__self_0_5) == (*__self_1_5)
                                && (*__self_0_6) == (*__self_1_6)
                        }
                    },
                }
            }
            #[inline]
            fn ne(&self, other: &PenguinAttributes<M>) -> bool {
                match *other {
                    PenguinAttributes {
                        hat: ref __self_1_0,
                        background: ref __self_1_1,
                        skin: ref __self_1_2,
                        beak: ref __self_1_3,
                        weapon: ref __self_1_4,
                        clothes: ref __self_1_5,
                        eye: ref __self_1_6,
                    } => match *self {
                        PenguinAttributes {
                            hat: ref __self_0_0,
                            background: ref __self_0_1,
                            skin: ref __self_0_2,
                            beak: ref __self_0_3,
                            weapon: ref __self_0_4,
                            clothes: ref __self_0_5,
                            eye: ref __self_0_6,
                        } => {
                            (*__self_0_0) != (*__self_1_0)
                                || (*__self_0_1) != (*__self_1_1)
                                || (*__self_0_2) != (*__self_1_2)
                                || (*__self_0_3) != (*__self_1_3)
                                || (*__self_0_4) != (*__self_1_4)
                                || (*__self_0_5) != (*__self_1_5)
                                || (*__self_0_6) != (*__self_1_6)
                        }
                    },
                }
            }
        }
        impl<M: ManagedTypeApi> elrond_wasm::abi::TypeAbi for PenguinAttributes<M> {
            fn type_name() -> elrond_wasm::String {
                "PenguinAttributes".into()
            }
            fn provide_type_descriptions<TDC: elrond_wasm::abi::TypeDescriptionContainer>(
                accumulator: &mut TDC,
            ) {
                let type_name = Self::type_name();
                if !accumulator.contains_type(&type_name) {
                    accumulator.reserve_type_name(type_name.clone());
                    let mut field_descriptions = elrond_wasm::Vec::new();
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "hat",
                        field_type: <Option<Item<M>>>::type_name(),
                    });
                    <Option<Item<M>>>::provide_type_descriptions(accumulator);
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "background",
                        field_type: <Option<Item<M>>>::type_name(),
                    });
                    <Option<Item<M>>>::provide_type_descriptions(accumulator);
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "skin",
                        field_type: <Option<Item<M>>>::type_name(),
                    });
                    <Option<Item<M>>>::provide_type_descriptions(accumulator);
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "beak",
                        field_type: <Option<Item<M>>>::type_name(),
                    });
                    <Option<Item<M>>>::provide_type_descriptions(accumulator);
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "weapon",
                        field_type: <Option<Item<M>>>::type_name(),
                    });
                    <Option<Item<M>>>::provide_type_descriptions(accumulator);
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "clothes",
                        field_type: <Option<Item<M>>>::type_name(),
                    });
                    <Option<Item<M>>>::provide_type_descriptions(accumulator);
                    field_descriptions.push(elrond_wasm::abi::StructFieldDescription {
                        docs: &[],
                        name: "eye",
                        field_type: <Option<Item<M>>>::type_name(),
                    });
                    <Option<Item<M>>>::provide_type_descriptions(accumulator);
                    accumulator.insert(
                        type_name.clone(),
                        elrond_wasm::abi::TypeDescription {
                            docs: &[],
                            name: type_name,
                            contents: elrond_wasm::abi::TypeContents::Struct(field_descriptions),
                        },
                    );
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl<M: ::core::fmt::Debug + ManagedTypeApi> ::core::fmt::Debug for PenguinAttributes<M> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    PenguinAttributes {
                        hat: ref __self_0_0,
                        background: ref __self_0_1,
                        skin: ref __self_0_2,
                        beak: ref __self_0_3,
                        weapon: ref __self_0_4,
                        clothes: ref __self_0_5,
                        eye: ref __self_0_6,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "PenguinAttributes");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "hat",
                            &&(*__self_0_0),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "background",
                            &&(*__self_0_1),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "skin",
                            &&(*__self_0_2),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "beak",
                            &&(*__self_0_3),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "weapon",
                            &&(*__self_0_4),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "clothes",
                            &&(*__self_0_5),
                        );
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "eye",
                            &&(*__self_0_6),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl<M: ManagedTypeApi> PenguinAttributes<M> {
            pub fn new(args: &[(&ItemSlot, Item<M>)]) -> Self {
                let mut attributes = Self::empty();
                for (slot, item) in args {
                    let result = attributes.set_item(slot, Option::Some(item.clone()));
                    if result.is_err() {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &["Failed to set item on slot"],
                            &[],
                        ));
                    }
                }
                return attributes;
            }
            pub fn set_item(
                &mut self,
                slot: &ItemSlot,
                item: Option<Item<M>>,
            ) -> Result<(), ManagedBuffer<M>> {
                if self.is_slot_empty(slot) == false {
                    return Result::Err(ManagedBuffer::new_from_bytes(
                        b"The slot is not empty. Please free it, before setting an item.",
                    ));
                }
                return self.__set_item_no_check(slot, item);
            }
            pub fn get_fill_count(&self) -> usize {
                let mut size: usize = 0;
                for slot in ItemSlot::VALUES.iter() {
                    if self.is_slot_empty(slot) == false {
                        size += 1;
                    }
                }
                return size;
            }
            #[allow(unreachable_patterns)]
            pub fn get_item(&self, slot: &ItemSlot) -> Option<Item<M>> {
                match slot {
                    &ItemSlot::Hat => return self.hat.clone(),
                    &ItemSlot::Background => return self.background.clone(),
                    &ItemSlot::Skin => return self.skin.clone(),
                    &ItemSlot::Beak => return self.beak.clone(),
                    &ItemSlot::Weapon => return self.weapon.clone(),
                    &ItemSlot::Clothes => return self.clothes.clone(),
                    &ItemSlot::Eye => return self.eye.clone(),
                    _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["Missing slot. Please add it in get_item"],
                        &[],
                    )),
                };
            }
            #[allow(unreachable_patterns)]
            pub fn is_slot_empty(&self, slot: &ItemSlot) -> bool {
                let item = self.get_item(slot);
                match item {
                    Some(_) => false,
                    None => true,
                }
            }
            pub fn set_empty_slot(&mut self, slot: &ItemSlot) -> Result<(), ManagedBuffer<M>> {
                return self.__set_item_no_check(slot, Option::None);
            }
            pub fn empty() -> Self {
                Self {
                    hat: Option::None,
                    background: Option::None,
                    skin: Option::None,
                    beak: Option::None,
                    weapon: Option::None,
                    clothes: Option::None,
                    eye: Option::None,
                }
            }
            /// Set an item on a slot, without checking if the slot is empty.
            fn __set_item_no_check(
                &mut self,
                slot: &ItemSlot,
                item: Option<Item<M>>,
            ) -> Result<(), ManagedBuffer<M>> {
                #[allow(unreachable_patterns)]
                match slot {
                    ItemSlot::Hat => self.hat = item,
                    ItemSlot::Background => self.background = item,
                    ItemSlot::Skin => self.skin = item,
                    ItemSlot::Beak => self.beak = item,
                    ItemSlot::Weapon => self.weapon = item,
                    ItemSlot::Clothes => self.clothes = item,
                    ItemSlot::Eye => self.eye = item,
                    _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                        &["Missing slot. Please add it in get_item"],
                        &[],
                    )),
                }
                Result::Ok(())
            }
            fn to_managed_buffer(&self, slot: &ItemSlot) -> ManagedBuffer<M> {
                let item = match self.get_item(slot) {
                    Some(item) => {
                        let mut output = ManagedBuffer::new();
                        item.top_encode(&mut output).unwrap();
                        output
                    }
                    None => ManagedBuffer::<M>::new_from_bytes(b"unequipped"),
                };
                let mut managed_buffer = ManagedBuffer::<M>::new();
                managed_buffer.append_bytes(slot.to_title_bytes::<M>());
                managed_buffer.append_bytes(b":");
                managed_buffer.append(&item);
                return managed_buffer;
            }
        }
    }
}
use alloc::string::ToString;
use elrond_wasm::{elrond_codec::TopEncode, String};
use libs::*;
use structs::{
    item::Item, item_attributes::ItemAttributes, item_slot::*,
    penguin_attributes::PenguinAttributes,
};
pub trait Equip:
    elrond_wasm::contract_base::ContractBase
    + Sized
    + penguin_mint::MintPenguin
    + penguin_parse::ParsePenguin
    + storage::StorageModule
{
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn init(
        &self,
        penguins_identifier: elrond_wasm::types::TokenIdentifier<Self::Api>,
    ) -> SCResult<()> {
        self.penguins_identifier().set(&penguins_identifier);
        self.uri().set(
            elrond_wasm::types::ManagedBuffer::<Self::Api>::new_from_bytes(
                b"https://penguins-generator.herokuapp.com/",
            ),
        );
        return Ok(());
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn register_item(
        &self,
        item_slot: ItemSlot,
        items_id_to_add: MultiValueEncoded<
            Self::Api,
            elrond_wasm::types::TokenIdentifier<Self::Api>,
        >,
    ) -> SCResult<()> {
        if (!(self.blockchain().get_caller() == self.blockchain().get_owner_address())) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Only the owner can call this method.",
            );
        };
        for item_id in items_id_to_add {
            if (!(item_id != self.penguins_identifier().get())) {
                elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                    "You cannot register a penguin as an item.",
                );
            };
            self.items_slot(&item_id.into()).set(&item_slot);
        }
        return Ok(());
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn get_item_slot(
        &self,
        item_id: &elrond_wasm::types::TokenIdentifier<Self::Api>,
    ) -> OptionalValue<ItemSlot> {
        match self.items_slot(item_id).get() {
            ItemSlot::None => return OptionalValue::None,
            slot => return OptionalValue::Some(slot),
        }
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn customize(
        &self,
        payments: ManagedVec<Self::Api, EsdtTokenPayment<Self::Api>>,
        to_desequip_slots: MultiValueEncoded<Self::Api, ItemSlot>,
    ) -> SCResult<u64> {
        self.require_penguin_roles_set()?;
        if (!(payments.len() >= 1)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "You must provide at least one penguin.",
            );
        };
        if (!(payments.len() >= 2 || to_desequip_slots.len() >= 1)) {
            elrond_wasm :: contract_base :: ErrorHelper :: < Self :: Api > :: signal_error_with_message ("You must either provide at least one penguin and one item OR provide a slot to desequip.") ;
        };
        let first_payment = payments.get(0);
        let penguin_id = first_payment.token_identifier;
        let penguin_nonce = first_payment.token_nonce;
        if (!(&penguin_id == &self.penguins_identifier().get())) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Please provide a penguin as the first payment",
            );
        };
        if (!(first_payment.amount == 1)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "You must sent only one penguin.",
            );
        };
        let mut attributes = self.parse_penguin_attributes(&penguin_id, penguin_nonce);
        for slot in to_desequip_slots {
            self.desequip_slot(&mut attributes, &slot)?;
        }
        let items_token = payments.iter().skip(1);
        for payment in items_token {
            if (!(payment.amount == 1)) {
                elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                    "You must sent only one item.",
                );
            };
            let item = Item {
                token: payment.token_identifier.clone(),
                nonce: payment.token_nonce,
                name: self.get_token_name(&payment.token_identifier, payment.token_nonce)?,
            };
            self.equip_slot(&mut attributes, &item)?;
        }
        return self.update_penguin(&penguin_id, penguin_nonce, &attributes);
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn get_token_name(
        &self,
        item_id: &elrond_wasm::types::TokenIdentifier<Self::Api>,
        nonce: u64,
    ) -> SCResult<elrond_wasm::types::ManagedBuffer<Self::Api>> {
        {
            let mut ___buffer___ =
                elrond_wasm::types::ManagedBufferCachedBuilder::<Self::Api>::new_from_slice(&[]);
            elrond_wasm::formatter::FormatReceiver::push_static_ascii(&mut ___buffer___, b"token ");
            elrond_wasm::formatter::FormatReceiver::push_top_encode_bytes(
                &mut ___buffer___,
                &item_id,
            );
            elrond_wasm::formatter::FormatReceiver::push_static_ascii(
                &mut ___buffer___,
                b" nonce ",
            );
            elrond_wasm::formatter::FormatReceiver::push_top_encode_hex(&mut ___buffer___, &nonce);
            <Self::Api as elrond_wasm::api::PrintApi>::print_api_impl()
                .print_managed_buffer(___buffer___.into_managed_buffer().get_raw_handle());
        };
        let item_name = self
            .blockchain()
            .get_esdt_token_data(&self.blockchain().get_sc_address(), item_id, nonce)
            .name;
        return Ok(item_name);
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn equip_slot(
        &self,
        attributes: &mut PenguinAttributes<Self::Api>,
        item: &Item<Self::Api>,
    ) -> SCResult<()> {
        let item_id = &item.token;
        let item_slot = self.items_slot(&item_id).get();
        if (!(item_slot != ItemSlot::None)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "You are trying to equip a token that is not considered as an item",
            );
        };
        if (!(item_id != &self.penguins_identifier().get())) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Cannot equip a penguin as an item.",
            );
        };
        if attributes.is_slot_empty(&item_slot) == false {
            self.desequip_slot(attributes, &item_slot)?;
        }
        let result = attributes.set_item(&item_slot, Option::Some(item.clone()));
        if (!(result == Result::Ok(()))) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Cannot set item. Maybe the item is not considered like an item.",
            );
        };
        return SCResult::Ok(());
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn require_penguin_roles_set(&self) -> SCResult<()> {
        let penguin_id = self.penguins_identifier().get();
        let roles = self.blockchain().get_esdt_local_roles(&penguin_id);
        if (!(roles.has_role(&EsdtLocalRole::NftCreate) == true)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Local create role not set for penguin",
            );
        };
        if (!(roles.has_role(&EsdtLocalRole::NftBurn) == true)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Local burn role not set  for penguin",
            );
        };
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn fill(
        &self,
        _token: TokenIdentifier<Self::Api>,
        _nonce: u64,
        _amount: elrond_wasm::types::BigUint<Self::Api>,
    ) -> SCResult<()> {
        if (!(self.blockchain().get_caller() == self.blockchain().get_owner_address())) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Only the owner can call this method.",
            );
        };
        return Ok(());
    }
    /// Empty the item at the slot provided and sent it to the caller.
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn desequip_slot(
        &self,
        attributes: &mut PenguinAttributes<Self::Api>,
        slot: &ItemSlot,
    ) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        if (!(slot != &ItemSlot::None)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Slot value must be different to ItemSlot::None.",
            );
        };
        if (!(attributes.is_slot_empty(&slot) == false)) {
            elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                "Cannot sent item from an empty slot",
            );
        };
        let opt_item = attributes.get_item(&slot);
        match opt_item {
            Some(item) => {
                let item_id = item.token;
                let item_nonce = item.nonce;
                if (!(self.get_item_slot(&item_id).into_option().is_some())) {
                    elrond_wasm :: contract_base :: ErrorHelper :: < Self :: Api > :: signal_error_with_message ("A item to desequip is not considered like an item. The item has maybe been removed. Please contact an administrator.") ;
                };
                if self.blockchain().get_sc_balance(&item_id, item_nonce) <= 1 {
                    {
                        let mut ___buffer___ = elrond_wasm::types::ManagedBufferCachedBuilder::<
                            Self::Api,
                        >::new_from_slice(&[]);
                        elrond_wasm::formatter::FormatReceiver::push_static_ascii(
                            &mut ___buffer___,
                            b"No token ",
                        );
                        elrond_wasm::formatter::FormatReceiver::push_top_encode_bytes(
                            &mut ___buffer___,
                            &item_id,
                        );
                        elrond_wasm::formatter::FormatReceiver::push_static_ascii(
                            &mut ___buffer___,
                            b" with nonce ",
                        );
                        elrond_wasm::formatter::FormatReceiver::push_top_encode_hex(
                            &mut ___buffer___,
                            &item_nonce,
                        );
                        elrond_wasm::formatter::FormatReceiver::push_static_ascii(
                            &mut ___buffer___,
                            b" on the SC. Please contact an administrator.",
                        );
                        elrond_wasm :: contract_base :: ErrorHelper :: < Self :: Api > :: signal_error_with_message (___buffer___ . into_managed_buffer ()) ;
                    };
                }
                self.send().direct(
                    &caller,
                    &item_id,
                    item_nonce,
                    &elrond_wasm::types::BigUint::<Self::Api>::from(1u32),
                    &[],
                );
                let result = attributes.set_empty_slot(&slot);
                if (!(result.is_err() == false)) {
                    elrond_wasm::contract_base::ErrorHelper::<Self::Api>::signal_error_with_message(
                        "Error while emptying slot",
                    );
                };
                return SCResult::Ok(());
            }
            None => {
                return SCResult::Err("Slot is empty, we can't sent item to it".into());
            }
        }
    }
}
pub trait AutoImpl: elrond_wasm::contract_base::ContractBase {}
impl<C> Equip for C where
    C: AutoImpl + penguin_mint::MintPenguin + penguin_parse::ParsePenguin + storage::StorageModule
{
}
pub trait EndpointWrappers:
    elrond_wasm::contract_base::ContractBase
    + Equip
    + penguin_mint::EndpointWrappers
    + penguin_parse::EndpointWrappers
    + storage::EndpointWrappers
{
    #[inline]
    fn call_init(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            1i32,
        );
        let penguins_identifier = elrond_wasm::load_single_arg::<
            Self::Api,
            elrond_wasm::types::TokenIdentifier<Self::Api>,
        >(0i32, ArgId::from(&b"penguins_identifier"[..]));
        let result = self.init(penguins_identifier);
        elrond_wasm::io::finish_multi::<Self::Api, _>(&result);
    }
    #[inline]
    fn call_register_item(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        self.blockchain().check_caller_is_owner();
        let mut ___arg_loader = elrond_wasm::io::EndpointDynArgLoader::<Self::Api>::new();
        let item_slot: ItemSlot = elrond_wasm::load_dyn_arg::<Self::Api, _, _>(
            &mut ___arg_loader,
            ArgId::from(&b"item_slot"[..]),
        );
        let items_id_to_add: MultiValueEncoded<
            Self::Api,
            elrond_wasm::types::TokenIdentifier<Self::Api>,
        > = elrond_wasm::load_dyn_arg::<Self::Api, _, _>(
            &mut ___arg_loader,
            ArgId::from(&b"items_id_to_add"[..]),
        );
        elrond_wasm::io::assert_no_more_args::<Self::Api, _>(&___arg_loader);
        let result = self.register_item(item_slot, items_id_to_add);
        elrond_wasm::io::finish_multi::<Self::Api, _>(&result);
    }
    #[inline]
    fn call_get_item_slot(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        elrond_wasm::api::CallValueApiImpl::check_not_payable(&Self::Api::call_value_api_impl());
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            1i32,
        );
        let item_id = elrond_wasm::load_single_arg::<
            Self::Api,
            elrond_wasm::types::TokenIdentifier<Self::Api>,
        >(0i32, ArgId::from(&b"item_id"[..]));
        let result = self.get_item_slot(&item_id);
        elrond_wasm::io::finish_multi::<Self::Api, _>(&result);
    }
    #[inline]
    fn call_customize(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        let payments = self.call_value().all_esdt_transfers();
        let mut ___arg_loader = elrond_wasm::io::EndpointDynArgLoader::<Self::Api>::new();
        let to_desequip_slots: MultiValueEncoded<Self::Api, ItemSlot> =
            elrond_wasm::load_dyn_arg::<Self::Api, _, _>(
                &mut ___arg_loader,
                ArgId::from(&b"to_desequip_slots"[..]),
            );
        elrond_wasm::io::assert_no_more_args::<Self::Api, _>(&___arg_loader);
        let result = self.customize(payments, to_desequip_slots);
        elrond_wasm::io::finish_multi::<Self::Api, _>(&result);
    }
    #[inline]
    fn call_fill(&self) {
        <Self::Api as elrond_wasm::api::VMApi>::init_static();
        let (_amount, _token) =
            elrond_wasm::contract_base::CallValueWrapper::<Self::Api>::new().payment_token_pair();
        let _nonce = self.call_value().esdt_token_nonce();
        self.blockchain().check_caller_is_owner();
        elrond_wasm::api::EndpointArgumentApiImpl::check_num_arguments(
            &<Self::Api as elrond_wasm::api::EndpointArgumentApi>::argument_api_impl(),
            0i32,
        );
        let result = self.fill(_token, _nonce, _amount);
        elrond_wasm::io::finish_multi::<Self::Api, _>(&result);
    }
    fn call(&self, fn_name: &[u8]) -> bool {
        if match fn_name {
            b"callBack"
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self::EndpointWrappers::callback(self);
                return true;
            }
            b"init"
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::ViewContract,
                ) =>
            {
                elrond_wasm::external_view_contract::external_view_contract_constructor::<Self::Api>(
                );
                return true;
            }
            [105u8, 110u8, 105u8, 116u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_init();
                true
            }
            [114u8, 101u8, 103u8, 105u8, 115u8, 116u8, 101u8, 114u8, 73u8, 116u8, 101u8, 109u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_register_item();
                true
            }
            [103u8, 101u8, 116u8, 73u8, 116u8, 101u8, 109u8, 84u8, 121u8, 112u8, 101u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_get_item_slot();
                true
            }
            [99u8, 117u8, 115u8, 116u8, 111u8, 109u8, 105u8, 122u8, 101u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_customize();
                true
            }
            [102u8, 105u8, 108u8, 108u8]
                if <Self::Api as elrond_wasm::api::VMApi>::has_location(
                    elrond_wasm::abi::EndpointLocationAbi::MainContract,
                ) =>
            {
                self.call_fill();
                true
            }
            other => false,
        } {
            return true;
        }
        if penguin_mint::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if penguin_parse::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        if storage::EndpointWrappers::call(self, fn_name) {
            return true;
        }
        false
    }
    fn callback_selector(
        &self,
        mut ___cb_closure___: elrond_wasm::types::CallbackClosureForDeser<Self::Api>,
    ) -> elrond_wasm::types::CallbackSelectorResult<Self::Api> {
        let mut ___call_result_loader___ =
            elrond_wasm::io::EndpointDynArgLoader::<Self::Api>::new();
        let ___cb_closure_matcher___ = ___cb_closure___.matcher::<32usize>();
        if ___cb_closure_matcher___.matches_empty() {
            return elrond_wasm::types::CallbackSelectorResult::Processed;
        }
        match penguin_mint::EndpointWrappers::callback_selector(self, ___cb_closure___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_cb_closure) => {
                ___cb_closure___ = recovered_cb_closure;
            }
        }
        match penguin_parse::EndpointWrappers::callback_selector(self, ___cb_closure___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_cb_closure) => {
                ___cb_closure___ = recovered_cb_closure;
            }
        }
        match storage::EndpointWrappers::callback_selector(self, ___cb_closure___) {
            elrond_wasm::types::CallbackSelectorResult::Processed => {
                return elrond_wasm::types::CallbackSelectorResult::Processed;
            }
            elrond_wasm::types::CallbackSelectorResult::NotProcessed(recovered_cb_closure) => {
                ___cb_closure___ = recovered_cb_closure;
            }
        }
        elrond_wasm::types::CallbackSelectorResult::NotProcessed(___cb_closure___)
    }
    fn callback(&self) {
        if let Some(___cb_closure___) =
            elrond_wasm::types::CallbackClosureForDeser::storage_load_and_clear::<Self::Api>()
        {
            if let elrond_wasm::types::CallbackSelectorResult::NotProcessed(_) =
                self::EndpointWrappers::callback_selector(self, ___cb_closure___)
            {
                elrond_wasm::api::ErrorApiImpl::signal_error(
                    &Self::Api::error_api_impl(),
                    err_msg::CALLBACK_BAD_FUNC,
                );
            }
        }
    }
}
pub struct AbiProvider {}
impl elrond_wasm::contract_base::ContractAbiProvider for AbiProvider {
    type Api = elrond_wasm::api::uncallable::UncallableApi;
    fn abi() -> elrond_wasm::abi::ContractAbi {
        let mut contract_abi = elrond_wasm :: abi :: ContractAbi { build_info : elrond_wasm :: abi :: BuildInfoAbi { contract_crate : elrond_wasm :: abi :: ContractCrateBuildAbi { name : "equip_penguin" , version : "0.0.0" , git_version : "N/A" , } , framework : elrond_wasm :: abi :: FrameworkBuildAbi :: create () , } , docs : & [] , name : "Equip" , constructors : Vec :: new () , endpoints : Vec :: new () , has_callback : false , type_descriptions : < elrond_wasm :: abi :: TypeDescriptionContainerImpl as elrond_wasm :: abi :: TypeDescriptionContainer > :: new () , } ;
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "init",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi
            .add_input::<elrond_wasm::types::TokenIdentifier<Self::Api>>("penguins_identifier");
        contract_abi.add_type_descriptions::<elrond_wasm::types::TokenIdentifier<Self::Api>>();
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.constructors.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "registerItem",
            only_owner: true,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<ItemSlot>("item_slot");
        contract_abi.add_type_descriptions::<ItemSlot>();
        endpoint_abi . add_input :: < MultiValueEncoded < Self :: Api , elrond_wasm :: types :: TokenIdentifier < Self :: Api > > > ("items_id_to_add") ;
        contract_abi . add_type_descriptions :: < MultiValueEncoded < Self :: Api , elrond_wasm :: types :: TokenIdentifier < Self :: Api > > > () ;
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "getItemType",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Readonly,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &[],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<&elrond_wasm::types::TokenIdentifier<Self::Api>>("item_id");
        contract_abi.add_type_descriptions::<&elrond_wasm::types::TokenIdentifier<Self::Api>>();
        endpoint_abi.add_output::<OptionalValue<ItemSlot>>(&[]);
        contract_abi.add_type_descriptions::<OptionalValue<ItemSlot>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "customize",
            only_owner: false,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &["*"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_input::<MultiValueEncoded<Self::Api, ItemSlot>>("to_desequip_slots");
        contract_abi.add_type_descriptions::<MultiValueEncoded<Self::Api, ItemSlot>>();
        endpoint_abi.add_output::<SCResult<u64>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<u64>>();
        contract_abi.endpoints.push(endpoint_abi);
        let mut endpoint_abi = elrond_wasm::abi::EndpointAbi {
            docs: &[],
            name: "fill",
            only_owner: true,
            mutability: elrond_wasm::abi::EndpointMutabilityAbi::Mutable,
            location: elrond_wasm::abi::EndpointLocationAbi::MainContract,
            payable_in_tokens: &["*"],
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        endpoint_abi.add_output::<SCResult<()>>(&[]);
        contract_abi.add_type_descriptions::<SCResult<()>>();
        contract_abi.endpoints.push(endpoint_abi);
        contract_abi.coalesce(
            <penguin_mint::AbiProvider as elrond_wasm::contract_base::ContractAbiProvider>::abi(),
        );
        contract_abi.coalesce(
            <penguin_parse::AbiProvider as elrond_wasm::contract_base::ContractAbiProvider>::abi(),
        );
        contract_abi.coalesce(
            <storage::AbiProvider as elrond_wasm::contract_base::ContractAbiProvider>::abi(),
        );
        contract_abi
    }
}
pub struct ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    _phantom: core::marker::PhantomData<A>,
}
impl<A> elrond_wasm::contract_base::ContractBase for ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    type Api = A;
}
impl<A> penguin_mint::AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> penguin_parse::AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> storage::AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> AutoImpl for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> penguin_mint::EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> penguin_parse::EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> storage::EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> EndpointWrappers for ContractObj<A> where A: elrond_wasm::api::VMApi {}
impl<A> elrond_wasm::contract_base::CallableContract for ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    fn call(&self, fn_name: &[u8]) -> bool {
        EndpointWrappers::call(self, fn_name)
    }
    fn clone_obj(&self) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
        elrond_wasm::Box::new(ContractObj::<A> {
            _phantom: core::marker::PhantomData,
        })
    }
}
pub fn contract_obj<A>() -> ContractObj<A>
where
    A: elrond_wasm::api::VMApi,
{
    ContractObj {
        _phantom: core::marker::PhantomData,
    }
}
pub struct ContractBuilder;
impl elrond_wasm::contract_base::CallableContractBuilder for self::ContractBuilder {
    fn new_contract_obj<A: elrond_wasm::api::VMApi>(
        &self,
    ) -> elrond_wasm::Box<dyn elrond_wasm::contract_base::CallableContract> {
        elrond_wasm::Box::new(ContractObj::<A> {
            _phantom: core::marker::PhantomData,
        })
    }
}
pub use penguin_mint::endpoints as __endpoints_0__;
pub use penguin_parse::endpoints as __endpoints_1__;
pub use storage::endpoints as __endpoints_2__;
#[allow(non_snake_case)]
pub mod endpoints {
    use super::EndpointWrappers;
    pub use super::__endpoints_0__::*;
    pub use super::__endpoints_1__::*;
    pub use super::__endpoints_2__::*;
    pub fn init<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_init();
    }
    pub fn registerItem<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_register_item();
    }
    pub fn getItemType<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_get_item_slot();
    }
    pub fn customize<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_customize();
    }
    pub fn fill<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().call_fill();
    }
    pub fn callBack<A>()
    where
        A: elrond_wasm::api::VMApi,
    {
        super::contract_obj::<A>().callback();
    }
}
pub trait ProxyTrait:
    elrond_wasm::contract_base::ProxyObjBase
    + Sized
    + penguin_mint::ProxyTrait
    + penguin_parse::ProxyTrait
    + storage::ProxyTrait
{
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn init(
        self,
        penguins_identifier: elrond_wasm::types::TokenIdentifier<Self::Api>,
    ) -> elrond_wasm::types::ContractDeploy<Self::Api> {
        let ___address___ = self.into_fields();
        let mut ___contract_deploy___ =
            elrond_wasm::types::new_contract_deploy::<Self::Api>(___address___);
        ___contract_deploy___.push_endpoint_arg(&penguins_identifier);
        ___contract_deploy___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn register_item(
        self,
        item_slot: ItemSlot,
        items_id_to_add: MultiValueEncoded<
            Self::Api,
            elrond_wasm::types::TokenIdentifier<Self::Api>,
        >,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <SCResult<()> as elrond_wasm::elrond_codec::TopEncodeMulti>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"registerItem"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___.push_endpoint_arg(&item_slot);
        ___contract_call___.push_endpoint_arg(&items_id_to_add);
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn get_item_slot(
        self,
        item_id: &elrond_wasm::types::TokenIdentifier<Self::Api>,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <OptionalValue<ItemSlot> as elrond_wasm::elrond_codec::TopEncodeMulti>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"getItemType"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___.push_endpoint_arg(&item_id);
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn customize(
        self,
        payments: ManagedVec<Self::Api, EsdtTokenPayment<Self::Api>>,
        to_desequip_slots: MultiValueEncoded<Self::Api, ItemSlot>,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <SCResult<u64> as elrond_wasm::elrond_codec::TopEncodeMulti>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"customize"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___ = ___contract_call___.with_multi_token_transfer(payments);
        ___contract_call___.push_endpoint_arg(&to_desequip_slots);
        ___contract_call___
    }
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::type_complexity)]
    fn fill(
        self,
        _token: TokenIdentifier<Self::Api>,
        _nonce: u64,
        _amount: elrond_wasm::types::BigUint<Self::Api>,
    ) -> elrond_wasm::types::ContractCall<
        Self::Api,
        <SCResult<()> as elrond_wasm::elrond_codec::TopEncodeMulti>::DecodeAs,
    > {
        let ___address___ = self.into_fields();
        let mut ___contract_call___ = elrond_wasm::types::new_contract_call(
            ___address___,
            &b"fill"[..],
            ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new(),
        );
        ___contract_call___ = ___contract_call___.add_token_transfer(_token, _nonce, _amount);
        ___contract_call___
    }
}
pub struct Proxy<A>
where
    A: elrond_wasm::api::VMApi + 'static,
{
    pub address: elrond_wasm::types::ManagedAddress<A>,
}
impl<A> elrond_wasm::contract_base::ProxyObjBase for Proxy<A>
where
    A: elrond_wasm::api::VMApi + 'static,
{
    type Api = A;
    fn new_proxy_obj() -> Self {
        let zero_address = ManagedAddress::zero();
        Proxy {
            address: zero_address,
        }
    }
    fn contract(mut self, address: ManagedAddress<Self::Api>) -> Self {
        self.address = address;
        self
    }
    #[inline]
    fn into_fields(self) -> ManagedAddress<Self::Api> {
        self.address
    }
}
impl<A> penguin_mint::ProxyTrait for Proxy<A> where A: elrond_wasm::api::VMApi {}
impl<A> penguin_parse::ProxyTrait for Proxy<A> where A: elrond_wasm::api::VMApi {}
impl<A> storage::ProxyTrait for Proxy<A> where A: elrond_wasm::api::VMApi {}
impl<A> ProxyTrait for Proxy<A> where A: elrond_wasm::api::VMApi {}
