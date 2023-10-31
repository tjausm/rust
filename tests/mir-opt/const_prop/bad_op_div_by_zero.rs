// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// unit-test: ConstProp
// EMIT_MIR bad_op_div_by_zero.main.ConstProp.diff
#[allow(unconditional_panic)]
fn main() {
    // Verify that zero division is propagated to a panic
    // CHECK-LABEL: fn main(
    // CHECK: assert(!const true,
    let y = 0;
    let _z = 1 / y;
}
