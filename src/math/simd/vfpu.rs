macro_rules! vfpu_asm_return {
    ($($l:literal),*,$($i:ident$(.$e:expr)?),*,) => {
        unsafe {
            let mut o = core::mem::MaybeUninit::uninit();
            psp_vfpu::vfpu_asm!(
                $($l,)*
                in(reg) (&mut o),
                $(in(reg) (&$i$(.$e)*),)*
                options(nostack),
            );
            o.assume_init()
        }
    };
}
pub(crate) use vfpu_asm_return;
