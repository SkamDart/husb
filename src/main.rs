use clap::App;
use clap::Arg;
use hidapi::DeviceInfo;
use hidapi::HidApi;
use nix::sys::ioctl;
use std::fmt;

fn main() {
    let matches = App::new("husb")
        .about("USB Helper")
        .version("0.0.1")
        .author("Cameron Dart <cdart2@illinois.edu>")
        .subcommand(App::new("list").about("List all usb devices"))
        .subcommand(
            App::new("reset")
                .about("Reset a usb device by device path")
                .arg(
                    Arg::with_name("device_path")
                        .short("d")
                        .long("device_path")
                        .help("Device Path to reset")
                        .index(1)
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("list", _) => list_usb_devices(),
        ("reset", reset_matches) => {
            reset_usb_device(reset_matches.unwrap().value_of("device_path").unwrap())
        }
        _ => eprintln!("No command specified"),
    }
}

struct Device(DeviceInfo);

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // TODO Not sure the /dev/bus/usb should be hard coded
            "/dev/bus/usb/{}\n\tVendorId: {}\n\tProduct Id: {}\n\tSerial Number: {}\n\tManufacturer String: {}",
            self.0.path().to_str().unwrap_or("Unknown"),
            self.0.vendor_id(),
            self.0.product_id(),
            self.0.serial_number().unwrap_or("Unknown"),
            self.0.manufacturer_string().unwrap_or("Unknown")
        )
    }
}

fn list_usb_devices() {
    let hid = HidApi::new().expect("Failed to query USB device list");
    for dev_info in hid.device_list() {
        println!("{}\n", Device(dev_info.clone()));
    }
}

fn reset_usb_device(dev_path: &str) {
    println!("{}", dev_path);
    // rc = ioctl(fd, USBDEVFS_RESET, 0);
}
