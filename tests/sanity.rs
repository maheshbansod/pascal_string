use pascal_string::pascal_string;

#[test]
fn sanity() {
    let my_rusty_ident = pascal_string!(my_rusty_ident);
    assert_eq!(my_rusty_ident, "MyRustyIdent");
}
