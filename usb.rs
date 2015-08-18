const USB_BASE: usize = 0x4000_5C00;
const USB_BUFFER_BASE: usize = 0x4000_6000;

///CDC/ACM Protocol
///Two bulk endpoints


///activate register macrocell clock
///de-assert macrocell specific reset signal (set FRES in CNTR?)
///enable analog by setting PDWN bit in CNTR reg
///wait startup time
///remove reset condition by clearing FRES in CNTR
///clear ISTR register
///initialize control registers (enabled interrupts, chosen address of packet buffers)
///initialize packet buffer description table
///handle reset interrupt, and change LED color
pub fn init() {

}

///write to ADDRn_TX/ADDRn_RX
///set EP_TYPE in USB_EPnR reg
///set STAT_TX in USB_EPnR reg
///set COUNTn_TX
///set STAT_RX, COUNTn_RX (BL_SIZE, NUM_BLOCKS)
///for isocronous
///set EP_TYPE bits in USB_EPnR
///set STAT_RX/STAT_TX to 11
pub fn init_endpoint() {
}
