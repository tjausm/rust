// unit-test: ConstProp
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// EMIT_MIR bad_op_mod_by_zero.main.ConstProp.diff
#[allow(unconditional_panic)]
fn main() {
    // Verify that zero modulo is propagated to a panic
    // CHECK-LABEL: fn main(
    // CHECK: assert(!const true,
    let y = 0;
    let _z = 1 % y;
}
