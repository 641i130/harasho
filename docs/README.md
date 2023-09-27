# Notes:
The json file is for postman.
---
The game starts (this includes NesysService traffic):
- POST /service/respone/respone.php
- GET /alive/3800/Alive.txt
- GET /server/certify.php
- GET /alive/i.php
- GET /server/FireAlert.php
- POST /basicinfo
    Encryted with Public Key
The next requests are then encrypted with the AES information given in basicinfo

    - Protocol Unlock
