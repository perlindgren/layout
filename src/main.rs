// Experiment on safe layout for data types used for RTIC resources
#![feature(negative_bounds)]

use std::marker::PhantomData;
use std::ops::{self, Deref};

#[derive(Debug)]
struct Layout {
    address: usize,
    size: usize,
}
trait SafeLayout {
    fn get_layout(&self) -> Layout;
}

trait CustomLayout {}

/// CPUID (simplified version from Cortex-m)
pub struct CPUID {
    _marker: PhantomData<*const ()>,
}

mod cpuid {
    pub struct RegisterBlock {
        pub base: u32,
        _reserved0: [u32; 15],
        pub pfr: u32,
    }
}
unsafe impl Send for CPUID {}
unsafe impl Sync for CPUID {}
impl CPUID {
    /// Pointer to the register block
    pub const PTR: *const self::cpuid::RegisterBlock = 0xE000_ED00 as *const _;
}

impl ops::Deref for CPUID {
    type Target = cpuid::RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        println!("---------");
        unsafe { &*Self::PTR }
    }
}

struct Resources {
    cpuid: CPUID,
}

impl CustomLayout for CPUID {}

// thin pointer
impl<T> SafeLayout for T
where
    T: !CustomLayout,
{
    fn get_layout(&self) -> Layout {
        Layout {
            address: self.deref() as *const _ as usize,
            size: std::mem::size_of::<Self>(),
        }
    }
}

// fat pointer
impl SafeLayout for CPUID {
    fn get_layout(&self) -> Layout {
        Layout {
            address: self.deref() as *const _ as usize,
            size: std::mem::size_of::<cpuid::RegisterBlock>(),
        }
    }
}

fn main() {
    let p = Resources {
        cpuid: CPUID {
            _marker: PhantomData,
        },
    };

    let l = p.cpuid.get_layout();
    println!("{:x}", l.address);
    println!("{}", l.size);

    let d: i32 = 0;
    let l_native = d.get_layout();
    println!("{:x}", l_native.address);
    println!("{}", l_native.size);
}
