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

`openssl enc -d -aes-128-cfb -in aes.bin -out lol.txt -K '30313233343536373839303132333435' -iv '30313233343536373839303132333435'`

Oneliner:
`curl -X POST http://localhost/game | openssl enc -d -aes-128-cfb -K '30313233343536373839303132333435' -iv '30313233343536373839303132333435'`


### Test a game command:
The python file pads the given protocol string, then openssl encrypts it for the server to then decrypt and respond. With the response we get, we are able to decrypt that using the same method.

`python pad_plaintext.py '{"game":{"eventcode":"000","version":"2.4.1"},"param":{},"protocol":"unlock"}' | openssl enc -e -aes-128-cfb -K '30313233343536373839303132333435' -iv '30313233343536373839303132333435' | curl -X POST -H "Content-Type: application/octet-stream" --data-binary @- http://10.3.0.141/game | openssl enc -d -aes-128-cfb -K '30313233343536373839303132333435' -iv '30313233343536373839303132333435'`

`python pad_plaintext.py '{"game":{"eventcode":"000","version":"2.4.1"},"param":{},"protocol":"unlock","terminal":{"tenpo_id":"1337","tenpo_index":1337,"terminal_attrib":0,"terminal_id":"1C1B0D07CDBB"}}' | openssl enc -e -aes-128-cfb -K '30313233343536373839303132333435' -iv '30313233343536373839303132333435' | curl -X POST -H "Content-Type: application/octet-stream" --data-binary @- http://localhost/game | openssl enc -d -aes-128-cfb -K '30313233343536373839303132333435' -iv '30313233343536373839303132333435'`

Test `unlock`:
`python pad_plaintext.py '{"game":{"eventcode":"000","version":"2.4.1"},"param":{},"protocol":"unlock","terminal":{"tenpo_id":"1337","tenpo_index":1337,"terminal_attrib":0,"terminal_id":"1C1B0D07CDBB"}}' | openssl enc -e -aes-128-cfb -K '30313233343536373839303132333435' -iv '30313233343536373839303132333435' | curl -X POST -H "Content-Type: application/octet-stream" --data-binary @- http://localhost/game | openssl enc -d -aes-128-cfb -K '30313233343536373839303132333435' -iv '30313233343536373839303132333435' > mine.xxd && xxd mine.xxd`
