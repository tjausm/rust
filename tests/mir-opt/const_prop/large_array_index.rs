// unit-test: ConstProp
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// EMIT_MIR_FOR_EACH_BIT_WIDTH

// EMIT_MIR large_array_index.main.ConstProp.diff
fn main() {
    // check that we don't propagate this, because it's too large
    // CHECK-LABEL: fn main(
    // CHECK-NOT: {{_[0-9]+}} = const 0_u32;
    let x: u8 = [0_u8; 5000][2];
}
