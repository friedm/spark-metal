use core::clone::Clone;
use rawstruct::RawStruct;

pub const PERIPH_BASE: usize = 0x40000000;
const AHB_PERIPH_BASE: usize = PERIPH_BASE + 0x20000;

const RCC_APB2ENR_AFIOEN: usize = 0x00000001;
const RCC_APB2ENR_IOAEN: usize = 0x00000004;
const RCC_APB2ENR_IOBEN: usize = 0x0000008;

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

pub fn enable() {
    let mut rcc = RCC::from_mem(RCC_BASE);
    rcc.apb2enr |= RCC_APB2ENR_IOAEN;
    rcc.apb2enr |= RCC_APB2ENR_IOBEN;
    rcc.apb2enr |= RCC_APB2ENR_AFIOEN;
    rcc.write(RCC_BASE);
}
