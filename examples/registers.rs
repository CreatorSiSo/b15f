use b15f::B15fDriver;

fn main() -> Result<(), &'static str> {
    let mut driver = B15fDriver::new()?;
    driver.set_register_ddra(0x0f);

    loop {
        for i in 0x00..0x0f {
            driver.set_register_porta(i);    
        }
    }
}
