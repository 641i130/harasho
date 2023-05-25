#!/bin/bash

echo "This script removes all cert files in the directory."
echo "Continue? Press enter!"
read
rm *.key *.csr *.crt *.pem
# Generate Root Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout root.key -x509 -days 3650 -out root.crt -subj "/C=JP/ST=A/L=A/CN=Taito Arcade Machine CA"

# Generate Second Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout nesys.key -out nesys.csr -subj "/C=JP/ST=A/L=A/CN=nesys"
openssl x509 -req -in nesys.csr -CA root.crt -CAkey root.key -CAcreateserial -out nesys.crt -days 1825

# Generate Last Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout nesica1.key -out nesica1.csr -subj "/C=JP/ST=A/L=A/CN=nesica1"
openssl x509 -req -in nesica1.csr -CA nesys.crt -CAkey nesys.key -CAcreateserial -out nesica1.pem -days 730

echo "Use nesica1.pem and nesica1.key for the webserver!"

