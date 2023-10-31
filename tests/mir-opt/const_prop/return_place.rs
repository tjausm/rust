// unit-test: ConstProp
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// compile-flags: -C overflow-checks=on

// EMIT_MIR return_place.add.ConstProp.diff
// EMIT_MIR return_place.add.PreCodegen.before.mir
fn add() -> u32 {
    // CHECK-LABEL: fn add(
    // CHECK: {{_[0-9]+}} = const 4_u32;
    // CHECK-NEXT: return;
    2 + 2
}

fn main() {
    add();
}
