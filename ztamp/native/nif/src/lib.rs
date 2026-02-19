//! NIF bridge between the Elixir runtime and the `rztamp` Rust library.

mod atoms {
    rustler::atoms! {
        ok,
    }
}

/// No-operation stub used to verify that the NIF is loaded and callable.
#[rustler::nif]
fn nop() -> rustler::Atom {
    atoms::ok()
}

/// Verify that the `rztamp` library is linked and accessible.
#[rustler::nif]
fn alive() -> bool {
    rztamp::alive()
}

rustler::init!("Elixir.Ztamp.Nif");
