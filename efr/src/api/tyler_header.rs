use efr_macros::xml;

pub(crate) struct TylerHeader<'a> {
    pub(crate) email: &'a str,
    pub(crate) password_hash: &'a str,
}

xml! {
    impl<'a> Xml for TylerHeader<'a> {
        @ns {
            tyler = "urn:tyler:efm:services";
        }

        tyler:UserNameHeader {
            UserName { (self.email) }
            Password { (self.password_hash) }
        }
    }
}
