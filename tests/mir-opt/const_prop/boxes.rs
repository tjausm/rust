// unit-test: ConstProp
// compile-flags: -O
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY

#![feature(rustc_attrs, stmt_expr_attributes)]

// Note: this test verifies that we, in fact, do not const prop `#[rustc_box]`

// EMIT_MIR boxes.main.ConstProp.diff
fn main() {
    // CHECK-LABEL: fn main(
    // CHECK-NOT: {{_[0-9]+}} = const 43_i32;
    let x = *(#[rustc_box]
    Box::new(42))
        + 0;
}
