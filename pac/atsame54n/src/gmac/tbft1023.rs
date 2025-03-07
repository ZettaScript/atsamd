#[doc = "Register `TBFT1023` reader"]
pub struct R(crate::R<TBFT1023_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBFT1023_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBFT1023_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBFT1023_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFTX` reader - 512 to 1023 Byte Frames Transmitted without Error"]
pub struct NFTX_R(crate::FieldReader<u32, u32>);
impl NFTX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        NFTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFTX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - 512 to 1023 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new(self.bits as u32)
    }
}
#[doc = "512 to 1023 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbft1023](index.html) module"]
pub struct TBFT1023_SPEC;
impl crate::RegisterSpec for TBFT1023_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbft1023::R](R) reader structure"]
impl crate::Readable for TBFT1023_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBFT1023 to value 0"]
impl crate::Resettable for TBFT1023_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
