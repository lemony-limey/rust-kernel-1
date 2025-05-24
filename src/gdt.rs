use core::ptr::addr_of;
use lazy_static::lazy_static;
use x86_64::{
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable},
        tss::TaskStateSegment,
    },
    VirtAddr,
};
use x86_64::structures::gdt::{SegmentSelector};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            
            let stack_start = VirtAddr::from_ptr(unsafe { addr_of!(STACK) });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let k_code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let k_data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        gdt.add_entry(Descriptor::UserSegment(0));
        gdt.add_entry(Descriptor::user_code_segment());
        gdt.add_entry(Descriptor::user_data_segment());

        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector: k_code_selector,
                data_selector: k_data_selector,
                tss_selector
            }
        )
    };
}

struct Selectors
{
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init()
{
    use x86_64::instructions::{
        tables::load_tss,
        segmentation::{Segment},
    };
    
    GDT.0.load();
    
    unsafe {
        use x86_64::instructions::{segmentation, tables};
        
        segmentation::CS::set_reg(GDT.1.code_selector);
        segmentation::DS::set_reg(GDT.1.data_selector);
        segmentation::ES::set_reg(GDT.1.data_selector);
        segmentation::FS::set_reg(GDT.1.data_selector);
        segmentation::GS::set_reg(GDT.1.data_selector);
        segmentation::SS::set_reg(GDT.1.data_selector);
        
        load_tss(GDT.1.tss_selector);
    }
}
