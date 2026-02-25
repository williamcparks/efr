import fs from "node:fs";
import path from "node:path";

import { pki } from "node-forge";
import { parse } from "toml";
import { z } from "zod";

const RawConfigSchema = z.object({
  credentials: z.object({
    cert_der: z.string(),
    pkey_pem: z.string(),
  }),
  admin: z.object({
    email: z.string(),
    password: z.string(),
  }),
});

interface Config {
  credentials: {
    certDER: Uint8Array;
    pkeyPEM: pki.rsa.PrivateKey;
  };
  admin: {
    email: string;
    password: string;
  };
}

export const tryReadConfig = async (): Promise<Config> => {
  let cwd = process.cwd();
  const arg = process.argv[2];
  if (arg) {
    cwd = path.join(cwd, arg);
  }
  const configPath = path.join(cwd, "efr.toml");

  const tomlStr = await fs.promises.readFile(configPath, "utf-8");
  const parsed = RawConfigSchema.parse(parse(tomlStr));

  const certPath = path.join(cwd, parsed.credentials.cert_der);
  const certDER = await fs.promises.readFile(certPath);

  const pkeyPath = path.join(cwd, parsed.credentials.pkey_pem);
  const pkeyText = await fs.promises.readFile(pkeyPath, "utf-8");
  const pkeyPEM = pki.privateKeyFromPem(pkeyText);

  return {
    credentials: {
      certDER,
      pkeyPEM,
    },
    admin: parsed.admin,
  };
};
