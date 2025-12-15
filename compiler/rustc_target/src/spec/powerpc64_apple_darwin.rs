use crate::abi::Endian;
use crate::spec::{LinkerFlavor, Target, TargetOptions};

pub fn target() -> Target {
    let mut base = super::apple_base::opts("macos");
    base.cpu = "ppc64".to_string();
    base.max_atomic_width = Some(64);
    base.pre_link_args.insert(LinkerFlavor::Gcc, vec!["-arch".to_string(), "ppc64".to_string()]);
    base.link_env_remove.extend(super::apple_base::macos_link_env_remove());
    base.eliminate_frame_pointer = false;

    let arch = "powerpc64";
    let llvm_target = super::apple_base::macos_llvm_target(&arch);

    Target {
        llvm_target,
        pointer_width: 64,
        data_layout: "E-m:o-i64:64-n32:64".to_string(),
        arch: arch.to_string(),
        options: TargetOptions { endian: Endian::Big, mcount: "\u{1}mcount".to_string(), ..base },
    }
}
