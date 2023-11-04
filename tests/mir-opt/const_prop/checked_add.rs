// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// unit-test: ConstProp
// compile-flags: -C overflow-checks=on

// EMIT_MIR checked_add.main.ConstProp.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: {{_[0-9]+}} = const 2_u32;
    // CHECK-NOT: {{_[0-9]+}} = CheckedAdd({{_[0-9]+}} , const 1_u32);
    let x: u32 = 1 + 1;
}
