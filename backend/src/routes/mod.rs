macro_rules! import_same_name {
    ($($e:ident),*) => {
        $(
            mod $e;
            pub use $e::$e;
        )*
    }
}


pub mod settings;

import_same_name!(create, delete, get, get_all, get_logo, healthcheck, search, update, upload_logo, whoami, login, changes);
