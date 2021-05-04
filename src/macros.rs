use glib::translate::*;
use libc::c_void;
use std::{fmt, mem, ptr};

macro_rules! event_wrapper {
    ($name:ident, $ffi_name:ident) => {
        impl<'a> ToGlibPtr<'a, *const ::ffi::$ffi_name> for $name {
            type Storage = &'a Self;

            #[inline]
            fn to_glib_none(&'a self) -> Stash<'a, *const ::ffi::$ffi_name, Self> {
                let ptr = ToGlibPtr::<*const ::ffi::ClutterEvent>::to_glib_none(&self.0).0;
                Stash(ptr as *const ::ffi::$ffi_name, self)
            }
        }

        impl<'a> ToGlibPtrMut<'a, *mut ::ffi::$ffi_name> for $name {
            type Storage = &'a mut Self;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ::ffi::$ffi_name, Self> {
                let ptr = ToGlibPtrMut::<*mut ::ffi::ClutterEvent>::to_glib_none_mut(&mut self.0).0;
                StashMut(ptr as *mut ::ffi::$ffi_name, self)
            }
        }

        impl FromGlibPtrNone<*mut ::ffi::$ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut ::ffi::$ffi_name) -> Self {
                $name(from_glib_none(ptr as *mut ::ffi::ClutterEvent))
            }
        }

        impl FromGlibPtrBorrow<*mut ::ffi::$ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_borrow(
                ptr: *mut ::ffi::$ffi_name,
            ) -> glib::translate::Borrowed<Self> {
                glib::translate::Borrowed::new(
                    <$name as crate::FromEvent>::from(
                        crate::Event::from_glib_borrow(ptr as *mut ::ffi::ClutterEvent)
                            .into_inner(),
                    )
                    .map_err(std::mem::forget)
                    .unwrap(),
                )
            }
        }

        impl FromGlibPtrFull<*mut ::ffi::$ffi_name> for $name {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut ::ffi::$ffi_name) -> Self {
                $name(from_glib_full(ptr as *mut ::ffi::ClutterEvent))
            }
        }

        impl AsRef<::ffi::$ffi_name> for $name {
            #[inline]
            fn as_ref(&self) -> &::ffi::$ffi_name {
                unsafe {
                    let ptr: *const ::ffi::$ffi_name = self.to_glib_none().0;
                    &*ptr
                }
            }
        }

        impl AsMut<::ffi::$ffi_name> for $name {
            #[inline]
            fn as_mut(&mut self) -> &mut ::ffi::$ffi_name {
                unsafe {
                    let ptr: *mut ::ffi::$ffi_name = self.to_glib_none_mut().0;
                    &mut *ptr
                }
            }
        }
    };
}

macro_rules! event_subtype {
    ($name:ident, $($ty:path)|+) => {
        impl crate::FromEvent for $name {
            #[inline]
            fn is(ev: &crate::Event) -> bool {
                skip_assert_initialized!();
                match ev.as_ref().type_ {
                    $($ty)|+ => true,
                    _ => false,
                }
            }

            #[inline]
            fn from(ev: crate::Event) -> Result<Self, crate::Event> {
                skip_assert_initialized!();
                if Self::is(&ev) {
                    Ok($name(ev))
                }
                else {
                    Err(ev)
                }
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = crate::Event;

            fn deref(&self) -> &crate::Event {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut crate::Event {
                &mut self.0
            }
        }
    }
}
