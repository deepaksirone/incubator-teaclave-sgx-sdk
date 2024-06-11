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

extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;

pub mod bellerophon;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
    fn sample_main (eid: sgx_enclave_id_t, retval: *mut sgx_status_t) -> sgx_status_t;
    fn say_something(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
                     some_string: *const u8, len: usize) -> sgx_status_t;

    //fn decrypt_enclave (eid: sgx_enclave_id_t, retval: *mut sgx_status_t) -> sgx_status_t;
}

extern "C" {
    fn decrypt_enclave (eid: sgx_enclave_id_t, retval: *mut sgx_status_t, decrypt: int64_t) -> sgx_status_t;
}

#[no_mangle]
extern "C" fn ocall_print_string(ptr: *const c_char) { 
    let cstr = unsafe { std::ffi::CStr::from_ptr(ptr) };
    let s = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    println!("{}", s);
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

fn main() {

    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let mut retval = sgx_status_t::SGX_SUCCESS;

  
    let mut result = unsafe {
        decrypt_enclave(enclave.geteid(), &mut retval, 1)
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {
            println!("[+] Decryption Succeeded");

        },
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }


    result = unsafe {
        sample_main(enclave.geteid(),
                    &mut retval)
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    let input_string = String::from("This is a normal world string passed into Enclave!\n");

    result = unsafe {
        say_something(enclave.geteid(),
                      &mut retval,
                      input_string.as_ptr() as * const u8,
                      input_string.len())
    };

    match result {
        sgx_status_t::SGX_SUCCESS => {},
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }



    println!("[+] say_something success...");

    enclave.destroy();
}