use syn::{GenericParam, Generics, Lifetime, LifetimeParam, spanned::Spanned};

pub fn lt(mut generics: Generics) -> (Generics, Lifetime) {
    match generics.lifetimes().next() {
        Some(some) => {
            let lt = some.lifetime.clone();
            (generics, lt)
        }
        None => {
            let lt = Lifetime::new("'a", generics.span());

            generics
                .params
                .insert(0, GenericParam::Lifetime(LifetimeParam::new(lt.clone())));

            (generics, lt)
        }
    }
}
