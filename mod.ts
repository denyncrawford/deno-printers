import { dlopen } from "jsr:@denosaurs/plug";
import { Printer } from "./types.ts";

const { symbols } = await dlopen(
  {
    name: "printer_bindings",
    url: "/Users/genesissaraicarvajalsiverio/Projects/xirect/print/printer-bindings/target/debug/",
  },
  
  {
    get_printer_by_name: {
      parameters: ["buffer"],
      result: "pointer",
      nonblocking: false,
    },
    get_printers: {
      parameters: [],
      result: "pointer",
      nonblocking: false,
    },
    print_file: {
      parameters: [
        'buffer',
        'buffer',
        'buffer',
      ],
      result: 'bool',
      nonblocking: false
    },
    print: {
      parameters: [
        'buffer',
        'buffer',
        'buffer',
      ],
      result: 'bool',
      nonblocking: false
    },
  }
);

export function getPrinters(): Printer[] {
  const pointer = symbols.get_printers();
  return JSON.parse(new Deno.UnsafePointerView(pointer!).getCString());
}

export function getPrinterByName(name: string): Printer {
  const pointer = symbols.get_printer_by_name(new TextEncoder().encode(name));
  return JSON.parse(new Deno.UnsafePointerView(pointer!).getCString());
}

export function print(printer: Printer, text: string, jobName?: string): boolean {
  const encoder = new TextEncoder();
  const pointer = symbols.print(
    encoder.encode(printer.name),
    encoder.encode(text),
    jobName ? encoder.encode(jobName) : null,
  );
  return pointer
}

export function printFile(printer: Printer, file: string, jobName?: string): boolean {
  const encoder = new TextEncoder();
  const pointer = symbols.print_file(
    encoder.encode(printer.name),
    encoder.encode(file),
    jobName ? encoder.encode(jobName) : null,
  );
  return pointer
}