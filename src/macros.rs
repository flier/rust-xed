#[macro_export]
macro_rules! impl_as_ptr {
    ( $name:ident ( $ctype:ty ) ) => {
        impl $crate::raw::AsPtr for $name {
            type CType = $ctype;

            fn as_ptr(&self) -> *const Self::CType {
                &self.0 as *const _
            }
        }

        impl $crate::raw::AsMutPtr for $name {
            fn as_mut_ptr(&mut self) -> *mut Self::CType {
                &mut self.0 as *mut _
            }
        }
    };
}

#[macro_export]
macro_rules! properties {
    () => {};
    ( prefix = $prefix:ident ; ) => {};

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : $ty:tt { $(#[$get_attr:meta])* get; $(#[$set_attr:meta])* set; } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter $(#[$attr])* $(#[$get_attr])* $name $ty [< $prefix _get_ $name >] );
            properties!( __setter $(#[$attr])* $(#[$set_attr])* [< set_ $name >] $ty [< $prefix _set_ $name >] );
        }
        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : $ty:tt ? { $(#[$get_attr:meta])* get; $(#[$set_attr:meta])* set; } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter_opt $(#[$attr])* $(#[$get_attr])* $name $ty [< $prefix _get_ $name >] );
            properties!( __setter $(#[$attr])* $(#[$set_attr])* [< set_ $name >] $ty [< $prefix _set_ $name >] );
        }
        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : & $ty:tt { get; } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter_cast $(#[$attr])* $name $ty [< $prefix _get_ $name >] );
        }

        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : & $ty:tt ? { get; } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter_cast_opt $(#[$attr])* $name $ty [< $prefix _get_ $name >] );
        }

        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : $ty:tt { get; } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter $(#[$attr])* $name $ty [< $prefix _get_ $name >] );
        }

        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : & $ty:tt { $getter:ident } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter_cast $(#[$attr])* $name $ty $getter );
        }
        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : $ty:tt ? { $getter:ident } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter_opt $(#[$attr])* $name $ty $getter );
        }
        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : & $ty:tt ? { $getter:ident } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter_cast_opt $(#[$attr])* $name $ty $getter );
        }
        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : $ty:tt { $getter:ident } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter $(#[$attr])* $name $ty $getter );
        }
        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident { set; } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __setter_flag $(#[$attr])* [< set_ $name >] [< $prefix _set_ $name >] );
        }
        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : $ty:tt { set; } $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __setter $(#[$attr])* [< set_ $name >] $ty [< $prefix _set_ $name >] );
        }

        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        prefix = $prefix:ident ;
        $(#[$attr:meta])* $name:ident : $ty:tt $($rest:tt)*
    ) => {
        paste::paste! {
            properties!( __getter $(#[$attr])* $name $ty [< $prefix _ $name >] );
        }

        properties!( prefix = $prefix ; $( $rest )* );
    };

    (
        $(#[$attr:meta])* $name:ident : & $ty:tt ? { $fn:ident } $($rest:tt)*
    ) => {
        properties!( __getter_cast_opt $(#[$attr])* $name $ty $fn );
        properties!( $( $rest )* );
    };
    (
        $(#[$attr:meta])* $name:ident : & $ty:tt { $fn:ident } $($rest:tt)*
    ) => {
        properties!( __getter_cast $(#[$attr])* $name $ty $fn );
        properties!( $( $rest )* );
    };
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
            unsafe { ffi::$fn($crate::raw::AsPtr::as_ptr(self)) != 0 }
        }
    };
    (
        __getter $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> $ty {
            unsafe { ffi::$fn($crate::raw::AsPtr::as_ptr(self)) }.into()
        }
    };

    (
        __getter_opt $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> Option< $ty > {
            let res = unsafe { ffi::$fn($crate::raw::AsPtr::as_ptr(self)) };

            (res != 0).then_some(res.into())
        }
    };

    (
        __getter_cast $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> & $ty {
            unsafe {
                ffi::$fn($crate::raw::AsPtr::as_ptr(self))
                    .cast::<$ty>()
                    .as_ref()
                    .unwrap()
            }
        }
    };

    (
        __getter_cast_opt $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> Option<& $ty> {
            unsafe {
                ffi::$fn($crate::raw::AsPtr::as_ptr(self))
                    .cast::<$ty>()
                    .as_ref()
            }
        }
    };

    (
        __setter $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name (&mut self, v: $ty) -> &mut Self {
            unsafe { ffi::$fn($crate::raw::AsMutPtr::as_mut_ptr(self), v.into()); }

            self
        }
    };

    (
        __setter_flag $(#[$attr:meta])* $name:ident $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name (&mut self) -> &mut Self {
            unsafe { ffi::$fn($crate::raw::AsMutPtr::as_mut_ptr(self)); }

            self
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
            unsafe { ffi::$fn($crate::raw::AsPtr::as_ptr(self.0), self.1) != 0 }
        }
    };

    (
        __getter $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> $ty {
            unsafe { ffi::$fn($crate::raw::AsPtr::as_ptr(self.0), self.1) }.into()
        }
    };

    (
        __getter_opt $(#[$attr:meta])* $name:ident $ty:tt $fn:ident
    ) => {
        $(#[$attr])*
        pub fn $name(&self) -> Option<$ty> {
            let res = unsafe { ffi::$fn($crate::raw::AsPtr::as_ptr(self.0), self.1) };

            (res != 0).then_some(res.into())
        }
    };
}
