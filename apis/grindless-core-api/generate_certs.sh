#!/bin/bash

# Configuration variables
COMMON_NAME="nervio.us"
ORGANIZATION="Nervio"
ORGANIZATIONAL_UNIT="ID"
LOCALITY="Medellin"
STATE="Antioquia"
COUNTRY="CO"

# This certificates are used to other things not related to SSL or TLS
# Generate self-signed certificate
openssl req -x509 -newkey rsa:4096 -nodes \
    -keyout key.pem -out cert.pem -days 365 \
    -subj "/CN=${COMMON_NAME}/O=${ORGANIZATION}/OU=${ORGANIZATIONAL_UNIT}/L=${LOCALITY}/ST=${STATE}/C=${COUNTRY}"

echo "Self-signed certificate generated:"
ls -l cert.pem key.pem