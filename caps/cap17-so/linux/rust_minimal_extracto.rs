// SPDX-License-Identifier: GPL-2.0
struct RustMinimal {
    numbers: KVec<i32>,
}

impl kernel::Module for RustMinimal {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust minimal sample (init)\n");
        // [...]
        let mut numbers = KVec::new();
        numbers.push(72, GFP_KERNEL)?;
        // [...]
        Ok(RustMinimal { numbers })
    }
}
