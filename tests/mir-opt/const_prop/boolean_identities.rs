// unit-test: ConstProp
// compile-flags: -O -Zmir-opt-level=4

// EMIT_MIR boolean_identities.test.ConstProp.diff
pub fn test(x: bool, y: bool) -> bool {
    // Verify that boolean operators are solved (if possible)
    // CHECK-LABEL: fn test(
    // CHECK: {{_[0-9]+}} = const false;
    (y | true) & (x & false)
}

fn main() {
    test(true, false);
}
