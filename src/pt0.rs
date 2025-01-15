#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rate_length: RateLength,
    train: Train,
    loop_: Loop,
    restart: Restart,
}
impl RegisterBlock {
    #[doc = "0x00 - Pulse Train Configuration"]
    #[inline(always)]
    pub const fn rate_length(&self) -> &RateLength {
        &self.rate_length
    }
    #[doc = "0x04 - Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length."]
    #[inline(always)]
    pub const fn train(&self) -> &Train {
        &self.train
    }
    #[doc = "0x08 - Pulse Train Loop Count"]
    #[inline(always)]
    pub const fn loop_(&self) -> &Loop {
        &self.loop_
    }
    #[doc = "0x0c - Pulse Train Auto-Restart Configuration."]
    #[inline(always)]
    pub const fn restart(&self) -> &Restart {
        &self.restart
    }
}
#[doc = "RATE_LENGTH (rw) register accessor: Pulse Train Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`rate_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rate_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rate_length`]
module"]
#[doc(alias = "RATE_LENGTH")]
pub type RateLength = crate::Reg<rate_length::RateLengthSpec>;
#[doc = "Pulse Train Configuration"]
pub mod rate_length;
#[doc = "TRAIN (rw) register accessor: Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length.\n\nYou can [`read`](crate::Reg::read) this register and get [`train::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`train::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@train`]
module"]
#[doc(alias = "TRAIN")]
pub type Train = crate::Reg<train::TrainSpec>;
#[doc = "Write the repeating bit pattern that is shifted out, LSB first, when configured in Pulse Train mode. See PT_RATE_LENGTH.mode for setting the length."]
pub mod train;
#[doc = "LOOP (rw) register accessor: Pulse Train Loop Count\n\nYou can [`read`](crate::Reg::read) this register and get [`loop_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`loop_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loop_`]
module"]
#[doc(alias = "LOOP")]
pub type Loop = crate::Reg<loop_::LoopSpec>;
#[doc = "Pulse Train Loop Count"]
pub mod loop_;
#[doc = "RESTART (rw) register accessor: Pulse Train Auto-Restart Configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`restart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@restart`]
module"]
#[doc(alias = "RESTART")]
pub type Restart = crate::Reg<restart::RestartSpec>;
#[doc = "Pulse Train Auto-Restart Configuration."]
pub mod restart;
