use std::{time::Duration, io::{BufReader, BufRead, Write, ErrorKind}, error,fmt};
use serialport::{available_ports, SerialPortType::UsbPort};

const BAUD_RATE: u32 = 9600;
const VID: u16 = 1240;
const PID: u16 = 0x000a;

#[derive(Debug)]
struct HolterNotFound;
impl fmt::Display for HolterNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No se encuentra el dispositivo HeartSense ML, revise las conexiones. Por más ayuda, vaya a la sección 'Ayuda'")
    }
}
impl error::Error for HolterNotFound{}

pub fn get_graph() -> Result<Vec<String>,Box<dyn error::Error>> {
    let mut s_port = None;
    for port in available_ports().unwrap() {
        //println!("{:?}",port);
        match port.port_type {
            UsbPort(p_info) => {
                if p_info.vid == VID && p_info.pid == PID {
                    s_port = Some(port.port_name);
                }
            },
            _ => {},
        }
    }

    let mut serial_port = serialport::new(s_port.ok_or(HolterNotFound)?/*.expect("HEARTSENSE - PIC NOT FOUND")*/, BAUD_RATE)
        .timeout(Duration::from_secs(2))
        .open()?;
        //.expect("Failed to open serial port");

    let mut reader = BufReader::new(serial_port.try_clone().unwrap());
    let mut ret: Vec<String> = Vec::new();

    serial_port.write(&[50])?;//.expect("Write Failed");

    loop {
        let mut my_str = String::new();
        match reader.read_line(&mut my_str) {
            Ok(_) => { ret.push(my_str); },
            Err(e) => {
                if e.kind() == ErrorKind::TimedOut {
                    break;
                }
            },
        }
    }

    Ok(ret)
    
}
