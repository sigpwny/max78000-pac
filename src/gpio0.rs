#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    en0: En0,
    en0_set: En0Set,
    en0_clr: En0Clr,
    outen: Outen,
    outen_set: OutenSet,
    outen_clr: OutenClr,
    out: Out,
    out_set: OutSet,
    out_clr: OutClr,
    in_: In,
    intmode: Intmode,
    intpol: Intpol,
    inen: Inen,
    inten: Inten,
    inten_set: IntenSet,
    inten_clr: IntenClr,
    intfl: Intfl,
    _reserved17: [u8; 0x04],
    intfl_clr: IntflClr,
    wken: Wken,
    wken_set: WkenSet,
    wken_clr: WkenClr,
    _reserved21: [u8; 0x04],
    dualedge: Dualedge,
    padctrl0: Padctrl0,
    padctrl1: Padctrl1,
    en1: En1,
    en1_set: En1Set,
    en1_clr: En1Clr,
    en2: En2,
    en2_set: En2Set,
    en2_clr: En2Clr,
    _reserved30: [u8; 0x28],
    hysen: Hysen,
    srsel: Srsel,
    ds0: Ds0,
    ds1: Ds1,
    ps: Ps,
    _reserved35: [u8; 0x04],
    vssel: Vssel,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port."]
    #[inline(always)]
    pub const fn en0(&self) -> &En0 {
        &self.en0
    }
    #[doc = "0x04 - GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn en0_set(&self) -> &En0Set {
        &self.en0_set
    }
    #[doc = "0x08 - GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn en0_clr(&self) -> &En0Clr {
        &self.en0_clr
    }
    #[doc = "0x0c - GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port."]
    #[inline(always)]
    pub const fn outen(&self) -> &Outen {
        &self.outen
    }
    #[doc = "0x10 - GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn outen_set(&self) -> &OutenSet {
        &self.outen_set
    }
    #[doc = "0x14 - GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn outen_clr(&self) -> &OutenClr {
        &self.outen_clr
    }
    #[doc = "0x18 - GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers."]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x1c - GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn out_set(&self) -> &OutSet {
        &self.out_set
    }
    #[doc = "0x20 - GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn out_clr(&self) -> &OutClr {
        &self.out_clr
    }
    #[doc = "0x24 - GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port."]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x28 - GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port."]
    #[inline(always)]
    pub const fn intmode(&self) -> &Intmode {
        &self.intmode
    }
    #[doc = "0x2c - GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port."]
    #[inline(always)]
    pub const fn intpol(&self) -> &Intpol {
        &self.intpol
    }
    #[doc = "0x30 - GPIO Input Enable"]
    #[inline(always)]
    pub const fn inen(&self) -> &Inen {
        &self.inen
    }
    #[doc = "0x34 - GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port."]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x38 - GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn inten_set(&self) -> &IntenSet {
        &self.inten_set
    }
    #[doc = "0x3c - GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn inten_clr(&self) -> &IntenClr {
        &self.inten_clr
    }
    #[doc = "0x40 - GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port."]
    #[inline(always)]
    pub const fn intfl(&self) -> &Intfl {
        &self.intfl
    }
    #[doc = "0x48 - GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn intfl_clr(&self) -> &IntflClr {
        &self.intfl_clr
    }
    #[doc = "0x4c - GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port."]
    #[inline(always)]
    pub const fn wken(&self) -> &Wken {
        &self.wken
    }
    #[doc = "0x50 - GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn wken_set(&self) -> &WkenSet {
        &self.wken_set
    }
    #[doc = "0x54 - GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn wken_clr(&self) -> &WkenClr {
        &self.wken_clr
    }
    #[doc = "0x5c - GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port."]
    #[inline(always)]
    pub const fn dualedge(&self) -> &Dualedge {
        &self.dualedge
    }
    #[doc = "0x60 - GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
    #[inline(always)]
    pub const fn padctrl0(&self) -> &Padctrl0 {
        &self.padctrl0
    }
    #[doc = "0x64 - GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
    #[inline(always)]
    pub const fn padctrl1(&self) -> &Padctrl1 {
        &self.padctrl1
    }
    #[doc = "0x68 - GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
    #[inline(always)]
    pub const fn en1(&self) -> &En1 {
        &self.en1
    }
    #[doc = "0x6c - GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn en1_set(&self) -> &En1Set {
        &self.en1_set
    }
    #[doc = "0x70 - GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn en1_clr(&self) -> &En1Clr {
        &self.en1_clr
    }
    #[doc = "0x74 - GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
    #[inline(always)]
    pub const fn en2(&self) -> &En2 {
        &self.en2
    }
    #[doc = "0x78 - GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn en2_set(&self) -> &En2Set {
        &self.en2_set
    }
    #[doc = "0x7c - GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register."]
    #[inline(always)]
    pub const fn en2_clr(&self) -> &En2Clr {
        &self.en2_clr
    }
    #[doc = "0xa8 - GPIO Input Hysteresis Enable."]
    #[inline(always)]
    pub const fn hysen(&self) -> &Hysen {
        &self.hysen
    }
    #[doc = "0xac - GPIO Slew Rate Enable Register."]
    #[inline(always)]
    pub const fn srsel(&self) -> &Srsel {
        &self.srsel
    }
    #[doc = "0xb0 - GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
    #[inline(always)]
    pub const fn ds0(&self) -> &Ds0 {
        &self.ds0
    }
    #[doc = "0xb4 - GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
    #[inline(always)]
    pub const fn ds1(&self) -> &Ds1 {
        &self.ds1
    }
    #[doc = "0xb8 - GPIO Pull Select Mode."]
    #[inline(always)]
    pub const fn ps(&self) -> &Ps {
        &self.ps
    }
    #[doc = "0xc0 - GPIO Voltage Select."]
    #[inline(always)]
    pub const fn vssel(&self) -> &Vssel {
        &self.vssel
    }
}
#[doc = "EN0 (rw) register accessor: GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port.\n\nYou can [`read`](crate::Reg::read) this register and get [`en0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en0`]
module"]
#[doc(alias = "EN0")]
pub type En0 = crate::Reg<en0::En0Spec>;
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port."]
pub mod en0;
#[doc = "EN0_SET (rw) register accessor: GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`en0_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en0_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en0_set`]
module"]
#[doc(alias = "EN0_SET")]
pub type En0Set = crate::Reg<en0_set::En0SetSpec>;
#[doc = "GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register."]
pub mod en0_set;
#[doc = "EN0_CLR (rw) register accessor: GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`en0_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en0_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en0_clr`]
module"]
#[doc(alias = "EN0_CLR")]
pub type En0Clr = crate::Reg<en0_clr::En0ClrSpec>;
#[doc = "GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register."]
pub mod en0_clr;
#[doc = "OUTEN (rw) register accessor: GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::Reg::read) this register and get [`outen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outen`]
module"]
#[doc(alias = "OUTEN")]
pub type Outen = crate::Reg<outen::OutenSpec>;
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port."]
pub mod outen;
#[doc = "OUTEN_SET (rw) register accessor: GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`outen_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outen_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outen_set`]
module"]
#[doc(alias = "OUTEN_SET")]
pub type OutenSet = crate::Reg<outen_set::OutenSetSpec>;
#[doc = "GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register."]
pub mod outen_set;
#[doc = "OUTEN_CLR (rw) register accessor: GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`outen_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outen_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outen_clr`]
module"]
#[doc(alias = "OUTEN_CLR")]
pub type OutenClr = crate::Reg<outen_clr::OutenClrSpec>;
#[doc = "GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register."]
pub mod outen_clr;
#[doc = "OUT (rw) register accessor: GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers."]
pub mod out;
#[doc = "OUT_SET (w) register accessor: GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_set`]
module"]
#[doc(alias = "OUT_SET")]
pub type OutSet = crate::Reg<out_set::OutSetSpec>;
#[doc = "GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register."]
pub mod out_set;
#[doc = "OUT_CLR (w) register accessor: GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_clr`]
module"]
#[doc(alias = "OUT_CLR")]
pub type OutClr = crate::Reg<out_clr::OutClrSpec>;
#[doc = "GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register."]
pub mod out_clr;
#[doc = "IN (r) register accessor: GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port."]
pub mod in_;
#[doc = "INTMODE (rw) register accessor: GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`intmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmode`]
module"]
#[doc(alias = "INTMODE")]
pub type Intmode = crate::Reg<intmode::IntmodeSpec>;
#[doc = "GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port."]
pub mod intmode;
#[doc = "INTPOL (rw) register accessor: GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::Reg::read) this register and get [`intpol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intpol`]
module"]
#[doc(alias = "INTPOL")]
pub type Intpol = crate::Reg<intpol::IntpolSpec>;
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port."]
pub mod intpol;
#[doc = "INEN (rw) register accessor: GPIO Input Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inen`]
module"]
#[doc(alias = "INEN")]
pub type Inen = crate::Reg<inen::InenSpec>;
#[doc = "GPIO Input Enable"]
pub mod inen;
#[doc = "INTEN (rw) register accessor: GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port."]
pub mod inten;
#[doc = "INTEN_SET (rw) register accessor: GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten_set`]
module"]
#[doc(alias = "INTEN_SET")]
pub type IntenSet = crate::Reg<inten_set::IntenSetSpec>;
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register."]
pub mod inten_set;
#[doc = "INTEN_CLR (rw) register accessor: GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten_clr`]
module"]
#[doc(alias = "INTEN_CLR")]
pub type IntenClr = crate::Reg<inten_clr::IntenClrSpec>;
#[doc = "GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register."]
pub mod inten_clr;
#[doc = "INTFL (r) register accessor: GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl`]
module"]
#[doc(alias = "INTFL")]
pub type Intfl = crate::Reg<intfl::IntflSpec>;
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port."]
pub mod intfl;
#[doc = "INTFL_CLR (rw) register accessor: GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intfl_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intfl_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intfl_clr`]
module"]
#[doc(alias = "INTFL_CLR")]
pub type IntflClr = crate::Reg<intfl_clr::IntflClrSpec>;
#[doc = "GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register."]
pub mod intfl_clr;
#[doc = "WKEN (rw) register accessor: GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`wken::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken`]
module"]
#[doc(alias = "WKEN")]
pub type Wken = crate::Reg<wken::WkenSpec>;
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port."]
pub mod wken;
#[doc = "WKEN_SET (rw) register accessor: GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wken_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken_set`]
module"]
#[doc(alias = "WKEN_SET")]
pub type WkenSet = crate::Reg<wken_set::WkenSetSpec>;
#[doc = "GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register."]
pub mod wken_set;
#[doc = "WKEN_CLR (rw) register accessor: GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`wken_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wken_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wken_clr`]
module"]
#[doc(alias = "WKEN_CLR")]
pub type WkenClr = crate::Reg<wken_clr::WkenClrSpec>;
#[doc = "GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register."]
pub mod wken_clr;
#[doc = "DUALEDGE (rw) register accessor: GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`dualedge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dualedge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dualedge`]
module"]
#[doc(alias = "DUALEDGE")]
pub type Dualedge = crate::Reg<dualedge::DualedgeSpec>;
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port."]
pub mod dualedge;
#[doc = "PADCTRL0 (rw) register accessor: GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`padctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padctrl0`]
module"]
#[doc(alias = "PADCTRL0")]
pub type Padctrl0 = crate::Reg<padctrl0::Padctrl0Spec>;
#[doc = "GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
pub mod padctrl0;
#[doc = "PADCTRL1 (rw) register accessor: GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`padctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`padctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@padctrl1`]
module"]
#[doc(alias = "PADCTRL1")]
pub type Padctrl1 = crate::Reg<padctrl1::Padctrl1Spec>;
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
pub mod padctrl1;
#[doc = "EN1 (rw) register accessor: GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`en1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en1`]
module"]
#[doc(alias = "EN1")]
pub type En1 = crate::Reg<en1::En1Spec>;
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
pub mod en1;
#[doc = "EN1_SET (rw) register accessor: GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`en1_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en1_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en1_set`]
module"]
#[doc(alias = "EN1_SET")]
pub type En1Set = crate::Reg<en1_set::En1SetSpec>;
#[doc = "GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register."]
pub mod en1_set;
#[doc = "EN1_CLR (rw) register accessor: GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`en1_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en1_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en1_clr`]
module"]
#[doc(alias = "EN1_CLR")]
pub type En1Clr = crate::Reg<en1_clr::En1ClrSpec>;
#[doc = "GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register."]
pub mod en1_clr;
#[doc = "EN2 (rw) register accessor: GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nYou can [`read`](crate::Reg::read) this register and get [`en2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en2`]
module"]
#[doc(alias = "EN2")]
pub type En2 = crate::Reg<en2::En2Spec>;
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
pub mod en2;
#[doc = "EN2_SET (rw) register accessor: GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`en2_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en2_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en2_set`]
module"]
#[doc(alias = "EN2_SET")]
pub type En2Set = crate::Reg<en2_set::En2SetSpec>;
#[doc = "GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register."]
pub mod en2_set;
#[doc = "EN2_CLR (rw) register accessor: GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::Reg::read) this register and get [`en2_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en2_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en2_clr`]
module"]
#[doc(alias = "EN2_CLR")]
pub type En2Clr = crate::Reg<en2_clr::En2ClrSpec>;
#[doc = "GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register."]
pub mod en2_clr;
#[doc = "HYSEN (rw) register accessor: GPIO Input Hysteresis Enable.\n\nYou can [`read`](crate::Reg::read) this register and get [`hysen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hysen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hysen`]
module"]
#[doc(alias = "HYSEN")]
pub type Hysen = crate::Reg<hysen::HysenSpec>;
#[doc = "GPIO Input Hysteresis Enable."]
pub mod hysen;
#[doc = "SRSEL (rw) register accessor: GPIO Slew Rate Enable Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`srsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srsel`]
module"]
#[doc(alias = "SRSEL")]
pub type Srsel = crate::Reg<srsel::SrselSpec>;
#[doc = "GPIO Slew Rate Enable Register."]
pub mod srsel;
#[doc = "DS0 (rw) register accessor: GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ds0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ds0`]
module"]
#[doc(alias = "DS0")]
pub type Ds0 = crate::Reg<ds0::Ds0Spec>;
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
pub mod ds0;
#[doc = "DS1 (rw) register accessor: GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ds1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ds1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ds1`]
module"]
#[doc(alias = "DS1")]
pub type Ds1 = crate::Reg<ds1::Ds1Spec>;
#[doc = "GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
pub mod ds1;
#[doc = "PS (rw) register accessor: GPIO Pull Select Mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`ps::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ps::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ps`]
module"]
#[doc(alias = "PS")]
pub type Ps = crate::Reg<ps::PsSpec>;
#[doc = "GPIO Pull Select Mode."]
pub mod ps;
#[doc = "VSSEL (rw) register accessor: GPIO Voltage Select.\n\nYou can [`read`](crate::Reg::read) this register and get [`vssel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vssel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vssel`]
module"]
#[doc(alias = "VSSEL")]
pub type Vssel = crate::Reg<vssel::VsselSpec>;
#[doc = "GPIO Voltage Select."]
pub mod vssel;
