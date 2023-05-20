#![feature(specialization)]
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

impl<T> SafeLayout for T {
    default fn get_layout(&self) -> Layout {
        println!("generic");
        Layout {
            address: self.deref() as *const _ as usize,
            size: std::mem::size_of::<Self>(),
        }
    }
}

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
        unsafe { &*Self::PTR }
    }
}

struct Resources {
    cpuid: CPUID,
}

impl SafeLayout for CPUID {
    default fn get_layout(&self) -> Layout {
        println!("custom");
        Layout {
            address: self.deref() as *const _ as usize,
            size: std::mem::size_of::<Self>(),
        }
    }
}

fn main() {
    let p: u32 = 0;
    let l = p.get_layout();
    println!("{:x?}", l);

    let p = Resources {
        cpuid: CPUID {
            _marker: PhantomData,
        },
    };
    let l = p.cpuid.get_layout();
    println!("{:x?}", l);
}
