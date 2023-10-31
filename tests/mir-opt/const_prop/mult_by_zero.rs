// unit-test: ConstProp

// EMIT_MIR mult_by_zero.test.ConstProp.diff
fn test(x: i32) -> i32 {
    // Verify that zero multiplication propagates to 0
    // CHECK-LABEL: fn test(
    // CHECK:  bb0: {
    // CHECK:     {{_[0-9]+}}  = const 0_i32;
    // CHECK:     return;
    // CHECK:  }
    // CHECK: }
    x * 0
}

fn main() {
    test(10);
}
