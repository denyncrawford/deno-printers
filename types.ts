
/**
 * A printer object returned by the getPrinters() function {@link getPrinters}.
 */
export interface Printer {
    name: string
    system_name: string
    driver_name: string
    uri: string
    location: string
    is_default: boolean
    is_shared: boolean
    state: string
  }