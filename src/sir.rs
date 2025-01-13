#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sistat: Sistat,
    addr: Addr,
    _reserved2: [u8; 0xf8],
    fstat: Fstat,
    sfstat: Sfstat,
}
impl RegisterBlock {
    #[doc = "0x00 - System Initialization Status Register."]
    #[inline(always)]
    pub const fn sistat(&self) -> &Sistat {
        &self.sistat
    }
    #[doc = "0x04 - Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x100 - funcstat register."]
    #[inline(always)]
    pub const fn fstat(&self) -> &Fstat {
        &self.fstat
    }
    #[doc = "0x104 - Security function status register."]
    #[inline(always)]
    pub const fn sfstat(&self) -> &Sfstat {
        &self.sfstat
    }
}
#[doc = "SISTAT (r) register accessor: System Initialization Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sistat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sistat`]
module"]
#[doc(alias = "SISTAT")]
pub type Sistat = crate::Reg<sistat::SistatSpec>;
#[doc = "System Initialization Status Register."]
pub mod sistat;
#[doc = "ADDR (r) register accessor: Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1).\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
pub mod addr;
#[doc = "FSTAT (r) register accessor: funcstat register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstat`]
module"]
#[doc(alias = "FSTAT")]
pub type Fstat = crate::Reg<fstat::FstatSpec>;
#[doc = "funcstat register."]
pub mod fstat;
#[doc = "SFSTAT (r) register accessor: Security function status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sfstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfstat`]
module"]
#[doc(alias = "SFSTAT")]
pub type Sfstat = crate::Reg<sfstat::SfstatSpec>;
#[doc = "Security function status register."]
pub mod sfstat;
