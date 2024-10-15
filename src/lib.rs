use std::ffi::{CStr, CString};

use deno_bindgen::deno_bindgen;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

// #[derive(Serialize)]
pub struct PrinterWrapper<'a> {
    pub printer: &'a printers::printer::Printer,
}

fn printer_state_to_string(state: &printers::printer::PrinterState) -> &'static str {
    match state {
        printers::printer::PrinterState::READY => "READY",
        printers::printer::PrinterState::PAUSED => "PAUSED",
        printers::printer::PrinterState::PRINTING => "PRINTING",
        printers::printer::PrinterState::UNKNOWN => "UNKNOWN",
    }
}

impl<'a> Serialize for PrinterWrapper<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let state_value = printer_state_to_string(&self.printer.state);
        let mut printer_json = serializer.serialize_struct("Printer", 9)?;
        printer_json.serialize_field("name", &self.printer.name)?;
        printer_json.serialize_field("system_name", &self.printer.system_name)?;
        printer_json.serialize_field("driver_name", &self.printer.driver_name)?;
        printer_json.serialize_field("uri", &self.printer.uri)?;
        printer_json.serialize_field("location", &self.printer.location)?;
        printer_json.serialize_field("is_default", &self.printer.is_default)?;
        printer_json.serialize_field("is_shared", &self.printer.is_shared)?;
        printer_json.serialize_field("state", &state_value)?;
        printer_json.end()
    }
}

#[deno_bindgen]
fn get_printer_by_name(name: *mut i8) -> *mut i8 {
    unsafe {
        match CStr::from_ptr(name).to_str() {
            Ok(name_str) => match printers::get_printer_by_name(name_str) {
                Some(printer) => {
                    let printer_wrapper = PrinterWrapper { printer: &printer };
                    match serde_json::to_string(&printer_wrapper) {
                        Ok(json_str) => CString::new(json_str).unwrap().into_raw(),
                        Err(_) => std::ptr::null_mut(),
                    }
                }
                None => std::ptr::null_mut(),
            },
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[deno_bindgen]
fn get_printers() -> *mut i8 {
    let printers = printers::get_printers();
    let printer_wrappers = printers
        .iter()
        .map(|printer| PrinterWrapper { printer: &printer })
        .collect::<Vec<PrinterWrapper>>();

    // let printers_json = serde_json::to_string(&printer_wrappers).unwrap();

    let printers_json = CString::new(serde_json::to_string(&printer_wrappers).unwrap())
        .unwrap()
        .into_raw();

    printers_json
}

#[deno_bindgen]
fn print(printer: *mut i8, text: *mut i8, job_name: *mut i8) -> bool {
    unsafe {
        let printer_str = match CStr::from_ptr(printer).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let text_str = match CStr::from_ptr(text).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let job_name_str = if !job_name.is_null() {
            match CStr::from_ptr(job_name).to_str() {
                Ok(s) => Some(s),
                Err(_) => return false,
            }
        } else {
            None
        };

        if let Some(printer) = printers::get_printer_by_name(printer_str) {
            printer
                .print(text_str.as_bytes(), job_name_str)
                .unwrap_or(false)
        } else {
            false
        }
    }
}

#[deno_bindgen]
fn print_file(printer: *mut i8, file: *mut i8, job_name: *mut i8) -> bool {
    unsafe {
        let printer_str = match CStr::from_ptr(printer).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let file_str = match CStr::from_ptr(file).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        let job_name_str = if !job_name.is_null() {
            match CStr::from_ptr(job_name).to_str() {
                Ok(s) => Some(s),
                Err(_) => return false,
            }
        } else {
            None
        };

        if let Some(printer) = printers::get_printer_by_name(printer_str) {
            printer.print_file(file_str, job_name_str).unwrap_or(false)
        } else {
            false
        }
    }
}
