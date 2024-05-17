// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![crate_name = "decryptionenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;

//extern crate sgx_rand as rand;
//extern crate serde;
//extern crate serde_json;

#[link_name = "sgx_pcl"]
extern "C" {
    pub fn pcl_entry_bellerophon(elf_base: *const c_void, key: *const u8);
}

//#[link_name = "enclave_hello"]
//extern "C" {
//    pub fn say_something(some_string: *const u8, some_len: usize) -> sgx_status_t;
//}

// TODO: Take some policy as a parameter
#[no_mangle]
#[link_section = ".nipx"]
pub extern "C"
fn decrypt_enclave() -> sgx_status_t {
    let key: &[u8] = &[0xa; 16];
    let elf_base = std::enclave::get_enclave_base();
    unsafe {
        pcl_entry_bellerophon(elf_base as *const c_void, key.as_ptr() as *const u8);
    }
    sgx_status_t::SGX_SUCCESS 
}

#[no_mangle]
pub extern "C"
fn sample_main() -> sgx_status_t {

    sgx_status_t::SGX_SUCCESS
}
