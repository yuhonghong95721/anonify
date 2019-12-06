/* automatically generated by rust-bindgen */

#![allow(dead_code)]
use anonify_types::*;
use sgx_types::*;

extern "C" {
    pub fn ecall_get_state(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        sig: *mut [u8; 64usize],
        pubkey: *mut [u8; 32usize],
        msg: *mut [u8; 32usize],
        state: *mut u64,
    ) -> sgx_status_t;
}
extern "C" {
    pub fn ecall_state_transition(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        sig: *mut [u8; 64usize],
        pubkey: *mut [u8; 32usize],
        target: *mut [u8; 20usize],
        value: *mut u64,
        ciphertext1: *mut [u8; 60usize],
        ciphertext2: *mut [u8; 60usize],
    ) -> sgx_status_t;
}
extern "C" {
    pub fn ecall_init_state(
        eid: sgx_enclave_id_t,
        retval: *mut sgx_status_t,
        sig: *mut [u8; 64usize],
        pubkey: *mut [u8; 32usize],
        value: *mut u64,
        ciphertext: *mut [u8; 60usize],
    ) -> sgx_status_t;
}
extern "C" {
    pub fn ecall_run_tests(
        eid: sgx_enclave_id_t,
        ext_ptr: *const RawPointer,
        result: *mut ResultStatus,
    ) -> sgx_status_t;
}
