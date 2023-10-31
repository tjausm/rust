// unit-test: ConstProp
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// EMIT_MIR_FOR_EACH_BIT_WIDTH

// EMIT_MIR repeat.main.ConstProp.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: {{_[0-9]+}} = const 42_u32;
    let x: u32 = [42; 8][2] + 0;
}
