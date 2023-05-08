use x86_64::instructions::port::Port;
use bitflags::bitflags;

bitflags! {
    pub struct ControllerConfigFlags: u8 {
        const ENABLE_KEYBOARD_INTERRUPT = 0b00000001;
        const ENABLE_MOUSE_INTERRUPT = 0b00000010;
        const SET_SYSTEM_FLAG = 0b00000100;
        const DISABLE_KEYBOARD = 0b00010000;
        const DISABLE_MOUSE = 0b00100000;
        const ENABLE_TRANSLATE = 0b01000000;
    }
}

bitflags! {
    pub struct ControllerStatusFlags: u8 {
        const OUTPUT_FULL = 0b00000001;
        const INPUT_FULL = 0b00000010;
        const SYSTEM_FLAG = 0b00000100;
        const INPUT_IS_COMMAND = 0b00001000;
        const KEYBOARD_LOCK = 0b00010000;
        const MOUSE_OUTPUT_FULL = 0b00100000;
        const TIMEOUT_ERR = 0b01000000;
        const PARITY_ERR = 0b10000000;
    }
}

const READ_INTERNAL_RAM: u8 = 0x20;
const WRITE_INTERNAL_RAM: u8 = 0x60;

#[derive(Debug)]
pub enum ControllerError {
    Timeout
}

pub struct Controller {
    command_register: Port<u8>,
    data_register: Port<u8>,
    timeout: usize
}

impl Controller {
    pub const unsafe fn new() -> Self {
        Self {
            command_register: Port::new(0x64),
            data_register: Port::new(0x60),
            timeout: 10000,
        }
    }
    
    pub fn read_config(&mut self) -> Result<ControllerConfigFlags, ControllerError> {
        Ok(ControllerConfigFlags::from_bits_truncate(self.read_internal_ram(0)?))
    }
    
    pub fn write_config(&mut self, config: ControllerConfigFlags) -> Result<(), ControllerError> {
        self.write_internal_ram(0, config.bits())
    }
    
    pub fn read_internal_ram(&mut self, byte_number: u8) -> Result<u8, ControllerError> {
        let command = READ_INTERNAL_RAM as u8 | byte_number & 0x1f;
        self.wait_for_write()?;
        unsafe { self.command_register.write(command as u8); }
        self.read_data()
    }
    
    pub fn write_internal_ram(&mut self, byte_number: u8, data: u8) -> Result<(), ControllerError> {
        let command = WRITE_INTERNAL_RAM as u8 | byte_number & 0x1f;
        self.wait_for_write()?;
        unsafe { self.command_register.write(command as u8); }
        self.write_data(data)
    }
    
    fn wait_for_read(&mut self) -> Result<(), ControllerError> {
        let mut cycles = 0;
        while cycles < self.timeout {
            if self.read_status().contains(ControllerStatusFlags::OUTPUT_FULL) {
                return Ok(());
            }
            cycles += 1;
        }
        Err(ControllerError::Timeout)
    }
    
    fn wait_for_write(&mut self) -> Result<(), ControllerError> {
        let mut cycles = 0;
        while cycles < self.timeout {
            if !self.read_status().contains(ControllerStatusFlags::INPUT_FULL) {
                return Ok(());
            }
            cycles += 1;
        }
        Err(ControllerError::Timeout)
    }
    
    pub fn read_status(&mut self) -> ControllerStatusFlags {
        ControllerStatusFlags::from_bits_truncate(unsafe { self.command_register.read() })
    }
    
    pub fn read_data(&mut self) -> Result<u8, ControllerError> {
        self.wait_for_read()?;
        Ok(unsafe { self.data_register.read() })
    }
    
    pub fn write_data(&mut self, data: u8) -> Result<(), ControllerError> {
        self.wait_for_write()?;
        unsafe { self.data_register.write(data) };
        Ok(())
    }
}