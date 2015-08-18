use core::clone::Clone;
use rawstruct::RawStruct;

pub const PERIPH_BASE: usize = 0x4000_0000;
const AHB_PERIPH_BASE: usize = PERIPH_BASE + 0x20000;

const RCC_APB2ENR_AFIOEN: usize = 0x0000_0001;
const RCC_APB2ENR_IOAEN: usize = 0x0000_0004;
const RCC_APB2ENR_IOBEN: usize = 0x0000_0008;

const RCC_APB1ENR_USBEN: usize = 0x0080_0000;

const RCC_AHBENR_CRCEN: usize = 0x0040;

const RCC_BDCR_RTCEN: usize = 0x0000_8000;

const RCC_BASE: usize = AHB_PERIPH_BASE + 0x1000;

#[derive(Clone)]
struct RCC {
    cr: usize,
    cfgr: usize,
    cir: usize,
    apb2rstr: usize,
    apb1rstr: usize,
    ahbenr: usize,
    apb2enr: usize,
    apb1enr: usize,
    bdcr: usize,
    csr: usize
}

impl RawStruct<RCC> for RCC {
    fn to_struct(&self) -> RCC {
        (*self).clone()
    }
}

pub fn enable_apb2() {
    let mut rcc = RCC::from_mem(RCC_BASE);
    rcc.apb2enr |= RCC_APB2ENR_IOAEN;
    rcc.apb2enr |= RCC_APB2ENR_IOBEN;
    rcc.apb2enr |= RCC_APB2ENR_AFIOEN;
    rcc.write(RCC_BASE);
}

pub fn enable_apb1() {
    let mut rcc = RCC::from_mem(RCC_BASE);
    rcc.apb1enr |= RCC_APB1ENR_USBEN;
    rcc.write(RCC_BASE);
}

pub fn enable_ahb() {
    let mut rcc = RCC::from_mem(RCC_BASE);
    rcc.ahbenr |= RCC_AHBENR_CRCEN;
    rcc.write(RCC_BASE);
}

pub fn enable_rtc() {
    let mut rcc = RCC::from_mem(RCC_BASE);
    rcc.bdcr |= RCC_BDCR_RTCEN;
    rcc.write(RCC_BASE);
}
