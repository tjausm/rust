// EMIT_MIR_FOR_EACH_PANIC_STRATEGY
// unit-test: ConstProp
// compile-flags: -Zmir-enable-passes=+Inline

// EMIT_MIR inherit_overflow.main.ConstProp.diff
fn main() {
    // After inlining, this will contain a `CheckedBinaryOp`.
    // Propagating the overflow is ok as codegen will just skip emitting the panic.

    // CHECK-LABEL: fn main(
    // CHECK: {{_[0-9]+}} = <u8 as Add>::add(const u8::MAX, const 1_u8) -> {{bb[0-9]+}};
    let _ = <u8 as std::ops::Add>::add(255, 1);
}
