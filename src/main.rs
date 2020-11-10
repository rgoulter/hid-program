extern crate hidapi;

use hidapi::{DeviceInfo,HidApi};

fn is_bm40rgb_raw_hid(deviceInfo: &DeviceInfo) -> bool {
    deviceInfo.vendor_id() == 0xFEED &&
        deviceInfo.product_id() == 0x6060 &&
        deviceInfo.usage_page() == 0xFF60 &&
        deviceInfo.usage() == 0x61
}

fn main() {
    println!("Printing all available hid devices:");

    match HidApi::new() {
        Ok(api) => {
            for device_info in api.device_list() {
                if is_bm40rgb_raw_hid(device_info) {
                    println!("{:04x}:{:04x} {:?} {:?}",
                             device_info.vendor_id(),
                             device_info.product_id(),
                             device_info.manufacturer_string(),
                             device_info.product_string());

                    match device_info.open_device(&api) {
                        Ok(device) => {
                            // 32 is RAW_EPSIZE
                            // let mut data: [u8; 32] = [0; 32].enumerate().map(|idx, v| 120 + idx);
                            let mut data: [u8; 32] = [120; 32];
                            device.write(&data);

                            let mut buf: [u8; 32] = [0; 32];
                            device.read(&mut buf);
                            println!("read data: {:?}", buf)
                        },
                        Err(e) => {
                            eprintln!("Error opening device: {}", e);
                        },
                    }
                }
                // println!("{:04x}:{:04x}", device.vendor_id(), device.product_id());
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        },
    }
}
