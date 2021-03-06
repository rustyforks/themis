// Copyright 2018 (c) rust-themis developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Raw FFI bindings to libthemis.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
// For some weird reasons Clippy gets run on this crate as it's a path dependency of themis.
// This should not happen (see https://github.com/rust-lang-nursery/rust-clippy/issues/1066)
// but until that's fixed again, disable all lints which get triggered by the code generated
// by bindgen.
#![allow(clippy::all)]

/* automatically generated by rust-bindgen */

pub const THEMIS_SUCCESS: u32 = 0;
pub const THEMIS_SSESSION_SEND_OUTPUT_TO_PEER: u32 = 1;
pub const THEMIS_FAIL: u32 = 11;
pub const THEMIS_INVALID_PARAMETER: u32 = 12;
pub const THEMIS_NO_MEMORY: u32 = 13;
pub const THEMIS_BUFFER_TOO_SMALL: u32 = 14;
pub const THEMIS_DATA_CORRUPT: u32 = 15;
pub const THEMIS_INVALID_SIGNATURE: u32 = 16;
pub const THEMIS_NOT_SUPPORTED: u32 = 17;
pub const THEMIS_SSESSION_KA_NOT_FINISHED: u32 = 19;
pub const THEMIS_SSESSION_TRANSPORT_ERROR: u32 = 20;
pub const THEMIS_SSESSION_GET_PUB_FOR_ID_CALLBACK_ERROR: u32 = 21;
pub const THEMIS_SCOMPARE_SEND_OUTPUT_TO_PEER: u32 = 1;
pub const THEMIS_SCOMPARE_MATCH: u32 = 21;
pub const THEMIS_SCOMPARE_NO_MATCH: u32 = 22;
pub const THEMIS_SCOMPARE_NOT_READY: u32 = 0;
pub const THEMIS_SESSION_ID_TAG: &'static [u8; 5usize] = b"TSID\0";
pub const THEMIS_SESSION_PROTO_TAG: &'static [u8; 5usize] = b"TSPM\0";
pub const STATE_IDLE: u32 = 0;
pub const STATE_NEGOTIATING: u32 = 1;
pub const STATE_ESTABLISHED: u32 = 2;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int32_t = ::std::os::raw::c_int;
pub type __ssize_t = ::std::os::raw::c_long;
pub type themis_status_t = i32;
extern "C" {
    pub fn themis_secure_cell_encrypt_seal(
        master_key: *const u8,
        master_key_length: usize,
        user_context: *const u8,
        user_context_length: usize,
        message: *const u8,
        message_length: usize,
        encrypted_message: *mut u8,
        encrypted_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_cell_decrypt_seal(
        master_key: *const u8,
        master_key_length: usize,
        user_context: *const u8,
        user_context_length: usize,
        encrypted_message: *const u8,
        encrypted_message_length: usize,
        plain_message: *mut u8,
        plain_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_cell_encrypt_seal_with_passphrase(
        passphrase: *const u8,
        passphrase_length: usize,
        user_context: *const u8,
        user_context_length: usize,
        message: *const u8,
        message_length: usize,
        encrypted_message: *mut u8,
        encrypted_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_cell_decrypt_seal_with_passphrase(
        passphrase: *const u8,
        passphrase_length: usize,
        user_context: *const u8,
        user_context_length: usize,
        encrypted_message: *const u8,
        encrypted_message_length: usize,
        plain_message: *mut u8,
        plain_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_cell_encrypt_token_protect(
        master_key: *const u8,
        master_key_length: usize,
        user_context: *const u8,
        user_context_length: usize,
        message: *const u8,
        message_length: usize,
        context: *mut u8,
        context_length: *mut usize,
        encrypted_message: *mut u8,
        encrypted_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_cell_decrypt_token_protect(
        master_key: *const u8,
        master_key_length: usize,
        user_context: *const u8,
        user_context_length: usize,
        encrypted_message: *const u8,
        encrypted_message_length: usize,
        context: *const u8,
        context_length: usize,
        plain_message: *mut u8,
        plain_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_cell_encrypt_context_imprint(
        master_key: *const u8,
        master_key_length: usize,
        message: *const u8,
        message_length: usize,
        context: *const u8,
        context_length: usize,
        encrypted_message: *mut u8,
        encrypted_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_cell_decrypt_context_imprint(
        master_key: *const u8,
        master_key_length: usize,
        encrypted_message: *const u8,
        encrypted_message_length: usize,
        context: *const u8,
        context_length: usize,
        plain_message: *mut u8,
        plain_message_length: *mut usize,
    ) -> themis_status_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secure_comparator_type {
    _unused: [u8; 0],
}
pub type secure_comparator_t = secure_comparator_type;
extern "C" {
    pub fn secure_comparator_create() -> *mut secure_comparator_t;
}
extern "C" {
    pub fn secure_comparator_destroy(comp_ctx: *mut secure_comparator_t) -> themis_status_t;
}
extern "C" {
    pub fn secure_comparator_append_secret(
        comp_ctx: *mut secure_comparator_t,
        secret_data: *const ::std::os::raw::c_void,
        secret_data_length: usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_comparator_begin_compare(
        comp_ctx: *mut secure_comparator_t,
        compare_data: *mut ::std::os::raw::c_void,
        compare_data_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_comparator_proceed_compare(
        comp_ctx: *mut secure_comparator_t,
        peer_compare_data: *const ::std::os::raw::c_void,
        peer_compare_data_length: usize,
        compare_data: *mut ::std::os::raw::c_void,
        compare_data_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_comparator_get_result(comp_ctx: *const secure_comparator_t) -> themis_status_t;
}
extern "C" {
    pub fn themis_gen_sym_key(key: *mut u8, key_length: *mut usize) -> themis_status_t;
}
extern "C" {
    pub fn themis_gen_rsa_key_pair(
        private_key: *mut u8,
        private_key_length: *mut usize,
        public_key: *mut u8,
        public_key_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_gen_ec_key_pair(
        private_key: *mut u8,
        private_key_length: *mut usize,
        public_key: *mut u8,
        public_key_length: *mut usize,
    ) -> themis_status_t;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum themis_key_kind {
    THEMIS_KEY_INVALID = 0,
    THEMIS_KEY_RSA_PRIVATE = 1,
    THEMIS_KEY_RSA_PUBLIC = 2,
    THEMIS_KEY_EC_PRIVATE = 3,
    THEMIS_KEY_EC_PUBLIC = 4,
}
pub use self::themis_key_kind as themis_key_kind_t;
extern "C" {
    pub fn themis_get_asym_key_kind(key: *const u8, length: usize) -> themis_key_kind_t;
}
extern "C" {
    pub fn themis_is_valid_asym_key(key: *const u8, length: usize) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_message_encrypt(
        private_key: *const u8,
        private_key_length: usize,
        public_key: *const u8,
        public_key_length: usize,
        message: *const u8,
        message_length: usize,
        encrypted_message: *mut u8,
        encrypted_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_message_decrypt(
        private_key: *const u8,
        private_key_length: usize,
        public_key: *const u8,
        public_key_length: usize,
        encrypted_message: *const u8,
        encrypted_message_length: usize,
        message: *mut u8,
        message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_message_sign(
        private_key: *const u8,
        private_key_length: usize,
        message: *const u8,
        message_length: usize,
        signed_message: *mut u8,
        signed_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_message_verify(
        public_key: *const u8,
        public_key_length: usize,
        signed_message: *const u8,
        signed_message_length: usize,
        message: *mut u8,
        message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_message_wrap(
        private_key: *const u8,
        private_key_length: usize,
        public_key: *const u8,
        public_key_length: usize,
        message: *const u8,
        message_length: usize,
        wrapped_message: *mut u8,
        wrapped_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn themis_secure_message_unwrap(
        private_key: *const u8,
        private_key_length: usize,
        public_key: *const u8,
        public_key_length: usize,
        wrapped_message: *const u8,
        wrapped_message_length: usize,
        message: *mut u8,
        message_length: *mut usize,
    ) -> themis_status_t;
}
pub type send_protocol_data_callback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *const u8,
        data_length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ) -> isize,
>;
pub type receive_protocol_data_callback = ::std::option::Option<
    unsafe extern "C" fn(
        data: *mut u8,
        data_length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ) -> isize,
>;
pub type protocol_state_changed_callback = ::std::option::Option<
    unsafe extern "C" fn(event: ::std::os::raw::c_int, user_data: *mut ::std::os::raw::c_void),
>;
pub type get_public_key_for_id_callback = ::std::option::Option<
    unsafe extern "C" fn(
        id: *const ::std::os::raw::c_void,
        id_length: usize,
        key_buffer: *mut ::std::os::raw::c_void,
        key_buffer_length: usize,
        user_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secure_session_user_callbacks_type {
    pub send_data: send_protocol_data_callback,
    pub receive_data: receive_protocol_data_callback,
    pub state_changed: protocol_state_changed_callback,
    pub get_public_key_for_id: get_public_key_for_id_callback,
    pub user_data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_secure_session_user_callbacks_type() {
    assert_eq!(
        ::std::mem::size_of::<secure_session_user_callbacks_type>(),
        40usize,
        concat!("Size of: ", stringify!(secure_session_user_callbacks_type))
    );
    assert_eq!(
        ::std::mem::align_of::<secure_session_user_callbacks_type>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(secure_session_user_callbacks_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_user_callbacks_type>())).send_data as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_user_callbacks_type),
            "::",
            stringify!(send_data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_user_callbacks_type>())).receive_data as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_user_callbacks_type),
            "::",
            stringify!(receive_data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_user_callbacks_type>())).state_changed as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_user_callbacks_type),
            "::",
            stringify!(state_changed)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_user_callbacks_type>())).get_public_key_for_id
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_user_callbacks_type),
            "::",
            stringify!(get_public_key_for_id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_user_callbacks_type>())).user_data as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_user_callbacks_type),
            "::",
            stringify!(user_data)
        )
    );
}
pub type secure_session_user_callbacks_t = secure_session_user_callbacks_type;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secure_session_peer_type {
    pub id: *mut u8,
    pub id_length: usize,
    pub ecdh_key: *mut u8,
    pub ecdh_key_length: usize,
    pub sign_key: *mut u8,
    pub sign_key_length: usize,
}
#[test]
fn bindgen_test_layout_secure_session_peer_type() {
    assert_eq!(
        ::std::mem::size_of::<secure_session_peer_type>(),
        48usize,
        concat!("Size of: ", stringify!(secure_session_peer_type))
    );
    assert_eq!(
        ::std::mem::align_of::<secure_session_peer_type>(),
        8usize,
        concat!("Alignment of ", stringify!(secure_session_peer_type))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<secure_session_peer_type>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_peer_type),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_peer_type>())).id_length as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_peer_type),
            "::",
            stringify!(id_length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_peer_type>())).ecdh_key as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_peer_type),
            "::",
            stringify!(ecdh_key)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_peer_type>())).ecdh_key_length as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_peer_type),
            "::",
            stringify!(ecdh_key_length)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_peer_type>())).sign_key as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_peer_type),
            "::",
            stringify!(sign_key)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<secure_session_peer_type>())).sign_key_length as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(secure_session_peer_type),
            "::",
            stringify!(sign_key_length)
        )
    );
}
pub type secure_session_peer_t = secure_session_peer_type;
extern "C" {
    pub fn secure_session_peer_init(
        peer: *mut secure_session_peer_t,
        id: *const ::std::os::raw::c_void,
        id_len: usize,
        ecdh_key: *const ::std::os::raw::c_void,
        ecdh_key_len: usize,
        sign_key: *const ::std::os::raw::c_void,
        sign_key_len: usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_session_peer_cleanup(peer: *mut secure_session_peer_t);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secure_session_type {
    _unused: [u8; 0],
}
pub type secure_session_t = secure_session_type;
pub type secure_session_handler = ::std::option::Option<
    unsafe extern "C" fn(
        session_ctx: *mut secure_session_t,
        data: *const ::std::os::raw::c_void,
        data_length: usize,
        output: *mut ::std::os::raw::c_void,
        output_length: *mut usize,
    ) -> themis_status_t,
>;
extern "C" {
    pub fn secure_session_create(
        id: *const ::std::os::raw::c_void,
        id_length: usize,
        sign_key: *const ::std::os::raw::c_void,
        sign_key_length: usize,
        user_callbacks: *const secure_session_user_callbacks_t,
    ) -> *mut secure_session_t;
}
extern "C" {
    pub fn secure_session_destroy(session_ctx: *mut secure_session_t) -> themis_status_t;
}
extern "C" {
    pub fn secure_session_connect(session_ctx: *mut secure_session_t) -> themis_status_t;
}
extern "C" {
    pub fn secure_session_generate_connect_request(
        session_ctx: *mut secure_session_t,
        output: *mut ::std::os::raw::c_void,
        output_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_session_wrap(
        session_ctx: *mut secure_session_t,
        message: *const ::std::os::raw::c_void,
        message_length: usize,
        wrapped_message: *mut ::std::os::raw::c_void,
        wrapped_message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_session_unwrap(
        session_ctx: *mut secure_session_t,
        wrapped_message: *const ::std::os::raw::c_void,
        wrapped_message_length: usize,
        message: *mut ::std::os::raw::c_void,
        message_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_session_send(
        session_ctx: *mut secure_session_t,
        message: *const ::std::os::raw::c_void,
        message_length: usize,
    ) -> isize;
}
extern "C" {
    pub fn secure_session_receive(
        session_ctx: *mut secure_session_t,
        message: *mut ::std::os::raw::c_void,
        message_length: usize,
    ) -> isize;
}
extern "C" {
    pub fn secure_session_save(
        session_ctx: *const secure_session_t,
        out: *mut ::std::os::raw::c_void,
        out_length: *mut usize,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_session_load(
        session_ctx: *mut secure_session_t,
        in_: *const ::std::os::raw::c_void,
        in_length: usize,
        user_callbacks: *const secure_session_user_callbacks_t,
    ) -> themis_status_t;
}
extern "C" {
    pub fn secure_session_is_established(session_ctx: *const secure_session_t) -> bool;
}
extern "C" {
    pub fn secure_session_get_remote_id(
        session_ctx: *const secure_session_t,
        id: *mut u8,
        id_length: *mut usize,
    ) -> themis_status_t;
}
