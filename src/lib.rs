use std::sync::RwLock;

pub trait B15f: Sized {
    fn new() -> Result<Self, &'static str>;

    fn read_dip_switch(&self) -> u8;

    fn digital_read_0(&self) -> u8;

    fn digital_read_1(&self) -> u8;

    fn digital_write_0(&mut self, value: u8);

    fn digital_write_1(&mut self, value: u8);

    fn set_register_ddra(&mut self, value: u8);

    fn set_register_porta(&mut self, value: u8);

    fn get_register_pina(&self) -> u8;
}

pub struct B15fDriver {
    inner: RwLock<*mut b15f_sys::B15F>,
}

impl B15f for B15fDriver {
    fn new() -> Result<Self, &'static str> {
        let mut error_code = b15f_sys::ConnectionError_None;
        let instance = unsafe { b15f_sys::tryGetInstance(&mut error_code) };

        if error_code == b15f_sys::ConnectionError_Err {
            // TODO print message from exception
            return Err("Caught exception");
        }
        if instance.is_null() {
            return Err("Instance is null");
        }

        Ok(Self {
            // TODO Handle exeptions, prob needs C++ wrapper
            // SAFETY: Inside RwLock, ...
            inner: RwLock::new(instance),
        })
    }

    fn read_dip_switch(&self) -> u8 {
        unsafe { b15f_sys::B15F_readDipSwitch(*self.inner.read().unwrap()) }
    }

    fn digital_read_0(&self) -> u8 {
        unsafe { b15f_sys::B15F_digitalRead0(*self.inner.read().unwrap()) }
    }

    fn digital_read_1(&self) -> u8 {
        unsafe { b15f_sys::B15F_digitalRead1(*self.inner.read().unwrap()) }
    }

    fn digital_write_0(&mut self, value: u8) {
        unsafe { b15f_sys::B15F_digitalWrite0(*self.inner.read().unwrap(), value) }
    }

    fn digital_write_1(&mut self, value: u8) {
        unsafe { b15f_sys::B15F_digitalWrite1(*self.inner.read().unwrap(), value) }
    }

    fn set_register_ddra(&mut self, value: u8) {
        unsafe { b15f_sys::B15F_setRegisterDDRA(*self.inner.read().unwrap(), value) }
    }

    fn set_register_porta(&mut self, value: u8) {
        unsafe { b15f_sys::B15F_setRegisterPORTA(*self.inner.read().unwrap(), value) }
    }

    fn get_register_pina(&self) -> u8 {
        unsafe { b15f_sys::B15F_getRegisterPINA(*self.inner.read().unwrap()) }
    }
}

pub struct B15fStud {
    dip_switch: u8,
    in0: u8,
    in1: u8,
    out0: u8,
    out1: u8,
    register: u8,
    register_mask: u8,
}

impl B15f for B15fStud {
    fn new() -> Result<Self, &'static str> {
        Ok(Self {
            dip_switch: 1,
            in0: 0,
            in1: 0,
            out0: 0,
            out1: 0,
            register: 0,
            register_mask: 0,
        })
    }

    fn read_dip_switch(&self) -> u8 {
        self.dip_switch
    }

    fn digital_read_0(&self) -> u8 {
        self.in0
    }
    fn digital_read_1(&self) -> u8 {
        self.in1
    }
    fn digital_write_0(&mut self, value: u8) {
        println!("out0: {value:08b}");
        self.out0 = value;
    }
    fn digital_write_1(&mut self, value: u8) {
        println!("out1: {value:08b}");
        self.out1 = value;
    }

    fn set_register_ddra(&mut self, value: u8) {
        self.register_mask = value;
    }

    fn set_register_porta(&mut self, value: u8) {
        self.register |= value & self.register_mask;
    }

    fn get_register_pina(&self) -> u8 {
        todo!()
    }
}
