use crate::dart_opaque::DartOpaque;
use crate::generalized_isolate::{IntoDart, ZeroCopyBuffer};
use crate::rust_opaque::{DartSafe, RustOpaque};

/// Basically the Into trait.
/// We need this separate trait because we need to implement it for Vec<T> etc.
/// These blanket implementations allow us to accept external types in various places.
/// The initial reason for this was to allow mirrored types in StreamSink<>.
/// See also [PR 1285](https://github.com/fzyzcjy/flutter_rust_bridge/pull/1285)
pub trait IntoIntoDart<D: IntoDart> {
    fn into_into_dart(self) -> D;
}

impl<T, D> IntoIntoDart<Vec<D>> for Vec<T>
where
    T: IntoIntoDart<D>,
    Vec<D>: IntoDart,
    D: IntoDart,
{
    fn into_into_dart(self) -> Vec<D> {
        self.into_iter().map(|e| e.into_into_dart()).collect()
    }
}

impl<T, D> IntoIntoDart<Option<D>> for Option<T>
where
    T: IntoIntoDart<D>,
    D: IntoDart,
{
    fn into_into_dart(self) -> Option<D> {
        self.map(|e| e.into_into_dart())
    }
}

impl<T> IntoIntoDart<RustOpaque<T>> for RustOpaque<T>
where
    T: DartSafe,
{
    fn into_into_dart(self) -> RustOpaque<T> {
        self
    }
}

impl<T, D> IntoIntoDart<ZeroCopyBuffer<D>> for ZeroCopyBuffer<T>
where
    T: IntoIntoDart<D>,
    D: IntoDart,
    ZeroCopyBuffer<D>: IntoDart,
{
    fn into_into_dart(self) -> ZeroCopyBuffer<D> {
        ZeroCopyBuffer(self.0.into_into_dart())
    }
}

impl<T, const C: usize> IntoIntoDart<[T; C]> for [T; C]
where
    T: IntoDart,
    [T; C]: IntoDart,
{
    fn into_into_dart(self) -> [T; C] {
        self
    }
}

impl<T> IntoIntoDart<T> for Box<T>
where
    T: IntoDart,
{
    fn into_into_dart(self) -> T {
        *self
    }
}

// These tuple impls should probably be a macro, but that is not easily possible with macro_rules because of the field access
impl<A, AD, B, BD> IntoIntoDart<(AD, BD)> for (A, B)
where
    A: IntoIntoDart<AD>,
    AD: IntoDart,
    B: IntoIntoDart<BD>,
    BD: IntoDart,
{
    fn into_into_dart(self) -> (AD, BD) {
        (self.0.into_into_dart(), self.1.into_into_dart())
    }
}
impl<A, AD, B, BD, C, CD> IntoIntoDart<(AD, BD, CD)> for (A, B, C)
where
    A: IntoIntoDart<AD>,
    AD: IntoDart,
    B: IntoIntoDart<BD>,
    BD: IntoDart,
    C: IntoIntoDart<CD>,
    CD: IntoDart,
{
    fn into_into_dart(self) -> (AD, BD, CD) {
        (
            self.0.into_into_dart(),
            self.1.into_into_dart(),
            self.2.into_into_dart(),
        )
    }
}
impl<A, AD, B, BD, C, CD, D, DD> IntoIntoDart<(AD, BD, CD, DD)> for (A, B, C, D)
where
    A: IntoIntoDart<AD>,
    AD: IntoDart,
    B: IntoIntoDart<BD>,
    BD: IntoDart,
    C: IntoIntoDart<CD>,
    CD: IntoDart,
    D: IntoIntoDart<DD>,
    DD: IntoDart,
{
    fn into_into_dart(self) -> (AD, BD, CD, DD) {
        (
            self.0.into_into_dart(),
            self.1.into_into_dart(),
            self.2.into_into_dart(),
            self.3.into_into_dart(),
        )
    }
}
impl<A, AD, B, BD, C, CD, D, DD, E, ED> IntoIntoDart<(AD, BD, CD, DD, ED)> for (A, B, C, D, E)
where
    A: IntoIntoDart<AD>,
    AD: IntoDart,
    B: IntoIntoDart<BD>,
    BD: IntoDart,
    C: IntoIntoDart<CD>,
    CD: IntoDart,
    D: IntoIntoDart<DD>,
    DD: IntoDart,
    E: IntoIntoDart<ED>,
    ED: IntoDart,
{
    fn into_into_dart(self) -> (AD, BD, CD, DD, ED) {
        (
            self.0.into_into_dart(),
            self.1.into_into_dart(),
            self.2.into_into_dart(),
            self.3.into_into_dart(),
            self.4.into_into_dart(),
        )
    }
}

// more generic impls do not work because they crate possibly conflicting trait impls
// this is why here are some more specific impls

// Implementations for simple types
macro_rules! impl_into_into_dart_by_self {
    ($t:ty) => {
        impl IntoIntoDart<$t> for $t {
            fn into_into_dart(self) -> $t {
                self
            }
        }
    };
}

// Impls for primitive types are taken from the IntoDart trait

impl_into_into_dart_by_self!(u8);
impl_into_into_dart_by_self!(i8);
impl_into_into_dart_by_self!(u16);
impl_into_into_dart_by_self!(i16);
impl_into_into_dart_by_self!(u32);
impl_into_into_dart_by_self!(i32);
impl_into_into_dart_by_self!(u64);
impl_into_into_dart_by_self!(i64);
impl_into_into_dart_by_self!(f32);
impl_into_into_dart_by_self!(f64);
impl_into_into_dart_by_self!(bool);
impl_into_into_dart_by_self!(());
impl_into_into_dart_by_self!(usize);
impl_into_into_dart_by_self!(String);
impl_into_into_dart_by_self!(DartOpaque);
#[cfg(not(target_family = "wasm"))]
impl_into_into_dart_by_self!(allo_isolate::ffi::DartCObject);
#[cfg(target_family = "wasm")]
impl_into_into_dart_by_self!(wasm_bindgen::JsValue);
#[cfg(feature = "uuid")]
impl_into_into_dart_by_self!(uuid::Uuid);
impl_into_into_dart_by_self!(backtrace::Backtrace);

#[cfg(feature = "chrono")]
mod chrono_impls {
    use super::IntoIntoDart;
    use chrono::{Local, Utc};
    impl_into_into_dart_by_self!(chrono::Duration);
    impl_into_into_dart_by_self!(chrono::NaiveDateTime);
    impl_into_into_dart_by_self!(chrono::DateTime<Local>);
    impl_into_into_dart_by_self!(chrono::DateTime<Utc>);
}