#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    semaphores: [Semaphores; 8],
    _reserved1: [u8; 0x20],
    irq0: Irq0,
    mail0: Mail0,
    irq1: Irq1,
    mail1: Mail1,
    _reserved5: [u8; 0xb0],
    status: Status,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Read to test and set, returns prior value. Write 0 to clear semaphore."]
    #[inline(always)]
    pub const fn semaphores(&self, n: usize) -> &Semaphores {
        &self.semaphores[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Read to test and set, returns prior value. Write 0 to clear semaphore."]
    #[inline(always)]
    pub fn semaphores_iter(&self) -> impl Iterator<Item = &Semaphores> {
        self.semaphores.iter()
    }
    #[doc = "0x40 - Semaphore IRQ0 register."]
    #[inline(always)]
    pub const fn irq0(&self) -> &Irq0 {
        &self.irq0
    }
    #[doc = "0x44 - Semaphore Mailbox 0 register."]
    #[inline(always)]
    pub const fn mail0(&self) -> &Mail0 {
        &self.mail0
    }
    #[doc = "0x48 - Semaphore IRQ1 register."]
    #[inline(always)]
    pub const fn irq1(&self) -> &Irq1 {
        &self.irq1
    }
    #[doc = "0x4c - Semaphore Mailbox 1 register."]
    #[inline(always)]
    pub const fn mail1(&self) -> &Mail1 {
        &self.mail1
    }
    #[doc = "0x100 - Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
}
#[doc = "SEMAPHORES (rw) register accessor: Read to test and set, returns prior value. Write 0 to clear semaphore.\n\nYou can [`read`](crate::Reg::read) this register and get [`semaphores::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`semaphores::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@semaphores`]
module"]
#[doc(alias = "SEMAPHORES")]
pub type Semaphores = crate::Reg<semaphores::SemaphoresSpec>;
#[doc = "Read to test and set, returns prior value. Write 0 to clear semaphore."]
pub mod semaphores;
#[doc = "irq0 (rw) register accessor: Semaphore IRQ0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq0`]
module"]
#[doc(alias = "irq0")]
pub type Irq0 = crate::Reg<irq0::Irq0Spec>;
#[doc = "Semaphore IRQ0 register."]
pub mod irq0;
#[doc = "mail0 (rw) register accessor: Semaphore Mailbox 0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mail0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mail0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mail0`]
module"]
#[doc(alias = "mail0")]
pub type Mail0 = crate::Reg<mail0::Mail0Spec>;
#[doc = "Semaphore Mailbox 0 register."]
pub mod mail0;
#[doc = "irq1 (rw) register accessor: Semaphore IRQ1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq1`]
module"]
#[doc(alias = "irq1")]
pub type Irq1 = crate::Reg<irq1::Irq1Spec>;
#[doc = "Semaphore IRQ1 register."]
pub mod irq1;
#[doc = "mail1 (rw) register accessor: Semaphore Mailbox 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mail1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mail1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mail1`]
module"]
#[doc(alias = "mail1")]
pub type Mail1 = crate::Reg<mail1::Mail1Spec>;
#[doc = "Semaphore Mailbox 1 register."]
pub mod mail1;
#[doc = "status (rw) register accessor: Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken."]
pub mod status;
