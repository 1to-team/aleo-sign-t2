# aleo-sign-t2

Console tool for signing messages with a private key, designed for the Aleo Testnet 2 network's signature format.

```bash
Usage: aleo-sign-t2 --private-key <PRIVATE_KEY> --message <MESSAGE>

Options:
      --private-key <PRIVATE_KEY>  Your Testnet 2 PrivateKey
      --message <MESSAGE>          Message to sign
  -h, --help                       Print help
  -V, --version                    Print version
```

## Usage example

```
$ aleo-sign-t2 --private-key APrivateKey1zkp... --message "Hello World"

Message (11 bytes):
Hello World

Testnet 2 Address:
aleo1...

Testnet 2 Signature:
sign1ftal5ngunk4lv9hfygl45z35vqu9cufqlecumke9jety3w2s6vqtjj4hmjulh899zqsxfxk9wm8q40w9zd9v63sqevkz8zaddugwwq35q8nghcp83tgntvyuqgk8yh0temt6gdqpleee0nwnccxfzes6pawcdwyk4f70n9ecmz6675kvrfsruehe27ppdsxrp2jnvcmy2wws6sw0egv

```
