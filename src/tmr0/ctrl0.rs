#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "Mode Select for Timer A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ModeA {
    #[doc = "0: One-Shot Mode"]
    OneShot = 0,
    #[doc = "1: Continuous Mode"]
    Continuous = 1,
    #[doc = "2: Counter Mode"]
    Counter = 2,
    #[doc = "3: PWM Mode"]
    Pwm = 3,
    #[doc = "4: Capture Mode"]
    Capture = 4,
    #[doc = "5: Compare Mode"]
    Compare = 5,
    #[doc = "6: Gated Mode"]
    Gated = 6,
    #[doc = "7: Capture/Compare Mode"]
    Capcomp = 7,
    #[doc = "8: Dual Edge Capture Mode"]
    DualEdge = 8,
    #[doc = "14: Inactive Gated Mode"]
    Igated = 14,
}
impl From<ModeA> for u8 {
    #[inline(always)]
    fn from(variant: ModeA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ModeA {
    type Ux = u8;
}
impl crate::IsEnum for ModeA {}
#[doc = "Field `MODE_A` reader - Mode Select for Timer A"]
pub type ModeAR = crate::FieldReader<ModeA>;
impl ModeAR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ModeA> {
        match self.bits {
            0 => Some(ModeA::OneShot),
            1 => Some(ModeA::Continuous),
            2 => Some(ModeA::Counter),
            3 => Some(ModeA::Pwm),
            4 => Some(ModeA::Capture),
            5 => Some(ModeA::Compare),
            6 => Some(ModeA::Gated),
            7 => Some(ModeA::Capcomp),
            8 => Some(ModeA::DualEdge),
            14 => Some(ModeA::Igated),
            _ => None,
        }
    }
    #[doc = "One-Shot Mode"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == ModeA::OneShot
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == ModeA::Continuous
    }
    #[doc = "Counter Mode"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == ModeA::Counter
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == ModeA::Pwm
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == ModeA::Capture
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == ModeA::Compare
    }
    #[doc = "Gated Mode"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == ModeA::Gated
    }
    #[doc = "Capture/Compare Mode"]
    #[inline(always)]
    pub fn is_capcomp(&self) -> bool {
        *self == ModeA::Capcomp
    }
    #[doc = "Dual Edge Capture Mode"]
    #[inline(always)]
    pub fn is_dual_edge(&self) -> bool {
        *self == ModeA::DualEdge
    }
    #[doc = "Inactive Gated Mode"]
    #[inline(always)]
    pub fn is_igated(&self) -> bool {
        *self == ModeA::Igated
    }
}
#[doc = "Field `MODE_A` writer - Mode Select for Timer A"]
pub type ModeAW<'a, REG> = crate::FieldWriter<'a, REG, 4, ModeA>;
impl<'a, REG> ModeAW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One-Shot Mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::OneShot)
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::Continuous)
    }
    #[doc = "Counter Mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::Counter)
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::Pwm)
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::Capture)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::Compare)
    }
    #[doc = "Gated Mode"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::Gated)
    }
    #[doc = "Capture/Compare Mode"]
    #[inline(always)]
    pub fn capcomp(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::Capcomp)
    }
    #[doc = "Dual Edge Capture Mode"]
    #[inline(always)]
    pub fn dual_edge(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::DualEdge)
    }
    #[doc = "Inactive Gated Mode"]
    #[inline(always)]
    pub fn igated(self) -> &'a mut crate::W<REG> {
        self.variant(ModeA::Igated)
    }
}
#[doc = "Clock Divider Select for Timer A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkdivA {
    #[doc = "0: Prescaler Divide-By-1"]
    DivBy1 = 0,
    #[doc = "1: Prescaler Divide-By-2"]
    DivBy2 = 1,
    #[doc = "2: Prescaler Divide-By-4"]
    DivBy4 = 2,
    #[doc = "3: Prescaler Divide-By-8"]
    DivBy8 = 3,
    #[doc = "4: Prescaler Divide-By-16"]
    DivBy16 = 4,
    #[doc = "5: Prescaler Divide-By-32"]
    DivBy32 = 5,
    #[doc = "6: Prescaler Divide-By-64"]
    DivBy64 = 6,
    #[doc = "7: Prescaler Divide-By-128"]
    DivBy128 = 7,
    #[doc = "8: Prescaler Divide-By-256"]
    DivBy256 = 8,
    #[doc = "9: Prescaler Divide-By-512"]
    DivBy512 = 9,
    #[doc = "10: Prescaler Divide-By-1024"]
    DivBy1024 = 10,
    #[doc = "11: Prescaler Divide-By-2048"]
    DivBy2048 = 11,
    #[doc = "12: TBD"]
    DivBy4096 = 12,
}
impl From<ClkdivA> for u8 {
    #[inline(always)]
    fn from(variant: ClkdivA) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkdivA {
    type Ux = u8;
}
impl crate::IsEnum for ClkdivA {}
#[doc = "Field `CLKDIV_A` reader - Clock Divider Select for Timer A"]
pub type ClkdivAR = crate::FieldReader<ClkdivA>;
impl ClkdivAR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkdivA> {
        match self.bits {
            0 => Some(ClkdivA::DivBy1),
            1 => Some(ClkdivA::DivBy2),
            2 => Some(ClkdivA::DivBy4),
            3 => Some(ClkdivA::DivBy8),
            4 => Some(ClkdivA::DivBy16),
            5 => Some(ClkdivA::DivBy32),
            6 => Some(ClkdivA::DivBy64),
            7 => Some(ClkdivA::DivBy128),
            8 => Some(ClkdivA::DivBy256),
            9 => Some(ClkdivA::DivBy512),
            10 => Some(ClkdivA::DivBy1024),
            11 => Some(ClkdivA::DivBy2048),
            12 => Some(ClkdivA::DivBy4096),
            _ => None,
        }
    }
    #[doc = "Prescaler Divide-By-1"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == ClkdivA::DivBy1
    }
    #[doc = "Prescaler Divide-By-2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == ClkdivA::DivBy2
    }
    #[doc = "Prescaler Divide-By-4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == ClkdivA::DivBy4
    }
    #[doc = "Prescaler Divide-By-8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == ClkdivA::DivBy8
    }
    #[doc = "Prescaler Divide-By-16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == ClkdivA::DivBy16
    }
    #[doc = "Prescaler Divide-By-32"]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == ClkdivA::DivBy32
    }
    #[doc = "Prescaler Divide-By-64"]
    #[inline(always)]
    pub fn is_div_by_64(&self) -> bool {
        *self == ClkdivA::DivBy64
    }
    #[doc = "Prescaler Divide-By-128"]
    #[inline(always)]
    pub fn is_div_by_128(&self) -> bool {
        *self == ClkdivA::DivBy128
    }
    #[doc = "Prescaler Divide-By-256"]
    #[inline(always)]
    pub fn is_div_by_256(&self) -> bool {
        *self == ClkdivA::DivBy256
    }
    #[doc = "Prescaler Divide-By-512"]
    #[inline(always)]
    pub fn is_div_by_512(&self) -> bool {
        *self == ClkdivA::DivBy512
    }
    #[doc = "Prescaler Divide-By-1024"]
    #[inline(always)]
    pub fn is_div_by_1024(&self) -> bool {
        *self == ClkdivA::DivBy1024
    }
    #[doc = "Prescaler Divide-By-2048"]
    #[inline(always)]
    pub fn is_div_by_2048(&self) -> bool {
        *self == ClkdivA::DivBy2048
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn is_div_by_4096(&self) -> bool {
        *self == ClkdivA::DivBy4096
    }
}
#[doc = "Field `CLKDIV_A` writer - Clock Divider Select for Timer A"]
pub type ClkdivAW<'a, REG> = crate::FieldWriter<'a, REG, 4, ClkdivA>;
impl<'a, REG> ClkdivAW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler Divide-By-1"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy1)
    }
    #[doc = "Prescaler Divide-By-2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy2)
    }
    #[doc = "Prescaler Divide-By-4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy4)
    }
    #[doc = "Prescaler Divide-By-8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy8)
    }
    #[doc = "Prescaler Divide-By-16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy16)
    }
    #[doc = "Prescaler Divide-By-32"]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy32)
    }
    #[doc = "Prescaler Divide-By-64"]
    #[inline(always)]
    pub fn div_by_64(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy64)
    }
    #[doc = "Prescaler Divide-By-128"]
    #[inline(always)]
    pub fn div_by_128(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy128)
    }
    #[doc = "Prescaler Divide-By-256"]
    #[inline(always)]
    pub fn div_by_256(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy256)
    }
    #[doc = "Prescaler Divide-By-512"]
    #[inline(always)]
    pub fn div_by_512(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy512)
    }
    #[doc = "Prescaler Divide-By-1024"]
    #[inline(always)]
    pub fn div_by_1024(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy1024)
    }
    #[doc = "Prescaler Divide-By-2048"]
    #[inline(always)]
    pub fn div_by_2048(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy2048)
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn div_by_4096(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivA::DivBy4096)
    }
}
#[doc = "Field `POL_A` reader - Timer Polarity for Timer A"]
pub type PolAR = crate::BitReader;
#[doc = "Field `POL_A` writer - Timer Polarity for Timer A"]
pub type PolAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNC_A` reader - PWM Synchronization Mode for Timer A"]
pub type PwmsyncAR = crate::BitReader;
#[doc = "Field `PWMSYNC_A` writer - PWM Synchronization Mode for Timer A"]
pub type PwmsyncAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOLHPOL_A` reader - PWM Phase A (Non-Overlapping High) Polarity for Timer A"]
pub type NolhpolAR = crate::BitReader;
#[doc = "Field `NOLHPOL_A` writer - PWM Phase A (Non-Overlapping High) Polarity for Timer A"]
pub type NolhpolAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOLLPOL_A` reader - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer A"]
pub type NollpolAR = crate::BitReader;
#[doc = "Field `NOLLPOL_A` writer - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer A"]
pub type NollpolAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMCKBD_A` reader - PWM Phase A-Prime Output Disable for Timer A"]
pub type PwmckbdAR = crate::BitReader;
#[doc = "Field `PWMCKBD_A` writer - PWM Phase A-Prime Output Disable for Timer A"]
pub type PwmckbdAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_A` reader - Resets all flip flops in the CLK_TMR domain for Timer A. Self-clears."]
pub type RstAR = crate::BitReader;
#[doc = "Field `RST_A` writer - Resets all flip flops in the CLK_TMR domain for Timer A. Self-clears."]
pub type RstAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN_A` reader - Write 1 to Enable CLK_TMR for Timer A"]
pub type ClkenAR = crate::BitReader;
#[doc = "Field `CLKEN_A` writer - Write 1 to Enable CLK_TMR for Timer A"]
pub type ClkenAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_A` reader - Enable for Timer A"]
pub type EnAR = crate::BitReader;
#[doc = "Field `EN_A` writer - Enable for Timer A"]
pub type EnAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Mode Select for Timer B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ModeB {
    #[doc = "0: One-Shot Mode"]
    OneShot = 0,
    #[doc = "1: Continuous Mode"]
    Continuous = 1,
    #[doc = "2: Counter Mode"]
    Counter = 2,
    #[doc = "3: PWM Mode"]
    Pwm = 3,
    #[doc = "4: Capture Mode"]
    Capture = 4,
    #[doc = "5: Compare Mode"]
    Compare = 5,
    #[doc = "6: Gated Mode"]
    Gated = 6,
    #[doc = "7: Capture/Compare Mode"]
    Capcomp = 7,
    #[doc = "8: Dual Edge Capture Mode"]
    DualEdge = 8,
    #[doc = "14: Inactive Gated Mode"]
    Igated = 14,
}
impl From<ModeB> for u8 {
    #[inline(always)]
    fn from(variant: ModeB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ModeB {
    type Ux = u8;
}
impl crate::IsEnum for ModeB {}
#[doc = "Field `MODE_B` reader - Mode Select for Timer B"]
pub type ModeBR = crate::FieldReader<ModeB>;
impl ModeBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ModeB> {
        match self.bits {
            0 => Some(ModeB::OneShot),
            1 => Some(ModeB::Continuous),
            2 => Some(ModeB::Counter),
            3 => Some(ModeB::Pwm),
            4 => Some(ModeB::Capture),
            5 => Some(ModeB::Compare),
            6 => Some(ModeB::Gated),
            7 => Some(ModeB::Capcomp),
            8 => Some(ModeB::DualEdge),
            14 => Some(ModeB::Igated),
            _ => None,
        }
    }
    #[doc = "One-Shot Mode"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == ModeB::OneShot
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == ModeB::Continuous
    }
    #[doc = "Counter Mode"]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == ModeB::Counter
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == ModeB::Pwm
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == ModeB::Capture
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn is_compare(&self) -> bool {
        *self == ModeB::Compare
    }
    #[doc = "Gated Mode"]
    #[inline(always)]
    pub fn is_gated(&self) -> bool {
        *self == ModeB::Gated
    }
    #[doc = "Capture/Compare Mode"]
    #[inline(always)]
    pub fn is_capcomp(&self) -> bool {
        *self == ModeB::Capcomp
    }
    #[doc = "Dual Edge Capture Mode"]
    #[inline(always)]
    pub fn is_dual_edge(&self) -> bool {
        *self == ModeB::DualEdge
    }
    #[doc = "Inactive Gated Mode"]
    #[inline(always)]
    pub fn is_igated(&self) -> bool {
        *self == ModeB::Igated
    }
}
#[doc = "Field `MODE_B` writer - Mode Select for Timer B"]
pub type ModeBW<'a, REG> = crate::FieldWriter<'a, REG, 4, ModeB>;
impl<'a, REG> ModeBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One-Shot Mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::OneShot)
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::Continuous)
    }
    #[doc = "Counter Mode"]
    #[inline(always)]
    pub fn counter(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::Counter)
    }
    #[doc = "PWM Mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::Pwm)
    }
    #[doc = "Capture Mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::Capture)
    }
    #[doc = "Compare Mode"]
    #[inline(always)]
    pub fn compare(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::Compare)
    }
    #[doc = "Gated Mode"]
    #[inline(always)]
    pub fn gated(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::Gated)
    }
    #[doc = "Capture/Compare Mode"]
    #[inline(always)]
    pub fn capcomp(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::Capcomp)
    }
    #[doc = "Dual Edge Capture Mode"]
    #[inline(always)]
    pub fn dual_edge(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::DualEdge)
    }
    #[doc = "Inactive Gated Mode"]
    #[inline(always)]
    pub fn igated(self) -> &'a mut crate::W<REG> {
        self.variant(ModeB::Igated)
    }
}
#[doc = "Clock Divider Select for Timer B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkdivB {
    #[doc = "0: Prescaler Divide-By-1"]
    DivBy1 = 0,
    #[doc = "1: Prescaler Divide-By-2"]
    DivBy2 = 1,
    #[doc = "2: Prescaler Divide-By-4"]
    DivBy4 = 2,
    #[doc = "3: Prescaler Divide-By-8"]
    DivBy8 = 3,
    #[doc = "4: Prescaler Divide-By-16"]
    DivBy16 = 4,
    #[doc = "5: Prescaler Divide-By-32"]
    DivBy32 = 5,
    #[doc = "6: Prescaler Divide-By-64"]
    DivBy64 = 6,
    #[doc = "7: Prescaler Divide-By-128"]
    DivBy128 = 7,
    #[doc = "8: Prescaler Divide-By-256"]
    DivBy256 = 8,
    #[doc = "9: Prescaler Divide-By-512"]
    DivBy512 = 9,
    #[doc = "10: Prescaler Divide-By-1024"]
    DivBy1024 = 10,
    #[doc = "11: Prescaler Divide-By-2048"]
    DivBy2048 = 11,
    #[doc = "12: TBD"]
    DivBy4096 = 12,
}
impl From<ClkdivB> for u8 {
    #[inline(always)]
    fn from(variant: ClkdivB) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkdivB {
    type Ux = u8;
}
impl crate::IsEnum for ClkdivB {}
#[doc = "Field `CLKDIV_B` reader - Clock Divider Select for Timer B"]
pub type ClkdivBR = crate::FieldReader<ClkdivB>;
impl ClkdivBR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkdivB> {
        match self.bits {
            0 => Some(ClkdivB::DivBy1),
            1 => Some(ClkdivB::DivBy2),
            2 => Some(ClkdivB::DivBy4),
            3 => Some(ClkdivB::DivBy8),
            4 => Some(ClkdivB::DivBy16),
            5 => Some(ClkdivB::DivBy32),
            6 => Some(ClkdivB::DivBy64),
            7 => Some(ClkdivB::DivBy128),
            8 => Some(ClkdivB::DivBy256),
            9 => Some(ClkdivB::DivBy512),
            10 => Some(ClkdivB::DivBy1024),
            11 => Some(ClkdivB::DivBy2048),
            12 => Some(ClkdivB::DivBy4096),
            _ => None,
        }
    }
    #[doc = "Prescaler Divide-By-1"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == ClkdivB::DivBy1
    }
    #[doc = "Prescaler Divide-By-2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == ClkdivB::DivBy2
    }
    #[doc = "Prescaler Divide-By-4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == ClkdivB::DivBy4
    }
    #[doc = "Prescaler Divide-By-8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == ClkdivB::DivBy8
    }
    #[doc = "Prescaler Divide-By-16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == ClkdivB::DivBy16
    }
    #[doc = "Prescaler Divide-By-32"]
    #[inline(always)]
    pub fn is_div_by_32(&self) -> bool {
        *self == ClkdivB::DivBy32
    }
    #[doc = "Prescaler Divide-By-64"]
    #[inline(always)]
    pub fn is_div_by_64(&self) -> bool {
        *self == ClkdivB::DivBy64
    }
    #[doc = "Prescaler Divide-By-128"]
    #[inline(always)]
    pub fn is_div_by_128(&self) -> bool {
        *self == ClkdivB::DivBy128
    }
    #[doc = "Prescaler Divide-By-256"]
    #[inline(always)]
    pub fn is_div_by_256(&self) -> bool {
        *self == ClkdivB::DivBy256
    }
    #[doc = "Prescaler Divide-By-512"]
    #[inline(always)]
    pub fn is_div_by_512(&self) -> bool {
        *self == ClkdivB::DivBy512
    }
    #[doc = "Prescaler Divide-By-1024"]
    #[inline(always)]
    pub fn is_div_by_1024(&self) -> bool {
        *self == ClkdivB::DivBy1024
    }
    #[doc = "Prescaler Divide-By-2048"]
    #[inline(always)]
    pub fn is_div_by_2048(&self) -> bool {
        *self == ClkdivB::DivBy2048
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn is_div_by_4096(&self) -> bool {
        *self == ClkdivB::DivBy4096
    }
}
#[doc = "Field `CLKDIV_B` writer - Clock Divider Select for Timer B"]
pub type ClkdivBW<'a, REG> = crate::FieldWriter<'a, REG, 4, ClkdivB>;
impl<'a, REG> ClkdivBW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler Divide-By-1"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy1)
    }
    #[doc = "Prescaler Divide-By-2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy2)
    }
    #[doc = "Prescaler Divide-By-4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy4)
    }
    #[doc = "Prescaler Divide-By-8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy8)
    }
    #[doc = "Prescaler Divide-By-16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy16)
    }
    #[doc = "Prescaler Divide-By-32"]
    #[inline(always)]
    pub fn div_by_32(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy32)
    }
    #[doc = "Prescaler Divide-By-64"]
    #[inline(always)]
    pub fn div_by_64(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy64)
    }
    #[doc = "Prescaler Divide-By-128"]
    #[inline(always)]
    pub fn div_by_128(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy128)
    }
    #[doc = "Prescaler Divide-By-256"]
    #[inline(always)]
    pub fn div_by_256(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy256)
    }
    #[doc = "Prescaler Divide-By-512"]
    #[inline(always)]
    pub fn div_by_512(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy512)
    }
    #[doc = "Prescaler Divide-By-1024"]
    #[inline(always)]
    pub fn div_by_1024(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy1024)
    }
    #[doc = "Prescaler Divide-By-2048"]
    #[inline(always)]
    pub fn div_by_2048(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy2048)
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn div_by_4096(self) -> &'a mut crate::W<REG> {
        self.variant(ClkdivB::DivBy4096)
    }
}
#[doc = "Field `POL_B` reader - Timer Polarity for Timer B"]
pub type PolBR = crate::BitReader;
#[doc = "Field `POL_B` writer - Timer Polarity for Timer B"]
pub type PolBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSYNC_B` reader - PWM Synchronization Mode for Timer B"]
pub type PwmsyncBR = crate::BitReader;
#[doc = "Field `PWMSYNC_B` writer - PWM Synchronization Mode for Timer B"]
pub type PwmsyncBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOLHPOL_B` reader - PWM Phase A (Non-Overlapping High) Polarity for Timer B"]
pub type NolhpolBR = crate::BitReader;
#[doc = "Field `NOLHPOL_B` writer - PWM Phase A (Non-Overlapping High) Polarity for Timer B"]
pub type NolhpolBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOLLPOL_B` reader - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer B"]
pub type NollpolBR = crate::BitReader;
#[doc = "Field `NOLLPOL_B` writer - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer B"]
pub type NollpolBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMCKBD_B` reader - PWM Phase A-Prime Output Disable for Timer B"]
pub type PwmckbdBR = crate::BitReader;
#[doc = "Field `PWMCKBD_B` writer - PWM Phase A-Prime Output Disable for Timer B"]
pub type PwmckbdBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_B` reader - Resets all flip flops in the CLK_TMR domain for Timer B. Self-clears."]
pub type RstBR = crate::BitReader;
#[doc = "Field `RST_B` writer - Resets all flip flops in the CLK_TMR domain for Timer B. Self-clears."]
pub type RstBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN_B` reader - Write 1 to Enable CLK_TMR for Timer B"]
pub type ClkenBR = crate::BitReader;
#[doc = "Field `CLKEN_B` writer - Write 1 to Enable CLK_TMR for Timer B"]
pub type ClkenBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_B` reader - Enable for Timer B"]
pub type EnBR = crate::BitReader;
#[doc = "Field `EN_B` writer - Enable for Timer B"]
pub type EnBW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Mode Select for Timer A"]
    #[inline(always)]
    pub fn mode_a(&self) -> ModeAR {
        ModeAR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Clock Divider Select for Timer A"]
    #[inline(always)]
    pub fn clkdiv_a(&self) -> ClkdivAR {
        ClkdivAR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Timer Polarity for Timer A"]
    #[inline(always)]
    pub fn pol_a(&self) -> PolAR {
        PolAR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM Synchronization Mode for Timer A"]
    #[inline(always)]
    pub fn pwmsync_a(&self) -> PwmsyncAR {
        PwmsyncAR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM Phase A (Non-Overlapping High) Polarity for Timer A"]
    #[inline(always)]
    pub fn nolhpol_a(&self) -> NolhpolAR {
        NolhpolAR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer A"]
    #[inline(always)]
    pub fn nollpol_a(&self) -> NollpolAR {
        NollpolAR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM Phase A-Prime Output Disable for Timer A"]
    #[inline(always)]
    pub fn pwmckbd_a(&self) -> PwmckbdAR {
        PwmckbdAR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Resets all flip flops in the CLK_TMR domain for Timer A. Self-clears."]
    #[inline(always)]
    pub fn rst_a(&self) -> RstAR {
        RstAR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write 1 to Enable CLK_TMR for Timer A"]
    #[inline(always)]
    pub fn clken_a(&self) -> ClkenAR {
        ClkenAR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable for Timer A"]
    #[inline(always)]
    pub fn en_a(&self) -> EnAR {
        EnAR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Mode Select for Timer B"]
    #[inline(always)]
    pub fn mode_b(&self) -> ModeBR {
        ModeBR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Clock Divider Select for Timer B"]
    #[inline(always)]
    pub fn clkdiv_b(&self) -> ClkdivBR {
        ClkdivBR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Timer Polarity for Timer B"]
    #[inline(always)]
    pub fn pol_b(&self) -> PolBR {
        PolBR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PWM Synchronization Mode for Timer B"]
    #[inline(always)]
    pub fn pwmsync_b(&self) -> PwmsyncBR {
        PwmsyncBR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PWM Phase A (Non-Overlapping High) Polarity for Timer B"]
    #[inline(always)]
    pub fn nolhpol_b(&self) -> NolhpolBR {
        NolhpolBR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer B"]
    #[inline(always)]
    pub fn nollpol_b(&self) -> NollpolBR {
        NollpolBR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PWM Phase A-Prime Output Disable for Timer B"]
    #[inline(always)]
    pub fn pwmckbd_b(&self) -> PwmckbdBR {
        PwmckbdBR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Resets all flip flops in the CLK_TMR domain for Timer B. Self-clears."]
    #[inline(always)]
    pub fn rst_b(&self) -> RstBR {
        RstBR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write 1 to Enable CLK_TMR for Timer B"]
    #[inline(always)]
    pub fn clken_b(&self) -> ClkenBR {
        ClkenBR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for Timer B"]
    #[inline(always)]
    pub fn en_b(&self) -> EnBR {
        EnBR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode Select for Timer A"]
    #[inline(always)]
    pub fn mode_a(&mut self) -> ModeAW<Ctrl0Spec> {
        ModeAW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Clock Divider Select for Timer A"]
    #[inline(always)]
    pub fn clkdiv_a(&mut self) -> ClkdivAW<Ctrl0Spec> {
        ClkdivAW::new(self, 4)
    }
    #[doc = "Bit 8 - Timer Polarity for Timer A"]
    #[inline(always)]
    pub fn pol_a(&mut self) -> PolAW<Ctrl0Spec> {
        PolAW::new(self, 8)
    }
    #[doc = "Bit 9 - PWM Synchronization Mode for Timer A"]
    #[inline(always)]
    pub fn pwmsync_a(&mut self) -> PwmsyncAW<Ctrl0Spec> {
        PwmsyncAW::new(self, 9)
    }
    #[doc = "Bit 10 - PWM Phase A (Non-Overlapping High) Polarity for Timer A"]
    #[inline(always)]
    pub fn nolhpol_a(&mut self) -> NolhpolAW<Ctrl0Spec> {
        NolhpolAW::new(self, 10)
    }
    #[doc = "Bit 11 - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer A"]
    #[inline(always)]
    pub fn nollpol_a(&mut self) -> NollpolAW<Ctrl0Spec> {
        NollpolAW::new(self, 11)
    }
    #[doc = "Bit 12 - PWM Phase A-Prime Output Disable for Timer A"]
    #[inline(always)]
    pub fn pwmckbd_a(&mut self) -> PwmckbdAW<Ctrl0Spec> {
        PwmckbdAW::new(self, 12)
    }
    #[doc = "Bit 13 - Resets all flip flops in the CLK_TMR domain for Timer A. Self-clears."]
    #[inline(always)]
    pub fn rst_a(&mut self) -> RstAW<Ctrl0Spec> {
        RstAW::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to Enable CLK_TMR for Timer A"]
    #[inline(always)]
    pub fn clken_a(&mut self) -> ClkenAW<Ctrl0Spec> {
        ClkenAW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable for Timer A"]
    #[inline(always)]
    pub fn en_a(&mut self) -> EnAW<Ctrl0Spec> {
        EnAW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Mode Select for Timer B"]
    #[inline(always)]
    pub fn mode_b(&mut self) -> ModeBW<Ctrl0Spec> {
        ModeBW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Clock Divider Select for Timer B"]
    #[inline(always)]
    pub fn clkdiv_b(&mut self) -> ClkdivBW<Ctrl0Spec> {
        ClkdivBW::new(self, 20)
    }
    #[doc = "Bit 24 - Timer Polarity for Timer B"]
    #[inline(always)]
    pub fn pol_b(&mut self) -> PolBW<Ctrl0Spec> {
        PolBW::new(self, 24)
    }
    #[doc = "Bit 25 - PWM Synchronization Mode for Timer B"]
    #[inline(always)]
    pub fn pwmsync_b(&mut self) -> PwmsyncBW<Ctrl0Spec> {
        PwmsyncBW::new(self, 25)
    }
    #[doc = "Bit 26 - PWM Phase A (Non-Overlapping High) Polarity for Timer B"]
    #[inline(always)]
    pub fn nolhpol_b(&mut self) -> NolhpolBW<Ctrl0Spec> {
        NolhpolBW::new(self, 26)
    }
    #[doc = "Bit 27 - PWM Phase A-Prime (Non-Overlapping Low) Polarity for Timer B"]
    #[inline(always)]
    pub fn nollpol_b(&mut self) -> NollpolBW<Ctrl0Spec> {
        NollpolBW::new(self, 27)
    }
    #[doc = "Bit 28 - PWM Phase A-Prime Output Disable for Timer B"]
    #[inline(always)]
    pub fn pwmckbd_b(&mut self) -> PwmckbdBW<Ctrl0Spec> {
        PwmckbdBW::new(self, 28)
    }
    #[doc = "Bit 29 - Resets all flip flops in the CLK_TMR domain for Timer B. Self-clears."]
    #[inline(always)]
    pub fn rst_b(&mut self) -> RstBW<Ctrl0Spec> {
        RstBW::new(self, 29)
    }
    #[doc = "Bit 30 - Write 1 to Enable CLK_TMR for Timer B"]
    #[inline(always)]
    pub fn clken_b(&mut self) -> ClkenBW<Ctrl0Spec> {
        ClkenBW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable for Timer B"]
    #[inline(always)]
    pub fn en_b(&mut self) -> EnBW<Ctrl0Spec> {
        EnBW::new(self, 31)
    }
}
#[doc = "Timer Control Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for Ctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
