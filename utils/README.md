# Utils

todo (add the missing encrypted endpoints)

---

# Encryption testing with curl and openssl
### `/basicinfo` request
`curl -X POST http://localhost/basicinfo -o encrypted_data.bin`

Decrypt:

`openssl rsautl -inkey public_key.pem -pubin -in encrypted_data.bin -raw`
(it decrypts with public key and encrypts with private)

Oneliner:

`curl -X POST http://localhost/basicinfo | openssl rsautl -inkey public_key.pem -pubin`

### `/game/*` request
`curl -X POST http://localhost/game -o aes.bin`

Decrypt:

(key and iv are `0123456789012345` in hex format for openssl)

`openssl enc -d -aes-128-cfb -in aes.bin -out lol.txt -K '3031323334353637383930313233343536373839303132333435363738393031' -iv '30313233343536373839303132333435'`

Oneliner:
`curl -X POST http://localhost/game | openssl enc -d -aes-128-cfb -K '30313233343536373839303132333435' -iv '3031323334353637383930313
2333435'`
