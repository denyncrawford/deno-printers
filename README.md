# Native Printer Bindings for Deno
This is a Deno module that provides native printer bindings for Deno.

[![JSR](https://jsr.io/badges/@denyncrawford/printers)](https://jsr.io/@<scope>/<package>)

## Installation

```sh
deno add @denyncrawford/printers
```

> Note: This package requires the `--unstable-ffi` flag to be set.

## Usage

```ts
import { getPrinters, getPrinterByName, print, printFile } from "jsr:@denyncrawford/printers";

const printers = getPrinters();
const printer = getPrinterByName("HP OfficeJet Pro 8600");

print(printer, "Hello World!");
printFile(printer, "/path/to/file.txt");
```

## Building

To build your own lib binary, run the following command:

```sh
cargo build --release --lib
```