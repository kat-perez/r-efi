//! Universal Serial Bus Definitions

#[derive(Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct DeviceRequest {
    pub request_type: u8,
    pub request: u8,
    pub value: u16,
    pub index: u16,
    pub length: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct DeviceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub bcd_usb: u16,
    pub device_class: u8,
    pub device_sub_class: u8,
    pub device_protocol: u8,
    pub max_packet_size0: u8,
    pub id_vendor: u16,
    pub id_product: u16,
    pub bcd_device: u16,
    pub str_manufacturer: u8,
    pub str_product: u8,
    pub str_serial_number: u8,
    pub num_configurations: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct ConfigDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub total_length: u16,
    pub num_interfaces: u8,
    pub configuration_value: u8,
    pub configuration: u8,
    pub attributes: u8,
    pub max_power: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct InterfaceDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub interface_number: u8,
    pub alternate_setting: u8,
    pub num_endpoints: u8,
    pub interface_class: u8,
    pub interface_sub_class: u8,
    pub interface_protocol: u8,
    pub interface: u8,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct EndpointDescriptor {
    pub length: u8,
    pub descriptor_type: u8,
    pub endpoint_address: u8,
    pub attributes: u8,
    pub max_packet_size: u16,
    pub interval: u8,
}

#[cfg(test)]
mod test {
    use core::mem;
    use super::*;

    // USB device requests and descriptors are standard USB wire-format
    // structures, so verify that the layouts are byte-packed.
    #[test]
    fn layout() {
        assert_eq!(mem::align_of::<DeviceRequest>(), 1);
        assert_eq!(mem::size_of::<DeviceRequest>(), 8);

        assert_eq!(mem::align_of::<DeviceDescriptor>(), 1);
        assert_eq!(mem::size_of::<DeviceDescriptor>(), 18);

        assert_eq!(mem::align_of::<ConfigDescriptor>(), 1);
        assert_eq!(mem::size_of::<ConfigDescriptor>(), 9);

        assert_eq!(mem::align_of::<InterfaceDescriptor>(), 1);
        assert_eq!(mem::size_of::<InterfaceDescriptor>(), 9);

        assert_eq!(mem::align_of::<EndpointDescriptor>(), 1);
        assert_eq!(mem::size_of::<EndpointDescriptor>(), 7);
    }
}
