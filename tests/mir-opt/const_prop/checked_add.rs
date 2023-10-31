// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// unit-test: ConstProp
// compile-flags: -C overflow-checks=on

// EMIT_MIR checked_add.main.ConstProp.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: = const 2_u32;
    // CHECK-NOT: = CheckedAdd(_1, const 1_u32);
    let x: u32 = 1 + 1;
}
