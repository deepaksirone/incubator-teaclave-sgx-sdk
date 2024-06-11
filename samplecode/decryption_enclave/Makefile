# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

######## SGX SDK Settings ########

SGX_SDK ?= /opt/sgxsdk
SGX_MODE ?= HW
SGX_ARCH ?= x64

TOP_DIR := ../..
include $(TOP_DIR)/buildenv.mk

ifeq ($(shell getconf LONG_BIT), 32)
	SGX_ARCH := x86
else ifeq ($(findstring -m32, $(CXXFLAGS)), -m32)
	SGX_ARCH := x86
endif

ifeq ($(SGX_ARCH), x86)
	SGX_COMMON_CFLAGS := -m32
	SGX_LIBRARY_PATH := $(SGX_SDK)/lib
	SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x86/sgx_sign
	SGX_EDGER8R := $(SGX_SDK)/bin/x86/sgx_edger8r
else
	SGX_COMMON_CFLAGS := -m64
	SGX_LIBRARY_PATH := $(SGX_SDK)/lib64
	SGX_ENCLAVE_SIGNER := $(SGX_SDK)/bin/x64/sgx_sign
	SGX_EDGER8R := $(SGX_SDK)/bin/x64/sgx_edger8r
endif

ifeq ($(SGX_DEBUG), 1)
ifeq ($(SGX_PRERELEASE), 1)
$(error Cannot set SGX_DEBUG and SGX_PRERELEASE at the same time!!)
endif
endif

Pcl_Encryption_Tool := $(SGX_SDK)/bin/x64/sgx_encrypt
PCL_KEY := ./key.bin
PCL_KEY_SIZE := 16

ifneq ($(SGX_MODE), HW)
PCL_LIB_NAME := sgx_pclsim
else  # ifneq ($(SGX_MODE), HW)
PCL_LIB_NAME := sgx_pcl
endif # ifneq ($(SGX_MODE), HW)

#PCL_LINK_FLAGS := -Wl,--whole-archive -l$(PCL_LIB_NAME) -Wl,--no-whole-archive


ifeq ($(SGX_DEBUG), 1)
	SGX_COMMON_CFLAGS += -O0 -g
else
	SGX_COMMON_CFLAGS += -O2
endif

SGX_COMMON_CFLAGS += -fstack-protector
SGX_COMMON_CXXFLAGS := $(SGX_COMMON_CFLAGS) -std=c++11

######## CUSTOM Settings ########

CUSTOM_LIBRARY_PATH := ./lib
CUSTOM_BIN_PATH := ./bin
CUSTOM_EDL_PATH := ../../edl
CUSTOM_COMMON_PATH := ../../common

######## EDL Settings ########

Enclave_EDL_Files := enclave/Enclave_t.c enclave/Enclave_t.h app/Enclave_u.c app/Enclave_u.h

######## APP Settings ########

App_Rust_Flags := --release
App_Include_Paths := -I ./app -I./include -I$(SGX_SDK)/include -I$(CUSTOM_EDL_PATH)
App_C_Flags := $(SGX_COMMON_CFLAGS) -fPIC -Wno-attributes $(App_Include_Paths)

App_Rust_Path := ./app/target/release
App_Enclave_u_Object :=app/libEnclave_u.a
App_Name := bin/app

######## Enclave Settings ########

ifneq ($(SGX_MODE), HW)
	Trts_Library_Name := sgx_trts_sim
	Service_Library_Name := sgx_tservice_sim
else
	Trts_Library_Name := sgx_trts
	Service_Library_Name := sgx_tservice
endif
Crypto_Library_Name := sgx_tcrypto
KeyExchange_Library_Name := sgx_tkey_exchange
ProtectedFs_Library_Name := sgx_tprotected_fs

