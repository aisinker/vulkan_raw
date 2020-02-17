use std::fmt::{Display, Formatter, Error};

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ApiVersion(u32);

impl ApiVersion {

    #[inline(always)]
    pub fn new(major: u32, minor: u32, patch: u32)-> ApiVersion {
        ApiVersion(
            (major << 22) | ((minor & 0x0000_03FF) << 12) | (patch & 0x0000_0FFF)
        )
    }

    #[inline(always)]
    pub fn major(&self)->u32{
        (self.0 & 0xFFC0_0000) >> 22
    }

    #[inline(always)]
    pub fn minor(&self)->u32{
        (self.0 & 0x003F_F000) >> 12
    }

    #[inline(always)]
    pub fn patch(&self)->u32{
        self.0 & 0x0000_0FFF
    }
}

impl From<u32> for ApiVersion{
    #[inline(always)]
    fn from(api_version: u32) -> Self {
        ApiVersion(api_version)
    }
}

impl Into<u32> for ApiVersion{
    #[inline(always)]
    fn into(self) -> u32 {
        self.0
    }
}

impl Display for ApiVersion {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.patch())
    }
}
