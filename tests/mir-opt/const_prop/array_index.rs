// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// unit-test: ConstProp
// EMIT_MIR_FOR_EACH_BIT_WIDTH

// EMIT_MIR array_index.main.ConstProp.diff
fn main() {
    // Verify that we use result of propagation to index arrays
    // CHECK-LABEL: fn main(
    // CHECK:  scope 1 {
    // CHECK:     debug x => _1;
    // CHECK:  }

    // CHECK:  bb0: {
    // CHECK:     _1 = const 2_u32;
    // CHECK:     return;
    // CHECK:  }
    // CHECK: }
    let x: u32 = [0, 1, 2, 3][2];
}
