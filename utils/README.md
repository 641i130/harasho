# Utils

todo (add the missing ones)

---

# Encryption testing with curl and openssl
### `/basicinfo` request
`curl -X POST http://data.nesys.jp/basicinfo -o encrypted_data.bin`
Decrypt:
`openssl rsautl -inkey pub_key.pem -pubin -in encrypted_data.bin -raw`
(it decrypts with public key and encrypts with private)

### `/game/*` request
`curl -X POST http://10.3.0.141/game -o aes.bin`
Decrypt:
(key and iv are `0123456789012345` in hex format for openssl)
`openssl enc -d -aes-128-cfb -in aes.bin -out lol.txt -K '303132333435363
73839303132333435' -iv '30313233343536373839303132333435'`
