#[macro_export]
macro_rules! Wrap {

    // Simple struct
    ($origin:ident, $wrapper:ident
        $(, derive = { $($derive:path),* })?)=>{
        #[doc = concat!("A NewType Wrapper around `", stringify!($origin), "`")]
        $(#[derive($($derive),*)])?
        struct $wrapper($origin);

        impl From<$origin> for $wrapper {
            fn from(v: $origin) -> Self {
                Self(v)
            }
        }

        impl From<$wrapper> for $origin {
            fn from(v: $wrapper) -> Self {
                v.0
            }
        }

        impl core::ops::Deref for $wrapper {
            type Target = $origin;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    // Have generic with constraints
    ($origin:ident<
        $($generic:ident $($(:)?$(+)?$gconstraints: path)*),*
        >, $wrapper:ident
        $(, derive = { $($derive:path),* })?)=>{
        #[doc = concat!("A NewType Wrapper around `", stringify!($origin), "`")]
        $(#[derive($($derive),*)])?
        struct $wrapper<$($generic),*>( $origin<$($generic),*> )
            where $($generic:$($gconstraints +)*),*;

        impl<$($generic: $($gconstraints +)*),*> From<$origin<$($generic),*>> for $wrapper<$($generic),*> {
            fn from(v: $origin<$($generic),*>) -> Self {
                Self(v)
            }
        }

        impl<$($generic: $($gconstraints +)*),*> From<$wrapper<$($generic),*>> for $origin<$($generic),*> {
            fn from(v: $wrapper<$($generic),*>) -> Self {
                v.0
            }
        }

        impl<$($generic: $($gconstraints +)*),*> core::ops::Deref for $wrapper<$($generic),*> {
            type Target = $origin<$($generic),*>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$($generic: $($gconstraints +)*),*> core::ops::DerefMut for $wrapper<$($generic),*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    // Have lifetime with constraints
    ($origin:ident<
        $($lifetime:lifetime $($(:)?$(+)?$lconstraints: lifetime)*),*>, $wrapper:ident
        $(, derive = { $($derive:path),* })?)=>{
        #[doc = concat!("A NewType Wrapper around `", stringify!($origin), "`")]
        struct $wrapper<$($lifetime: $($lconstraints +)*),*>( $origin<$($lifetime),*> );

        impl<$($lifetime: $($lconstraints +)*),*> From<$origin<$($lifetime),*>> for $wrapper<$($lifetime),*> {
            fn from(v: $origin<$($lifetime),*>) -> Self {
                Self(v)
            }
        }

        impl<$($lifetime: $($lconstraints +)*),*> From<$wrapper<$($lifetime),*>> for $origin<$($lifetime),*> {
            fn from(v: $wrapper<$($lifetime),*>) -> Self {
                v.0
            }
        }

        impl<$($lifetime: $($lconstraints +)*),*> core::ops::Deref for $wrapper<$($lifetime),*> {
            type Target = $origin<$($lifetime),*>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$($lifetime: $($lconstraints +)*),*> core::ops::DerefMut for $wrapper<$($lifetime),*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    // Have generic and lifetime with constraints
    ($origin:ident<
        $($lifetime:lifetime $($(:)?$(+)?$lconstraints: lifetime)*),*,
        $($generic:ident $($(:)?$(+)?$gconstraints: path)*),*
        >, $wrapper:ident
        $(, derive = { $($derive:path),* })?)=>{
        #[doc = concat!("A NewType Wrapper around `", stringify!($origin), "`")]
        $(#[derive($($derive),*)])?
        struct $wrapper<$($lifetime: $($lconstraints +)*),*, $($generic),*>( $origin<$($lifetime),*, $($generic),*> )
            where $($generic:$($gconstraints +)*),*;

        impl<$($lifetime: $($lconstraints +)*,)* $($generic: $($gconstraints +)*),*> From<$origin<$($lifetime),*, $($generic),*>> for $wrapper<$($lifetime),*, $($generic),*> {
            fn from(v: $origin<$($lifetime),*, $($generic),*>) -> Self {
                Self(v)
            }
        }

        impl<$($lifetime: $($lconstraints +)*,)* $($generic: $($gconstraints +)*),*> From<$wrapper<$($lifetime),*, $($generic),*>> for $origin<$($lifetime),*, $($generic),*> {
            fn from(v: $wrapper<$($lifetime),*, $($generic),*>) -> Self {
                v.0
            }
        }

        impl<$($lifetime: $($lconstraints +)*,)* $($generic: $($gconstraints +)*),*> core::ops::Deref for $wrapper<$($lifetime),*, $($generic),*> {
            type Target = $origin<$($lifetime),*, $($generic),*>;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$($lifetime: $($lconstraints +)*,)* $($generic: $($gconstraints +)*),*> core::ops::DerefMut for $wrapper<$($lifetime),*, $($generic),*> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

}
