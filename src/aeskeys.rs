#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    key0: Key0,
    key1: Key1,
    key2: Key2,
    key3: Key3,
    key4: Key4,
    key5: Key5,
    key6: Key6,
    key7: Key7,
}
impl RegisterBlock {
    #[doc = "0x00 - AES Key 0."]
    #[inline(always)]
    pub const fn key0(&self) -> &Key0 {
        &self.key0
    }
    #[doc = "0x04 - AES Key 1."]
    #[inline(always)]
    pub const fn key1(&self) -> &Key1 {
        &self.key1
    }
    #[doc = "0x08 - AES Key 2."]
    #[inline(always)]
    pub const fn key2(&self) -> &Key2 {
        &self.key2
    }
    #[doc = "0x0c - AES Key 3."]
    #[inline(always)]
    pub const fn key3(&self) -> &Key3 {
        &self.key3
    }
    #[doc = "0x10 - AES Key 4."]
    #[inline(always)]
    pub const fn key4(&self) -> &Key4 {
        &self.key4
    }
    #[doc = "0x14 - AES Key 5."]
    #[inline(always)]
    pub const fn key5(&self) -> &Key5 {
        &self.key5
    }
    #[doc = "0x18 - AES Key 6."]
    #[inline(always)]
    pub const fn key6(&self) -> &Key6 {
        &self.key6
    }
    #[doc = "0x1c - AES Key 7."]
    #[inline(always)]
    pub const fn key7(&self) -> &Key7 {
        &self.key7
    }
}
#[doc = "KEY0 (rw) register accessor: AES Key 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key0`]
module"]
#[doc(alias = "KEY0")]
pub type Key0 = crate::Reg<key0::Key0Spec>;
#[doc = "AES Key 0."]
pub mod key0;
#[doc = "KEY1 (rw) register accessor: AES Key 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key1`]
module"]
#[doc(alias = "KEY1")]
pub type Key1 = crate::Reg<key1::Key1Spec>;
#[doc = "AES Key 1."]
pub mod key1;
#[doc = "KEY2 (rw) register accessor: AES Key 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key2`]
module"]
#[doc(alias = "KEY2")]
pub type Key2 = crate::Reg<key2::Key2Spec>;
#[doc = "AES Key 2."]
pub mod key2;
#[doc = "KEY3 (rw) register accessor: AES Key 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key3`]
module"]
#[doc(alias = "KEY3")]
pub type Key3 = crate::Reg<key3::Key3Spec>;
#[doc = "AES Key 3."]
pub mod key3;
#[doc = "KEY4 (rw) register accessor: AES Key 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`key4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key4`]
module"]
#[doc(alias = "KEY4")]
pub type Key4 = crate::Reg<key4::Key4Spec>;
#[doc = "AES Key 4."]
pub mod key4;
#[doc = "KEY5 (rw) register accessor: AES Key 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`key5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key5`]
module"]
#[doc(alias = "KEY5")]
pub type Key5 = crate::Reg<key5::Key5Spec>;
#[doc = "AES Key 5."]
pub mod key5;
#[doc = "KEY6 (rw) register accessor: AES Key 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`key6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key6`]
module"]
#[doc(alias = "KEY6")]
pub type Key6 = crate::Reg<key6::Key6Spec>;
#[doc = "AES Key 6."]
pub mod key6;
#[doc = "KEY7 (rw) register accessor: AES Key 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`key7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key7`]
module"]
#[doc(alias = "KEY7")]
pub type Key7 = crate::Reg<key7::Key7Spec>;
#[doc = "AES Key 7."]
pub mod key7;
