#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: [Ctrl; 3],
}
impl RegisterBlock {
    #[doc = "0x00..0x0c - Comparator Control Register."]
    #[inline(always)]
    pub const fn ctrl(&self, n: usize) -> &Ctrl {
        &self.ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x0c - Comparator Control Register."]
    #[inline(always)]
    pub fn ctrl_iter(&self) -> impl Iterator<Item = &Ctrl> {
        self.ctrl.iter()
    }
}
#[doc = "CTRL (rw) register accessor: Comparator Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Comparator Control Register."]
pub mod ctrl;
