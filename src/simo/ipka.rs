#[doc = "Register `IPKA` reader"]
pub type R = crate::R<IpkaSpec>;
#[doc = "Register `IPKA` writer"]
pub type W = crate::W<IpkaSpec>;
#[doc = "Field `IPKSETA` reader - Voltage Regulator Peak Current Setting"]
pub type IpksetaR = crate::FieldReader;
#[doc = "Field `IPKSETA` writer - Voltage Regulator Peak Current Setting"]
pub type IpksetaW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IPKSETB` reader - Voltage Regulator Peak Current Setting"]
pub type IpksetbR = crate::FieldReader;
#[doc = "Field `IPKSETB` writer - Voltage Regulator Peak Current Setting"]
pub type IpksetbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipkseta(&self) -> IpksetaR {
        IpksetaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetb(&self) -> IpksetbR {
        IpksetbR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipkseta(&mut self) -> IpksetaW<IpkaSpec> {
        IpksetaW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetb(&mut self) -> IpksetbW<IpkaSpec> {
        IpksetbW::new(self, 4)
    }
}
#[doc = "High Side FET Peak Current VREGO_A/VREGO_B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipka::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipka::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpkaSpec;
impl crate::RegisterSpec for IpkaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipka::R`](R) reader structure"]
impl crate::Readable for IpkaSpec {}
#[doc = "`write(|w| ..)` method takes [`ipka::W`](W) writer structure"]
impl crate::Writable for IpkaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPKA to value 0"]
impl crate::Resettable for IpkaSpec {
    const RESET_VALUE: u32 = 0;
}
