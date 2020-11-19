#!/bin/bash

set -e

pushd ../../../mk-cert
env ROOT_SIGALG=dilithium2 INT_SIGALG=dilithium2 LEAF_SIGALG=dilithium2 KEX_ALG=kyber512 \
    HOSTNAMES=testserver.com,second.testserver.com,localhost pipenv run python encoder.py
popd

cp ../../../mk-cert/kyber512.crt end.cert
cp ../../../mk-cert/kyber512.key end.key
cp ../../../mk-cert/kem-int.crt inter.cert
cp ../../../mk-cert/kem-int.key inter.key
cp ../../../mk-cert/kem-ca.crt ca.cert
cp ../../../mk-cert/kem-ca.key ca.key
cp ../../../mk-cert/kem-client.key client.key
cp ../../../mk-cert/kem-client.crt client.cert
cat inter.cert ca.cert > end.chain
cp end.chain client.chain
cat end.cert {inter,ca}.cert > end.fullchain
cat client.cert {inter,ca}.cert > client.fullchain
