import requests
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import rsa, padding
from cryptography.hazmat.primitives import hashes

# URL to send the POST request to
url = 'http://10.3.0.141/basicinfo'

# Send the POST request
response = requests.post(url, verify=False)

if response.status_code == 200:
    # Extract the encrypted data from the response
    encrypted_data = response.content

    # Load the private key from the PEM file
    private_key_file = 'private_key.pem'  # Replace with your private key file
    with open(private_key_file, 'rb') as key_file:
        private_key = serialization.load_pem_private_key(
            key_file.read(),
            password=None,  # If your key is password-protected, provide the password here
            backend=default_backend()
        )

    # Decrypt the data with PKCS#1 v1.5 padding
    decrypted_data = private_key.decrypt(
        encrypted_data,
        padding.PKCS1v15()
    )

    # Assuming the decrypted data is a UTF-8 encoded string, you can print it
    print("Decrypted Data:", decrypted_data.decode('utf-8'))
else:
    print("Failed to retrieve data. Status code:", response.status_code)

"""
import requests
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives import padding
from cryptography.hazmat.primitives.asymmetric import rsa

# URL to send the POST request to
url = 'http://10.3.0.141/basicinfo'

# Send the POST request
response = requests.post(url, verify=False)

if response.status_code == 200:
    # Extract the encrypted data from the response
    encrypted_data = response.content

    # Load the private key from the PEM file
    private_key_file = 'private_key.pem'  # Replace with your private key file
    with open(private_key_file, 'rb') as key_file:
        private_key = serialization.load_pem_private_key(
            key_file.read(),
            password=None,  # If your key is password-protected, provide the password here
            backend=default_backend()
        )

    # Decrypt the data with no padding
    decrypted_data = private_key.decrypt(
        encrypted_data,
        padding.PKCS7(128)
        #padding.PKCS1v15()  # Use PKCS#1 v1.5 padding for no padding
    )

    # Assuming the decrypted data is a UTF-8 encoded string, you can print it
    print("Decrypted Data:", decrypted_data.decode('utf-8'))
else:
    print("Failed to retrieve data. Status code:", response.status_code)

"""