RustEnclave_C_Files := $(wildcard ./enclave/*.c)
RustEnclave_C_Objects := $(RustEnclave_C_Files:.c=.o)
RustEnclave_Include_Paths := -I$(CUSTOM_COMMON_PATH)/inc -I$(CUSTOM_EDL_PATH) -I$(SGX_SDK)/include -I$(SGX_SDK)/include/tlibc -I$(SGX_SDK)/include/stlport -I$(SGX_SDK)/include/epid -I ./enclave -I./include

RustEnclave_Link_Libs := -L$(CUSTOM_LIBRARY_PATH) -lenclave
RustEnclave_Compile_Flags := $(SGX_COMMON_CFLAGS) $(ENCLAVE_CFLAGS) $(RustEnclave_Include_Paths)
RustEnclave_Link_Flags := -Wl,--no-undefined -nostdlib -nodefaultlibs -nostartfiles -L$(SGX_LIBRARY_PATH) \
	-Wl,--whole-archive -l$(Trts_Library_Name) -Wl,--no-whole-archive \
	-Wl,--start-group -lsgx_tstdc -l$(Service_Library_Name) -l$(Crypto_Library_Name) $(RustEnclave_Link_Libs) -Wl,--end-group \
	-Wl,--version-script=enclave/Enclave.lds \
	$(ENCLAVE_LDFLAGS)

RustEnclave_Name := enclave/enclave.so
Encrypted_RustEnclave_Name := $(RustEnclave_Name).enc
Encryption_Tool_Flags := -d -bellerophon
Signed_RustEnclave_Name := bin/enclave.signed.so

#TODO: Add enclave/bellerophon/EnclaveMessageExchange.cpp enclave/bellerophon/Utility_E1.cpp
RustEnclave_Bellerophon_Files := enclave/bellerophon/bellerophon.cpp 
Enclave_Include_Paths := -Ienclave -I$(SGX_SDK)/include -I$(SGX_SDK)/include/libcxx -I$(SGX_SDK)/include/tlibc -IInclude

Enclave_C_Flags := -nostdinc -fvisibility=hidden -fpie -fstack-protector $(Enclave_Include_Paths) -DPB_ENABLE_SGX
Enclave_Cpp_Flags := $(Enclave_C_Flags) -nostdinc++

# Enable the security flags
Enclave_Security_Link_Flags := -Wl,-z,relro,-z,now,-z,noexecstack
	
# PCL source files
PCL_ASM_FILES       := enclave/bellerophon/crypto/pcl_vpaes-x86_64.s \
                       enclave/bellerophon/crypto/pcl_ghash-x86_64.s

PCL_CPP_FILES       := enclave/bellerophon/crypto/pcl_crypto.cpp \
		       enclave/bellerophon/crypto/pcl_mem.cpp

PCL_C_FILES        := enclave/bellerophon/crypto/pcl_sha256.c    \
                      enclave/bellerophon/crypto/pcl_gcm128.c

# object files
PCL_CPP_OBJECTS                 := $(PCL_CPP_FILES:%.cpp=%.o)
PCL_C_OBJECTS                   := $(PCL_C_FILES:%.c=%.o)
PCL_ASM_OBJECTS                 := $(PCL_ASM_FILES:%.s=%.o)
PCL_INCLUDE_PATH		:= -I/usr/include \
				   -I/usr/include/x86_64-linux-gnu \
				   -I./enclave/bellerophon \
				   -I./enclave/bellerophon/crypto

RustEnclave_Bellerophon_Objects := $(RustEnclave_Bellerophon_Files:.cpp=.o)

RustEnclave_Bellerophon_Objects := $(RustEnclave_Bellerophon_Objects) $(PCL_CPP_OBJECTS) $(PCL_C_OBJECTS) $(PCL_ASM_OBJECTS)

.PHONY: all
all: $(App_Name) $(Signed_RustEnclave_Name)

######## EDL Objects ########

$(Enclave_EDL_Files): $(SGX_EDGER8R) enclave/Enclave.edl
	$(SGX_EDGER8R) --trusted enclave/Enclave.edl --search-path $(SGX_SDK)/include --search-path $(CUSTOM_EDL_PATH) --trusted-dir enclave
	$(SGX_EDGER8R) --untrusted enclave/Enclave.edl --search-path $(SGX_SDK)/include --search-path $(CUSTOM_EDL_PATH) --untrusted-dir app
	@echo "GEN  =>  $(Enclave_EDL_Files)"

######## App Objects ########

app/Enclave_u.o: $(Enclave_EDL_Files)
	@$(CC) $(App_C_Flags) -c app/Enclave_u.c -o $@
	@echo "CC   <=  $<"

$(App_Enclave_u_Object): app/Enclave_u.o
	$(AR) rcsD $@ $^
	cp $(App_Enclave_u_Object) ./lib

$(App_Name): $(App_Enclave_u_Object)
	@cd app && SGX_SDK=$(SGX_SDK) SGX_MODE=$(SGX_MODE) cargo build $(App_Rust_Flags)
	@echo "Cargo  =>  $@"
	mkdir -p bin
	cp $(App_Rust_Path)/app ./bin

######## Enclave Objects ########

enclave/Enclave_t.o: $(Enclave_EDL_Files)
	@$(CC) $(RustEnclave_Compile_Flags) -c enclave/Enclave_t.c -o $@
	@echo "CC <= $<"

enclave/bellerophon/bellerophon.o: enclave/bellerophon/bellerophon.cpp enclave/Enclave_t.h
	$(CXX) $(SGX_COMMON_CXXFLAGS) $(RustEnclave_Compile_Flags) $(Enclave_Cpp_Flags) -c $< -o $@
	@echo "CXX <= $<"

enclave/bellerophon/EnclaveMessageExchange.o: enclave/bellerophon/EnclaveMessageExchange.cpp enclave/Enclave_t.h
	$(CXX) $(SGX_COMMON_CXXFLAGS) $(RustEnclave_Compile_Flags)  $(Enclave_Cpp_Flags) -c $< -o $@
	@echo "CXX <= $<"

enclave/bellerophon/Utility_E1.o: enclave/bellerophon/Utility_E1.cpp
	$(CXX) $(SGX_COMMON_CXXFLAGS) $(RustEnclave_Compile_Flags)  $(Enclave_Cpp_Flags) -c $< -o $@
	@echo "CXX <= $<"

enclave/bellerophon/crypto/%.o: enclave/bellerophon/crypto/%.s
	@$(CC) $(SGX_COMMON_CFLAGS) $(RustEnclave_Compile_Flags)  $(PCL_INCLUDE_PATH) $(Enclave_C_Flags) -c $< -o $@
	@echo "CC   <=  $<"

enclave/bellerophon/crypto/%.o: enclave/bellerophon/crypto/%.c
	@$(CC) $(SGX_COMMON_CFLAGS) $(RustEnclave_Compile_Flags)  $(PCL_INCLUDE_PATH) $(Enclave_C_Flags) -c $< -o $@
	@echo "CC   <=  $<"

enclave/bellerophon/crypto/%.o: enclave/bellerophon/crypto/%.cpp
	@$(CC) $(SGX_COMMON_CXXFLAGS) $(RustEnclave_Compile_Flags)  $(PCL_INCLUDE_PATH) $(Enclave_Cpp_Flags) -c $< -o $@
	@echo "CC   <=  $<"

$(RustEnclave_Bellerophon_Objects): enclave/Enclave_t.h 


#enclave/bellerophon.o: enclave/bellerophon/bellerophon.cpp
#	@$(CXX) $(RustEnclave_Compile_Flags) -c enclave/bellerophon/bellerophon.cpp -o $@
#	$(OBJCOPY) $@ --rename-section .text=.nipx
#	$(OBJCOPY) $@ --rename-section .data=.nipx
#	$(OBJCOPY) $@ --rename-section .rodata=.niprod
#	@echo "CXX  <=  $<"


$(RustEnclave_Name): enclave enclave/Enclave_t.o $(RustEnclave_Bellerophon_Objects)
	$(CXX) enclave/Enclave_t.o $(RustEnclave_Bellerophon_Objects) -o $@ $(RustEnclave_Link_Flags) #$(PCL_LINK_FLAGS)
	@echo "LINK =>  $@"

$(Signed_RustEnclave_Name): $(RustEnclave_Name) $(Pcl_Encryption_Tool) $(PCL_KEY)
	@echo -n "Encrypt key: "
	@hexdump -v -e '/1 "%02X"' $(PCL_KEY)
	@echo ""
	$(Pcl_Encryption_Tool) -i $(RustEnclave_Name) -o $(Encrypted_RustEnclave_Name) -k $(PCL_KEY) $(Encryption_Tool_Flags)
	@echo "Encrypted => $(Encrypted_RustEnclave_Name)"

	mkdir -p bin
	@$(SGX_ENCLAVE_SIGNER) sign -key enclave/Enclave_private.pem -enclave $(Encrypted_RustEnclave_Name) -out $@ -config enclave/Enclave.config.xml
	@echo "SIGN =>  $@"

.PHONY: enclave
enclave:
	$(MAKE) -C ./enclave/


.PHONY: clean
clean:
	@rm -f $(App_Name) $(RustEnclave_Name) $(Signed_RustEnclave_Name) enclave/bellerophon/*.o enclave/bellerophon/crypto/*.o enclave/*.o enclave/*.enc enclave/*_t.* app/*_u.* lib/*.a
	@cd enclave && cargo clean && rm -f Cargo.lock
	@cd app && cargo clean && rm -f Cargo.lock
