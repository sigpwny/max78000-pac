#[doc = "Register `SNAPSHOT` reader"]
pub type R = crate::R<SnapshotSpec>;
#[doc = "Register `SNAPSHOT` writer"]
pub type W = crate::W<SnapshotSpec>;
#[doc = "Field `SNAPSHOT` reader - Snapshot Value."]
pub type SnapshotR = crate::FieldReader<u32>;
#[doc = "Field `SNAPSHOT` writer - Snapshot Value."]
pub type SnapshotW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Snapshot Value."]
    #[inline(always)]
    pub fn snapshot(&self) -> SnapshotR {
        SnapshotR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Snapshot Value."]
    #[inline(always)]
    pub fn snapshot(&mut self) -> SnapshotW<SnapshotSpec> {
        SnapshotW::new(self, 0)
    }
}
#[doc = "Snapshot register.\n\nYou can [`read`](crate::Reg::read) this register and get [`snapshot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snapshot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SnapshotSpec;
impl crate::RegisterSpec for SnapshotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`snapshot::R`](R) reader structure"]
impl crate::Readable for SnapshotSpec {}
#[doc = "`write(|w| ..)` method takes [`snapshot::W`](W) writer structure"]
impl crate::Writable for SnapshotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SNAPSHOT to value 0"]
impl crate::Resettable for SnapshotSpec {
    const RESET_VALUE: u32 = 0;
}
