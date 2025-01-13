#[doc = "Register `IPKB` reader"]
pub type R = crate::R<IpkbSpec>;
#[doc = "Register `IPKB` writer"]
pub type W = crate::W<IpkbSpec>;
#[doc = "Field `IPKSETC` reader - Voltage Regulator Peak Current Setting"]
pub type IpksetcR = crate::FieldReader;
#[doc = "Field `IPKSETC` writer - Voltage Regulator Peak Current Setting"]
pub type IpksetcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IPKSETD` reader - Voltage Regulator Peak Current Setting"]
pub type IpksetdR = crate::FieldReader;
#[doc = "Field `IPKSETD` writer - Voltage Regulator Peak Current Setting"]
pub type IpksetdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetc(&self) -> IpksetcR {
        IpksetcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetd(&self) -> IpksetdR {
        IpksetdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetc(&mut self) -> IpksetcW<IpkbSpec> {
        IpksetcW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Voltage Regulator Peak Current Setting"]
    #[inline(always)]
    pub fn ipksetd(&mut self) -> IpksetdW<IpkbSpec> {
        IpksetdW::new(self, 4)
    }
}
#[doc = "High Side FET Peak Current VREGO_C/VREGO_D Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ipkb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipkb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpkbSpec;
impl crate::RegisterSpec for IpkbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipkb::R`](R) reader structure"]
impl crate::Readable for IpkbSpec {}
#[doc = "`write(|w| ..)` method takes [`ipkb::W`](W) writer structure"]
impl crate::Writable for IpkbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IPKB to value 0"]
impl crate::Resettable for IpkbSpec {
    const RESET_VALUE: u32 = 0;
}
