/// Writes `val` to `addr`. Used to apply Errata workarounds.
unsafe fn poke(addr: u32, val: u32) {
    (addr as *mut u32).write_volatile(val);
}

/// Reads 32 bits from `addr`.
unsafe fn peek(addr: u32) -> u32 {
    (addr as *mut u32).read_volatile()
}

pub fn pre_enable() {
    // Works around Erratum 187 on chip revisions 1, 2 and 3.
    unsafe {
        poke(0x4006EC00, 0x00009375);
        poke(0x4006ED14, 0x00000003);
        poke(0x4006EC00, 0x00009375);
    }

    pre_wakeup();
}

pub fn post_enable() {
    post_wakeup();

    // Works around Erratum 187 on chip revisions 1, 2 and 3.
    unsafe {
        poke(0x4006EC00, 0x00009375);
        poke(0x4006ED14, 0x00000000);
        poke(0x4006EC00, 0x00009375);
    }

    // Works around Erratum 166 on chip revisions 1, 2 and 3.
    // fixes problem with double buffering
    const NRF_USBD_BASE: u32 = 0x40027000;
    unsafe {
        poke(NRF_USBD_BASE + 0x800, 0x7E3);
        poke(NRF_USBD_BASE + 0x804, 0x40);
    }
}

pub fn pre_wakeup() {
    // Works around Erratum 171 on chip revisions 1, 2 and 3.

    unsafe {
        if peek(0x4006EC00) == 0x00000000 {
            poke(0x4006EC00, 0x00009375);
        }

        poke(0x4006EC14, 0x000000C0);
        poke(0x4006EC00, 0x00009375);
    }
}

pub fn post_wakeup() {
    // Works around Erratum 171 on chip revisions 1, 2 and 3.

    unsafe {
        if peek(0x4006EC00) == 0x00000000 {
            poke(0x4006EC00, 0x00009375);
        }

        poke(0x4006EC14, 0x00000000);
        poke(0x4006EC00, 0x00009375);
    }
}

pub fn dma_start() {
    // Works around Erratum 199 on chip revisions 1, 2 and 3.

    unsafe { poke(0x40027C1C, 0x00000082) }
}

pub fn dma_complete() {
    // Works around Erratum 199 on chip revisions 1, 2 and 3.
    unsafe { poke(0x40027C1C, 0x00000000) }
}
