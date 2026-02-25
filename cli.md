# Cli

> Setup instructions for how to use the cli

Useful for testing and making requests

## Env Folder

The /env folder can be used to contain credentials you don't want in the git repo

```sh
mkdir env
```

## cert.der

The _cert.der_ file can be created from a pfx file using _openssl_

```sh
openssl pkcs12 -in input.pfx -clcerts -nokeys | openssl x509 -outform der -out cert.der
```

you can place this cert.der inside /env

## pkey.pem

The _pkey.pem_ is the raw private key information extractable using _openssl_

```sh
openssl pkcs12 -in input.pfx -nocerts -nodes -out pkey.pem
```

you can place this pkey.pem inside /env

## efr.toml

The _efile rust_ toml file contains credential information for quick usage

```toml
[credentials]
# the location of the x509 certificate (relative to the efr.toml)
cert_der = "./cert.der"
# the location of the private key (relative to the efr.toml)
pkey_pem = "./pkey.pem"

[admin]
# the user email you plan to use with the cli
email = "some email here"
# the password of the user
password = "some password"
```

you can place this efr.toml file in /env

## Running the cli

With the /env folder now setup you can now run the cli via

```sh
cargo run --release --bin efr-cli -- env
```

here the argument **env** indicates you have stored your efr.toml in a sub folder called "env"

## other languages

assume you are in other-languages/typescript or python and your _env_ folder is in the root then:

```sh
node ./dist/main.js ../../env
```

or in python client

```sh
uv ./src/main.py ../../env
```

since you are "2" levels below the root ../../env
