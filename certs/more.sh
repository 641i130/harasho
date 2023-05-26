#!/bin/bash

echo "This script removes all cert files in the directory."
echo "Continue? Press enter!"
read
rm *.key *.csr *.crt *.pem > /dev/null
# Generate Root Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout root.key -x509 -days 3650 -out root.csr -subj "/C=JP/ST=A/L=A/CN=Taito Arcade Machine CA"

# Generate Second Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout nesys.key -out nesys.csr -subj "/C=JP/ST=A/L=A/CN=nesys"
# Sign with root cert
openssl x509 -req -in nesys.csr -CA root.csr -CAkey root.key -CAcreateserial -out nesys.csr -days 1825

# Generate Last Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout nesica1.key -out nesica1.csr -subj "/C=JP/ST=A/L=A/CN=nesica1"

# Add domains
echo "subjectAltName = DNS:cert.nesys.jp,DNS:cert3.nesys.jp,DNS:data.nesys.jp,DNS:proxy.nesys.jp,DNS:nesys.taito.co.jp,DNS:fjm170920zero.nesica.net" > cert.ext
openssl req -in nesica1.csr -out nesica1.csr -config cert.ext

# Sign with nesys cert
openssl x509 -req -in nesica1.csr -CA nesys.csr -CAkey nesys.key -CAcreateserial -out nesica1.csr -days 730

openssl x509 -in nesica1.csr -out cert.pem -outform PEM
openssl rsa -in nesica1.key -out key.pem -outform PEM

echo "Use nesica1.pem and nesica1.key for the webserver!"

