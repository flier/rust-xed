#[macro_export]
macro_rules! properties {
    () => {};
    (
        $(#[$attr:meta])* $name:ident : $ty:tt ? { $fn:ident } $($rest:tt)*
    ) => {
        properties!( __getter_opt $(#[$attr])* $name $ty $fn );
        properties!( $( $rest )* );
    };
    (
        $(#[$attr:meta])* $name:ident : $ty:tt { $fn:ident } $($rest:tt)*
    ) => {
        properties!( __getter $(#[$attr])* $name $ty $fn );
        properties!( $( $rest )* );
    };

    (
        __getter $(#[$attr:meta])* $name:ident bool $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> bool {
            unsafe { ffi::$fn(self.as_ptr()) != 0 }
        }
    };

    (
        __getter $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> $ty {
            unsafe { ffi::$fn(self.as_ptr()) }.into()
        }
    };

    (
        __getter_opt $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> Option< $ty > {
            let res = unsafe { ffi::$fn(self.as_ptr()) };

            (res != 0).then_some(res.into())
        }
    };
}

#[macro_export]
macro_rules! enum_properties {
    () => {};
    (
        $(#[$attr:meta])* $name:ident : $ty:tt ? { $fn:ident } $($rest:tt)*
    ) => {
        enum_properties!( __getter_opt $(#[$attr])* $name $ty $fn );
        enum_properties!( $( $rest )* );
    };
    (
        $(#[$attr:meta])* $name:ident : $ty:tt { $fn:ident } $($rest:tt)*
    ) => {
        enum_properties!( __getter $(#[$attr])* $name $ty $fn );
        enum_properties!( $( $rest )* );
    };

    (
        __getter $(#[$attr:meta])* $name:ident bool $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> bool {
            unsafe { ffi::$fn((*self).into()) != 0 }
        }
    };

    (
        __getter $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> $ty {
            unsafe { ffi::$fn((*self).into()) }.into()
        }
    };

    (
        __getter_opt $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> Option< $ty > {
            let res = unsafe { ffi::$fn((*self).into()) };

            (res != 0).then_some(res.into())
        }
    };
}

#[macro_export]
macro_rules! indexed_properties {
    () => {};
    (
        $(#[$attr:meta])* $name:ident : $ty:tt ? { $fn:ident } $($rest:tt)*
    ) => {
        indexed_properties!( __getter_opt $(#[$attr])* $name $ty $fn );
        indexed_properties!( $( $rest )* );
    };
    (
        $(#[$attr:meta])* $name:ident : $ty:tt { $fn:ident } $($rest:tt)*
    ) => {
        indexed_properties!( __getter $(#[$attr])* $name $ty $fn );
        indexed_properties!( $( $rest )* );
    };

    (
        __getter $(#[$attr:meta])* $name:ident bool $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> bool {
            unsafe { ffi::$fn(self.0.as_ptr(), self.1) != 0 }
        }
    };

    (
        __getter $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> $ty {
            unsafe { ffi::$fn(self.0.as_ptr(), self.1) }.into()
        }
    };

    (
        __getter_opt $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> Option<$ty> {
            let res = unsafe { ffi::$fn(self.0.as_ptr(), self.1) };

            (res != 0).then_some(res.into())
        }
    };
}
