/*
Author: Ben Mezger (github.com/benmezger)
 */

cfg_if::cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        macro define_inst_macros() {r"
            .ifndef inst_macros_defined
                .set inst_macros_defined, 1

                .macro STORE_X p:vararg
                    sd \p
                .endm

                .macro LOAD_X p:vararg
                    lw \p
                .endm

                .macro WORD_SIZE p:vararg
                    8 \p
                .endm
            .endif
        "}
    }
    else {
        macro define_inst_macros() {r"
            .ifndef inst_macros_defined
                .set inst_macros_defined, 1

                .macro STORE_X p:vararg
                    sw \p
                .endm

                .macro LOAD_X p:vararg
                    lw \p
                .endm

                .macro WORD_SIZE p:vararg
                    4 \p
                .endm
            .endif
        "}
    }
}

global_asm!(concat!(define_inst_macros!(), include_str!("asm/macros.S")));
global_asm!(concat!(define_inst_macros!(), include_str!("asm/trap.S")));
global_asm!(concat!(define_inst_macros!(), include_str!("asm/user.S")));
global_asm!(concat!(define_inst_macros!(), include_str!("asm/boot.S")));
