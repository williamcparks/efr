# Typescript Client

Currently proof of concept showing how to sign a message (just auth user) and send it to the efm
once the rust impl is complete i will return and add further implentations to make the client complete

Rough sketch plan

- src/main.ts -> dist/main.js

Works like efr-cli

- src/[other] -> dist/[other]

Works like efr library crate

## Setup

install deps

```sh
npm i
```

run with env folder, (look at cli.md in root of repo for setup)

```sh
node ./dist/main [PATH_TO_ENV_FOLDER]
```
