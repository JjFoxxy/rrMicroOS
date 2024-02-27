use volatile_register::RW;

#[repr(C)]
struct UartRegs {
    rxtx: RW<u32>,
    txfull: RW<u32>,
    rxempty: RW<u32>,
    ev_status: RW<u32>,
    ev_pending: RW<u32>,
    ev_enable: RW<u32>,
    txempty: RW<u32>,
    rxfull: RW<u32>,
}

pub struct Uart {
    p: &'static mut UartRegs,
}

impl Uart {
    pub fn new() -> Uart {
        Uart {
            p: unsafe { &mut *(0xf0003000 as *mut UartRegs) },
        }
    }

    fn wait_tx_fifo(&self) {
        while self.p.txfull.read() != 0 {}
    }

    pub fn putc(&self, c: &u8) {
        self.wait_tx_fifo();
        unsafe { self.p.rxtx.write(*c as u32) };
    }

    pub fn puts(&self, string: &[u8]) {
        for c in string.iter() {
            self.putc(c);
        }
    }
}
