enum RAM {
    _64GB,
    _32GB,
    _16GB,
    _8GB,
}

enum SSD {
    _8T,
    _4T,
    _2T,
    _1T,
    _500GB,
    _250GB,
}

enum HDD {
    _22T,
    _20T,
    _18T,
    _16T,
    _14T,
    _12T,
    _10T,
    _8T,
    _6T,
    _5T,
    _4T,
    _3T,
    _2T,
    _1T,
}

type Hardware = (String, u32, u32);
type HardwareRAM = (String, u32, u32, RAM);
type HardwareSSD = (String, u32, u32, SSD);
type HardwareHDD = (String, u32, u32, HDD);

struct CustomPC {
    cpu: Hardware,
    mainboard: Hardware,
    ram: HardwareRAM,
    vga: Option<Hardware>,
    ssd: HardwareSSD,
    case: Hardware,
    power: Hardware,
    cooler: Option<Hardware>,
    hdd: Option<HardwareHDD>,
}

impl CustomPC {
    fn price_total(self) -> u32 {
        let mut total = 0u32;

        total += self.cpu.1 * self.cpu.2;
        total += self.mainboard.1 * self.mainboard.2;
        total += self.ram.1 * self.ram.2;
        total += match self.vga {
            Some((_, price, quantity)) => price * quantity,
            None => 0u32, 
        };
        total += self.ssd.1 * self.ssd.2;
        total += match self.cooler {
            Some((_, price, quantity)) => price * quantity,
            None => 0u32, 
        };
        total += match self.hdd {
            Some((_, price, quantity, _)) => price * quantity,
            None => 0u32, 
        };

        total
    }
}

fn work() {
    let pc = CustomPC {
        cpu: (String::from("AMD RYZEN 5 7500F"), 227970, 1),
        mainboard: (String::from("ASUS TUF Gaming B650-PLUS"), 297780, 1),
        ram: (String::from("Micron Crucial DDR5-4800 CL40"), 24090, 2, RAM::_8GB),
        vga: Some((String::from("RTX 4060 STORM X Dual OC D6 8GB "), 438590, 1)),
        ssd: (String::from("SAMSUNG 970 EVO Plus M.2 NVMe"), 80370, 1, SSD::_1T),
        case: (String::from("ABKO NCORE G30 TrueForce"), 47010, 1),
        power: (String::from("Micronics Classic II Full Change 600W 80PLUS BRONZE 230V EU"), 62500, 1),
        cooler: None,
        hdd: None,
    };

    println!("total: {}", pc.price_total());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        work();
    }
}
