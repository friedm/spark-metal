use core::clone::Clone;
use rawstruct::RawStruct;

pub const PERIPH_BASE: usize = 0x40000000;
const AHB_PERIPH_BASE: usize = PERIPH_BASE + 0x20000;

const RCC_APB2ENR_AFIOEN: usize = 0x00000001;
const RCC_APB2ENR_IOAEN: usize = 0x00000004;
const RCC_APB2ENR_IOBEN: usize = 0x0000008;

const RCC_BASE: usize = AHB_PERIPH_BASE + 0x1000;

const FLASH_ACR_PRFTBE: usize = 0x10;
const FLASH_ACR_LATENCY: usize = 0x03;
const FLASH_ACR_LATENCY_2: usize = 0x02;

#[derive(Clone)]
struct RCC {
    CR: usize,
    CFGR: usize,
    CIR: usize,
    APB2RSTR: usize,
    APB1RSTR: usize,
    AHBENR: usize,
    APB2ENR: usize,
    APB1ENR: usize,
    BDCR: usize,
    CSR: usize
}

impl RawStruct<RCC> for RCC {
    fn to_struct(&self) -> RCC {
        (*self).clone()
    }
}

pub fn enable() {
    let mut rcc = RCC::from_mem(RCC_BASE);
    rcc.APB2ENR |= RCC_APB2ENR_IOAEN;
    rcc.APB2ENR |= RCC_APB2ENR_IOBEN;
    rcc.APB2ENR |= RCC_APB2ENR_AFIOEN;
    rcc.write(RCC_BASE);
}
