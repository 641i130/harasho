import sys

def pkcs7_padding(data):
    block_size = 16  # AES block size in bytes
    padding_required = block_size - (len(data) % block_size)
    padding = chr(padding_required).encode() * padding_required
    return str.encode(data) + padding

plaintext = str(sys.argv[1])

# Apply PKCS7 padding
padded_plaintext = pkcs7_padding(plaintext)

# Output the padded plaintext to stdout so it can be piped into OpenSSL
sys.stdout.buffer.write(padded_plaintext)
