// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// unit-test: ConstProp
// compile-flags: -C overflow-checks=on

// EMIT_MIR indirect.main.ConstProp.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: {{_[0-9]+}} = const 3_u8;
    // CHECK-NOT: {{_[0-9]+}} = CheckedAdd({{_[0-9]+}}, const 1_u8);
    let x = (2u32 as u8) + 1;
}
