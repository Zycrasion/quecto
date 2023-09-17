macro_rules! import {
    ($mod:ident) => {
        mod $mod;
        pub use $mod::*;
    };
}

import!(asm);
import!(system_calls);
import!(program);