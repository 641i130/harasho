#!/bin/bash
echo "Don't run this unless you want new certs. Just use the ones already generated!"
exit
echo "This script removes all cert files in the directory."
echo "Continue? Press enter!"
read
rm *.srl *.key *.csr *.crt *.pem *.ext *.pfx > /dev/null
# Generate Root Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout root.key -x509 -days 3650 -out root.crt -subj "/C=JP/ST=A/L=A/CN=Taito Arcade Machine CA" 
# Generate Second Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout nesys.key -out nesys.crt -CA root.crt -CAkey root.key -subj "/C=JP/ST=A/L=A/CN=nesys" -days 3650
# Generate Last Certificate and Key
openssl req -newkey rsa:2048 -nodes -keyout nesica1.key -out nesica1.crt -CA nesys.crt -CAkey nesys.key -subj "/C=JP/ST=A/L=A/CN=nesica1" -addext "subjectAltName = DNS:cert.nesys.jp,DNS:cert3.nesys.jp,DNS:data.nesys.jp,DNS:proxy.nesys.jp,DNS:nesys.taito.co.jp,DNS:fjm170920zero.nesica.net" -days 3650
# Convert them to PFX for windows importing!
# Make sure to import them only to Local Machine Personal and Trusted Root authorities
openssl pkcs12 -export -certpbe PBE-SHA1-3DES -keypbe PBE-SHA1-3DES -nomac -inkey root.key -in root.crt -out root.pfx
openssl pkcs12 -export -certpbe PBE-SHA1-3DES -keypbe PBE-SHA1-3DES -nomac -inkey nesys.key -in nesys.crt -out nesys.pfx
openssl pkcs12 -export -certpbe PBE-SHA1-3DES -keypbe PBE-SHA1-3DES -nomac -inkey nesica1.key -in nesica1.crt -out nesica1.pfx

