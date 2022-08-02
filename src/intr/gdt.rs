use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

struct Selectors {
    cs: SegmentSelector,
    ts: SegmentSelector,
}

pub const DOUBLE_FAULT_IST_IDX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_IDX as usize] = {
            const STACKSIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACKSIZE] = [0; STACKSIZE];
            let start_stack = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = start_stack + STACKSIZE;
            stack_end
        };
        tss
    };
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let cs = gdt.add_entry(Descriptor::kernel_code_segment());
        let ts = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { cs, ts })
    };
}

pub fn gdt_init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.cs);
        load_tss(GDT.1.ts);
    }
}
