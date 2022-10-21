use regex::Regex;
use std::{
    cmp::Ordering,
    io::{Error, ErrorKind},
};

#[derive(Debug)]
struct Cidr {
    addr: [u8; 4],
    mask: u8,
}

impl Cidr {
    fn new(cidr_str: &str) -> Result<Cidr, Error> {
        let re = Regex::new(r"^([0-9]{1,3}\.){3}[0-9]{1,3}(/([0-9]|[1-2][0-9]|3[0-2]))?$").unwrap();
        if !re.is_match(cidr_str) {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid cidr"));
        }
        let addr_and_mask: Vec<&str> = cidr_str.split('/').collect();
        let addr_vec: Vec<u8> = addr_and_mask[0]
            .split('.')
            .map(|s: &str| s.parse::<u8>().unwrap())
            .collect();
        Ok(Cidr {
            addr: [addr_vec[0], addr_vec[1], addr_vec[2], addr_vec[3]],
            mask: addr_and_mask[1].parse::<u8>().unwrap(),
        })
    }
}

impl std::fmt::Display for Cidr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}/{}",
            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.mask
        )
    }
}

fn main() {
    let cidr = read_cidr_from_param();
    let complement = complement_cidr(cidr);
    for value in complement {
        println!("{}", value)
    }
}

fn read_cidr_from_param() -> Cidr {
    let param = std::env::args()
        .nth(1)
        .expect("expected cidr, no parameter given");
    match Cidr::new(param.as_str()) {
        Ok(cidr) => cidr,
        Err(error) => {
            panic!("{}", error)
        }
    }
}

fn u8_to_bin(n: u8) -> String {
    let bin_str = &format!("{:#010b}", n);
    String::from(bin_str.split('b').last().unwrap())
}

fn bin_to_u8(b: &str) -> u8 {
    let intval = isize::from_str_radix(b, 2).unwrap();
    intval as u8
}

fn complement_cidr(cidr: Cidr) -> Vec<Cidr> {
    let mut out: Vec<Cidr> = Vec::new();
    for n in 0..cidr.mask {
        let addr_index_to_change = (n / 8) as usize;
        let addr_value_to_change = cidr.addr[addr_index_to_change];
        let addr_binstr_to_change = u8_to_bin(addr_value_to_change);
        let str_index_to_change = (n % 8) as usize;
        let changed_bit = if addr_binstr_to_change
            .chars()
            .nth(str_index_to_change)
            .unwrap()
            == '1'
        {
            "0"
        } else {
            "1"
        };

        let changed_addr = &format!(
            "{}{}{}",
            &addr_binstr_to_change[..(str_index_to_change)],
            changed_bit,
            "0".repeat(7 - str_index_to_change)
        );

        let mut converted_add_vec: Vec<u8> = Vec::new();
        for i in 0..4 {
            let addr = match i.cmp(&addr_index_to_change) {
                Ordering::Less => cidr.addr[i],
                Ordering::Equal => bin_to_u8(changed_addr),
                Ordering::Greater => bin_to_u8("00000000"),
            };
            converted_add_vec.push(addr)
        }

        let converted_cidr = Cidr {
            mask: n + 1,
            addr: [
                converted_add_vec[0],
                converted_add_vec[1],
                converted_add_vec[2],
                converted_add_vec[3],
            ],
        };
        out.push(converted_cidr);
    }
    out
}
