#[doc = "Register `RST` writer"]
pub type W = crate::W<RstSpec>;
#[doc = "Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Reset {
    #[doc = "165: The first value to be written to reset the WDT."]
    Seq0 = 165,
    #[doc = "90: The second value to be written to reset the WDT."]
    Seq1 = 90,
}
impl From<Reset> for u8 {
    #[inline(always)]
    fn from(variant: Reset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Reset {
    type Ux = u8;
}
impl crate::IsEnum for Reset {}
#[doc = "Field `RESET` writer - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled."]
pub type ResetW<'a, REG> = crate::FieldWriter<'a, REG, 8, Reset>;
impl<'a, REG> ResetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The first value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq0(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Seq0)
    }
    #[doc = "The second value to be written to reset the WDT."]
    #[inline(always)]
    pub fn seq1(self) -> &'a mut crate::W<REG> {
        self.variant(Reset::Seq1)
    }
}
impl W {
    #[doc = "Bits 0:7 - Writing the watchdog counter 'reset sequence' to this register resets the watchdog counter. If the watchdog count exceeds INT_PERIOD_UPPER_LIMIT then a watchdog interrupt will occur, if enabled. If the watchdog count exceeds RST_PERIOD_UPPER_LIMIT then a watchdog reset will occur, if enabled."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<RstSpec> {
        ResetW::new(self, 0)
    }
}
#[doc = "Windowed Watchdog Timer Reset Register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstSpec;
impl crate::RegisterSpec for RstSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rst::W`](W) writer structure"]
impl crate::Writable for RstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RST to value 0"]
impl crate::Resettable for RstSpec {
    const RESET_VALUE: u32 = 0;
}
