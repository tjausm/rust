// unit-test: ConstProp
// compile-flags: -Zmir-enable-passes=+SimplifyConstCondition-after-const-prop
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
#[inline(never)]
fn foo(_: i32) { }

// EMIT_MIR switch_int.main.ConstProp.diff
// EMIT_MIR switch_int.main.SimplifyConstCondition-after-const-prop.diff
fn main() {
    // CHECK_LABEL: fn main(
    // CHECK: bb0: {
    // CHECK: _0 = foo(const 0_i32) -> bb1;
    // CHECK: }
    // CHECK: bb0: {
    // CHECK: return;
    // CHECK: }
    match 1 {
        1 => foo(0),
        _ => foo(-1),
    }
}
