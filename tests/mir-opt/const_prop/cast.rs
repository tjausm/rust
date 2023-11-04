// unit-test: ConstProp
// EMIT_MIR cast.main.ConstProp.diff

fn main() {
    // CHECK-LABEL: fn main(
    // CHECK: {{_[0-9]+}} = const 42_u32;
    // CHECK: {{_[0-9]+}} = const 42_u8;
    let x = 42u8 as u32;

    let y = 42u32 as u8;
}
