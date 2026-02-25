import tomllib
import sys
import os

from dataclasses import dataclass
from cryptography.hazmat.primitives.asymmetric.rsa import RSAPrivateKey
from cryptography.hazmat.primitives.serialization import load_pem_private_key

@dataclass
class RawCredentials:
    cert_der: bytes
    pkey_pem: str

@dataclass
class Credentials:
    cert_der: bytes
    pkey_pem: RSAPrivateKey

@dataclass
class Admin:
    email: str
    password: str

@dataclass
class RawConfig:
    credentials: RawCredentials
    admin: Admin

@dataclass
class Config:
    credentials: Credentials
    admin: Admin

    @staticmethod
    def try_new() -> "Config":
        cwd = os.getcwd()
        if len(sys.argv) >= 2:
            cwd = os.path.join(cwd, sys.argv[1])
        toml_path = os.path.join(cwd, "efr.toml")

        with open(toml_path, 'rb') as f:
            data = tomllib.load(f)

        raw = Config(
            credentials=Credentials(**data["credentials"]),
            admin=Admin(**data["admin"]),
        )

        cert_der_path = os.path.join(cwd, raw.credentials.cert_der)
        pkey_pem_path = os.path.join(cwd, raw.credentials.pkey_pem)

        with open(cert_der_path, 'rb') as f:
            cert_der = f.read()

        with open(pkey_pem_path, 'rb') as f:
            pkey_pem = load_pem_private_key(f.read(), password=None)

        creds = Credentials(
            cert_der=cert_der,
            pkey_pem=pkey_pem
        )

        return Config(
            credentials=creds,
            admin=raw.admin
        )