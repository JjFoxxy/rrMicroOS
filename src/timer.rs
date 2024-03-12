use volatile_register::RW;

#[repr(C)]
struct TimerRegs {
    load: RW<u32>,
    reload: RW<u32>,
    en: RW<u32>,
    update_value: RW<u32>,
    value: RW<u32>,
    ev_status: RW<u32>,
    ev_pending: RW<u32>,
    ev_enable: RW<u32>,
}

pub struct Timer {
    p: &'static mut TimerRegs,
}

impl Timer {
    pub fn new(base: u32) -> Timer {
        Timer {
            p: unsafe { &mut *(base as *mut TimerRegs) },
        }
    }

    pub fn set_load(&self, load: u32) {
        unsafe { self.p.load.write(load) };
    }

    pub fn set_reload(&self, reload: u32) {
        unsafe { self.p.reload.write(reload) };
    }

    pub fn enable(&self) {
        unsafe { self.p.en.write(1) };
    }

    pub fn disable(&self) {
        unsafe { self.p.en.write(0) };
    }

    pub fn set_oneshot(&self, duration: u32) {
        self.disable();
        self.set_load(duration);
        self.enable();
    }

    pub fn set_periodic(&self, period: u32) {
        self.disable();
        self.set_load(0);
        self.set_reload(period);
        self.enable();
    }
}
