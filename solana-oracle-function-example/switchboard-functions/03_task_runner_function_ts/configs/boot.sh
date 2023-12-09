#!/bin/bash

# Boots a Switchboard SGX Function

date_str=$(date "+%Y-%m-%d %H:%M:%S %Z")
echo "${date_str} boot.sh"

set -eo pipefail
set +u

if [[ ! -e "/sgx/nodejs/index.js" ]]; then
    echo "ERROR: index.js not found at /sgx/nodejs"
    exit 1
fi

if [[ ! -e "/sgx/nodejs.manifest" ]]; then
    echo "ERROR: function nodejs.manifest not found at /sgx"
    exit 1
fi

if [[ ! -e "/sgx/nodejs.manifest.sgx" ]]; then
    echo "ERROR: function nodejs.manifest.sgx manifest not found at /sgx"
    exit 1
fi

# Start SGX-enabled application
date_str=$(date "+%Y-%m-%d %H:%M:%S %Z")
echo "${date_str} Starting enclave.."
gramine-sgx /sgx/nodejs
