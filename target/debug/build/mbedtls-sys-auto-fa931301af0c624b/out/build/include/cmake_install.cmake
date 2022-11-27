# Install script for directory: /home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/neel/Desktop/unswCasualProfessional_Files/server_casProf/tls_server_client/target/debug/build/mbedtls-sys-auto-fa931301af0c624b/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "0")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/mbedtls" TYPE FILE PERMISSIONS OWNER_READ OWNER_WRITE GROUP_READ WORLD_READ FILES
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/aes.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/aesni.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/arc4.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/aria.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/asn1.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/asn1write.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/base64.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/bignum.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/blowfish.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/bn_mul.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/camellia.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ccm.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/certs.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/chacha20.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/chachapoly.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/check_config.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/cipher.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/cipher_internal.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/cmac.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/compat-1.3.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/config.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/config_psa.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/constant_time.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ctr_drbg.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/debug.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/des.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/dhm.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ecdh.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ecdsa.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ecjpake.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ecp.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ecp_internal.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/entropy.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/entropy_poll.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/error.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/gcm.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/havege.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/hkdf.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/hmac_drbg.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/md.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/md2.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/md4.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/md5.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/md_internal.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/memory_buffer_alloc.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/net.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/net_sockets.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/nist_kw.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/oid.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/padlock.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/pem.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/pk.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/pk_internal.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/pkcs11.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/pkcs12.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/pkcs5.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/platform.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/platform_time.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/platform_util.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/poly1305.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/psa_util.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ripemd160.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/rsa.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/rsa_internal.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/sha1.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/sha256.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/sha512.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ssl.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ssl_cache.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ssl_ciphersuites.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ssl_cookie.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ssl_internal.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/ssl_ticket.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/threading.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/timing.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/version.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/x509.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/x509_crl.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/x509_crt.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/x509_csr.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/mbedtls/xtea.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/psa" TYPE FILE PERMISSIONS OWNER_READ OWNER_WRITE GROUP_READ WORLD_READ FILES
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_builtin_composites.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_builtin_primitives.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_compat.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_config.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_driver_common.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_driver_contexts_composites.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_driver_contexts_primitives.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_extra.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_platform.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_se_driver.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_sizes.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_struct.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_types.h"
    "/home/neel/.cargo/registry/src/github.com-1ecc6299db9ec823/mbedtls-sys-auto-2.28.0/vendor/include/psa/crypto_values.h"
    )
endif()

