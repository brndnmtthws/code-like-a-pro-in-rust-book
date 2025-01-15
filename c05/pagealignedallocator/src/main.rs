#![feature(allocator_api)]

use lazy_static::lazy_static;
use std::alloc::{AllocError, Allocator, Layout};
use std::ptr;

fn mprotect_readwrite(data: &[u8]) -> Result<(), std::io::Error> {
    if data.is_empty() {
        // no-op
        return Ok(());
    }
    #[cfg(unix)]
    {
        use libc::{c_void, mprotect as c_mprotect, PROT_READ, PROT_WRITE};
        let ret = unsafe {
            c_mprotect(
                data.as_ptr() as *mut c_void,
                data.len() - 1,
                PROT_READ | PROT_WRITE,
            )
        };
        match ret {
            0 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }
    #[cfg(windows)]
    {
        use winapi::shared::minwindef::{DWORD, LPVOID};
        use winapi::um::memoryapi::VirtualProtect;
        use winapi::um::winnt::PAGE_READWRITE;

        let mut old: DWORD = 0;

        let res = unsafe {
            VirtualProtect(
                data.as_ptr() as LPVOID,
                data.len() - 1,
                PAGE_READWRITE,
                &mut old,
            )
        };
        match res {
            1 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }
}

fn mprotect_noaccess(data: &[u8]) -> Result<(), std::io::Error> {
    if data.is_empty() {
        // no-op
        return Ok(());
    }
    #[cfg(unix)]
    {
        use libc::{c_void, mprotect as c_mprotect, PROT_NONE};
        let ret = unsafe {
            c_mprotect(
                data.as_ptr() as *mut c_void,
                data.len() - 1,
                PROT_NONE,
            )
        };
        match ret {
            0 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }
    #[cfg(windows)]
    {
        use winapi::shared::minwindef::{DWORD, LPVOID};
        use winapi::um::memoryapi::VirtualProtect;
        use winapi::um::winnt::PAGE_NOACCESS;

        let mut old: DWORD = 0;

        let res = unsafe {
            VirtualProtect(
                data.as_ptr() as LPVOID,
                data.len() - 1,
                PAGE_NOACCESS,
                &mut old,
            )
        };
        match res {
            1 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }
}

/// Custom page-aligned allocator implementation. Creates blocks of
/// page-aligned heap-allocated memory regions, with a no-access pages
/// before and after the allocated region of memory.
pub struct PageAlignedAllocator;

lazy_static! {
    static ref PAGESIZE: usize = {
        #[cfg(unix)]
        {
            use libc::{sysconf, _SC_PAGE_SIZE};
            unsafe { sysconf(_SC_PAGE_SIZE) as usize }
        }
        #[cfg(windows)]
        {
            use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
            let mut si = SYSTEM_INFO::default();
            unsafe { GetSystemInfo(&mut si) };
            si.dwPageSize as usize
        }
    };
}

fn _page_round(size: usize, pagesize: usize) -> usize {
    size + (pagesize - size % pagesize)
}

unsafe impl Allocator for PageAlignedAllocator {
    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<ptr::NonNull<[u8]>, AllocError> {
        let pagesize = *PAGESIZE;
        let size = _page_round(layout.size(), pagesize) + 2 * pagesize;
        #[cfg(unix)]
        let out = {
            use libc::posix_memalign;
            let mut out = ptr::null_mut();

            // allocate full pages, in addition to an extra page at the
            // start and end which will remain locked with no
            // access permitted.
            let ret = unsafe { posix_memalign(&mut out, pagesize, size) };
            if ret != 0 {
                return Err(AllocError);
            }

            out
        };
        #[cfg(windows)]
        let out = {
            use winapi::um::memoryapi::VirtualAlloc;
            use winapi::um::winnt::{
                MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE,
            };
            unsafe {
                VirtualAlloc(
                    ptr::null_mut(),
                    size,
                    MEM_COMMIT | MEM_RESERVE,
                    PAGE_READWRITE,
                )
            }
        };

        // lock the pages at the fore of the region
        let fore_protected_region = unsafe {
            std::slice::from_raw_parts_mut(out as *mut u8, pagesize)
        };
        mprotect_noaccess(fore_protected_region)
            .map_err(|err| {
                eprintln!("mprotect error = {:?}, in allocator", err)
            })
            .ok();

        // lock the pages at the aft of the region
        let aft_protected_region_offset =
            pagesize + _page_round(layout.size(), pagesize);
        let aft_protected_region = unsafe {
            std::slice::from_raw_parts_mut(
                out.add(aft_protected_region_offset) as *mut u8,
                pagesize,
            )
        };
        mprotect_noaccess(aft_protected_region)
            .map_err(|err| {
                eprintln!("mprotect error = {:?}, in allocator", err)
            })
            .ok();

        let slice = unsafe {
            std::slice::from_raw_parts_mut(
                out.add(pagesize) as *mut u8,
                layout.size(),
            )
        };

        mprotect_readwrite(slice)
            .map_err(|err| {
                eprintln!("mprotect error = {:?}, in allocator", err)
            })
            .ok();

        unsafe { Ok(ptr::NonNull::new_unchecked(slice)) }
    }

    unsafe fn deallocate(&self, ptr: ptr::NonNull<u8>, layout: Layout) {
        let pagesize = *PAGESIZE;

        let ptr = ptr.as_ptr().offset(-(pagesize as isize));

        // unlock the fore protected region
        let fore_protected_region =
            std::slice::from_raw_parts_mut(ptr, pagesize);
        mprotect_readwrite(fore_protected_region)
            .map_err(|err| eprintln!("mprotect error = {:?}", err))
            .ok();

        // unlock the aft protected region
        let aft_protected_region_offset =
            pagesize + _page_round(layout.size(), pagesize);
        let aft_protected_region = std::slice::from_raw_parts_mut(
            ptr.add(aft_protected_region_offset),
            pagesize,
        );

        mprotect_readwrite(aft_protected_region)
            .map_err(|err| eprintln!("mprotect error = {:?}", err))
            .ok();

        #[cfg(unix)]
        {
            libc::free(ptr as *mut libc::c_void);
        }
        #[cfg(windows)]
        {
            use winapi::shared::minwindef::LPVOID;
            use winapi::um::memoryapi::VirtualFree;
            use winapi::um::winnt::MEM_RELEASE;
            VirtualFree(ptr as LPVOID, 0, MEM_RELEASE);
        }
    }
}

fn main() {
    let mut custom_alloc_vec: Vec<i32, _> =
        Vec::with_capacity_in(10, PageAlignedAllocator);
    for i in 0..10 {
        custom_alloc_vec.push(i + 1);
    }
    println!("custom_alloc_vec={:?}", custom_alloc_vec);
}
