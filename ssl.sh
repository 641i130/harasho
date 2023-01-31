#Step 1: Create a Root CA key
openssl genpkey -algorithm RSA -out rootCA.key

# Step 2: Create a Root CA self-signed certificate
openssl req -new -key rootCA.key -out rootCA.csr -subj "/CN=nesys"
openssl x509 -req -in rootCA.csr -out rootCA.crt -signkey rootCA.key -days 3650

# Step 3: Create a certificate key
openssl genpkey -algorithm RSA -out cert.key

# Step 4: Create a certificate signing request (CSR)
openssl req -new -key cert.key -out cert.csr -subj "/CN=PREMIUM"

# Step 5: Add subject alternative names (SAN) to the CSR
echo "subjectAltName = IP:127.0.0.1,DNS:localhost,DNS:cert3.nesys.jp,DNS:data.nesys.jp,DNS:proxy.nesys.jp,DNS:nesys.taito.co.jp,DNS:fjm170920zero.nesica.net" >> cert.ext
openssl req -in cert.csr -out cert.csr -config cert.ext

# Step 6: Sign the certificate with the Root CA
openssl x509 -req -in cert.csr -out cert.crt -CA rootCA.crt -CAkey rootCA.key -CAcreateserial -days 3650 -extfile cert.ext

# Finalize
openssl x509 -in cert.crt -out cert.pem -outform PEM
openssl rsa -in cert.key -out key.pem -outform PEM