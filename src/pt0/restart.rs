#[doc = "Register `RESTART` reader"]
pub type R = crate::R<RestartSpec>;
#[doc = "Register `RESTART` writer"]
pub type W = crate::W<RestartSpec>;
#[doc = "Field `pt_x_select` reader - Auto-Restart PT X Select"]
pub type PtXSelectR = crate::FieldReader;
#[doc = "Field `pt_x_select` writer - Auto-Restart PT X Select"]
pub type PtXSelectW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `on_pt_x_loop_exit` reader - Enable Auto-Restart on PT X Loop Exit"]
pub type OnPtXLoopExitR = crate::BitReader;
#[doc = "Field `on_pt_x_loop_exit` writer - Enable Auto-Restart on PT X Loop Exit"]
pub type OnPtXLoopExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pt_y_select` reader - Auto-Restart PT Y Select"]
pub type PtYSelectR = crate::FieldReader;
#[doc = "Field `pt_y_select` writer - Auto-Restart PT Y Select"]
pub type PtYSelectW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `on_pt_y_loop_exit` reader - Enable Auto-Restart on PT Y Loop Exit"]
pub type OnPtYLoopExitR = crate::BitReader;
#[doc = "Field `on_pt_y_loop_exit` writer - Enable Auto-Restart on PT Y Loop Exit"]
pub type OnPtYLoopExitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Auto-Restart PT X Select"]
    #[inline(always)]
    pub fn pt_x_select(&self) -> PtXSelectR {
        PtXSelectR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Enable Auto-Restart on PT X Loop Exit"]
    #[inline(always)]
    pub fn on_pt_x_loop_exit(&self) -> OnPtXLoopExitR {
        OnPtXLoopExitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Auto-Restart PT Y Select"]
    #[inline(always)]
    pub fn pt_y_select(&self) -> PtYSelectR {
        PtYSelectR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Enable Auto-Restart on PT Y Loop Exit"]
    #[inline(always)]
    pub fn on_pt_y_loop_exit(&self) -> OnPtYLoopExitR {
        OnPtYLoopExitR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Auto-Restart PT X Select"]
    #[inline(always)]
    pub fn pt_x_select(&mut self) -> PtXSelectW<RestartSpec> {
        PtXSelectW::new(self, 0)
    }
    #[doc = "Bit 7 - Enable Auto-Restart on PT X Loop Exit"]
    #[inline(always)]
    pub fn on_pt_x_loop_exit(&mut self) -> OnPtXLoopExitW<RestartSpec> {
        OnPtXLoopExitW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Auto-Restart PT Y Select"]
    #[inline(always)]
    pub fn pt_y_select(&mut self) -> PtYSelectW<RestartSpec> {
        PtYSelectW::new(self, 8)
    }
    #[doc = "Bit 15 - Enable Auto-Restart on PT Y Loop Exit"]
    #[inline(always)]
    pub fn on_pt_y_loop_exit(&mut self) -> OnPtYLoopExitW<RestartSpec> {
        OnPtYLoopExitW::new(self, 15)
    }
}
#[doc = "Pulse Train Auto-Restart Configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`restart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`restart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RestartSpec;
impl crate::RegisterSpec for RestartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`restart::R`](R) reader structure"]
impl crate::Readable for RestartSpec {}
#[doc = "`write(|w| ..)` method takes [`restart::W`](W) writer structure"]
impl crate::Writable for RestartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESTART to value 0"]
impl crate::Resettable for RestartSpec {
    const RESET_VALUE: u32 = 0;
}
