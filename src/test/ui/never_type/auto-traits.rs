// check-pass

#![feature(optin_builtin_traits)]

fn main() {
    enum Void {}

    auto trait Auto {}
    fn assert_auto<T: Auto>() {}
    assert_auto::<Void>();
    assert_auto::<!>();

    fn assert_send<T: Send>() {}
    assert_send::<Void>();
    assert_send::<!>();
}
