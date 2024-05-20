#include "sgx_eid.h"
//#include "EnclaveInitiator_t.h"
//#include "EnclaveMessageExchange.h"
//#include "error_codes.h"
//#include "Utility_E1.h"
#include "sgx_dh.h"
#include "sgx_utils.h"
#include "../Enclave_t.h"
//#include <map>

extern "C" sgx_status_t get_key_bellerophon(uint8_t *key) __attribute__((section(".nipx")));

extern "C" sgx_status_t get_key_bellerophon(uint8_t *key)
{
	//dh_session_t decrypt_enclave_session;
	sgx_status_t stat;
	return hello_from_pcl(&stat);

	//ATTESTATION_STATUS stat = create_session_with_decryptor(&decrypt_enclave_session);
	
	//if (stat != SGX_SUCCESS) {
		//return stat;
	//}

	// Get enclave report and send to the decryptor enclave
	// Get back decryption key corresponding to enclave hash
	// Copy decryption key back to *key
	
}
