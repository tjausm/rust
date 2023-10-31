// skip-filecheck
// unit-test: ConstProp

// EMIT_MIR mutable_variable_aggregate_mut_ref.main.ConstProp.diff
fn main() {
    // Const prop aggregates even if partially or fully modified
    let mut x = (42, 43);
    let z = &mut x;
    z.1 = 99;
    let y = x;
}
