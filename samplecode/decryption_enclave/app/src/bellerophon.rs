
use sgx_types::*;

#[no_mangle]
extern "C" fn hello_from_pcl() -> sgx_status_t
{
    println!("[hello_from_pcl] Calling an ocall from the PCL code");
    sgx_status_t::SGX_SUCCESS
}
