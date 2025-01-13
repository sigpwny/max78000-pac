#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg0: Reg0,
    reg1: Reg1,
    reg2: Reg2,
    reg3: Reg3,
}
impl RegisterBlock {
    #[doc = "0x00 - Register 0."]
    #[inline(always)]
    pub const fn reg0(&self) -> &Reg0 {
        &self.reg0
    }
    #[doc = "0x04 - Register 1."]
    #[inline(always)]
    pub const fn reg1(&self) -> &Reg1 {
        &self.reg1
    }
    #[doc = "0x08 - Register 2."]
    #[inline(always)]
    pub const fn reg2(&self) -> &Reg2 {
        &self.reg2
    }
    #[doc = "0x0c - Register 3."]
    #[inline(always)]
    pub const fn reg3(&self) -> &Reg3 {
        &self.reg3
    }
}
#[doc = "REG0 (rw) register accessor: Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg0`]
module"]
#[doc(alias = "REG0")]
pub type Reg0 = crate::Reg<reg0::Reg0Spec>;
#[doc = "Register 0."]
pub mod reg0;
#[doc = "REG1 (rw) register accessor: Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg1`]
module"]
#[doc(alias = "REG1")]
pub type Reg1 = crate::Reg<reg1::Reg1Spec>;
#[doc = "Register 1."]
pub mod reg1;
#[doc = "REG2 (rw) register accessor: Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg2`]
module"]
#[doc(alias = "REG2")]
pub type Reg2 = crate::Reg<reg2::Reg2Spec>;
#[doc = "Register 2."]
pub mod reg2;
#[doc = "REG3 (rw) register accessor: Register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`reg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg3`]
module"]
#[doc(alias = "REG3")]
pub type Reg3 = crate::Reg<reg3::Reg3Spec>;
#[doc = "Register 3."]
pub mod reg3;
