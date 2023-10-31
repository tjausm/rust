// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// unit-test: ConstProp
// compile-flags: -Zmir-opt-level=1

trait NeedsDrop: Sized {
    const NEEDS: bool = std::mem::needs_drop::<Self>();
}

impl<This> NeedsDrop for This {}

// EMIT_MIR control_flow_simplification.hello.ConstProp.diff
// EMIT_MIR control_flow_simplification.hello.PreCodegen.before.mir
fn hello<T>(){
    if <bool>::NEEDS {
        panic!()
    }
}

pub fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: _1 = hello::<()>() -> [return: bb1, unwind continue];
    // CHECK: _2 = hello::<Vec<()>>() -> [return: bb2, unwind continue];
    // CHECK: return;
    hello::<()>();
    hello::<Vec<()>>();
}
