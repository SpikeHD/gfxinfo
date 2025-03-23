pub fn vendor_id_to_name(vendor_id: u16) -> &'static str {
  match vendor_id {
    0x1002 => "AMD",
    0x10DE => "Nvidia",
    0x8086 => "Intel",
    0x106B => "Apple",
    _ => "Unknown",
  }
}