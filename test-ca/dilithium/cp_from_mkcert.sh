#!/bin/bash

set -e

pushd ../../../mk-cert
env ROOT_SIGALG=dilithium2 INT_SIGALG=dilithium2 LEAF_SIGALG=dilithium2 \
    HOSTNAMES=testserver.com,second.testserver.com,localhost pipenv run python encoder.py
popd

cp ../../../mk-cert/signing.crt end.cert
cp ../../../mk-cert/signing.key end.key
cp ../../../mk-cert/signing-int.crt inter.cert
cp ../../../mk-cert/signing-int.key inter.key
cp ../../../mk-cert/signing-ca.crt ca.cert
cp ../../../mk-cert/signing-ca.key ca.key
cat inter.cert ca.cert > end.chain
cat end.cert {inter,ca}.cert > end.fullchain

