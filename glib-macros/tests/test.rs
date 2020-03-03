// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ::glib_macros::{GBoxed, GEnum};
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::{FromGlib, ToGlib};
#[macro_use]
extern crate glib;
#[macro_use]
extern crate bitflags;

#[test]
fn derive_genum() {
    #[derive(Debug, Eq, PartialEq, Clone, Copy, GEnum)]
    #[repr(u32)]
    #[genum(type_name = "TestAnimalType")]
    enum Animal {
        Goat,
        #[genum(name = "The Dog")]
        Dog,
        #[genum(name = "The Cat", nick = "chat")]
        Cat = 5,
        Badger,
    }

    assert_eq!(Animal::Goat.to_glib(), 0);
    assert_eq!(Animal::Dog.to_glib(), 1);
    assert_eq!(Animal::Cat.to_glib(), 5);

    assert_eq!(Animal::from_glib(0), Animal::Goat);
    assert_eq!(Animal::from_glib(1), Animal::Dog);
    assert_eq!(Animal::from_glib(5), Animal::Cat);

    assert_eq!(
        Animal::Goat.to_value().get::<Animal>(),
        Ok(Some(Animal::Goat))
    );
    assert_eq!(
        Animal::Dog.to_value().get::<Animal>(),
        Ok(Some(Animal::Dog))
    );
    assert_eq!(
        Animal::Cat.to_value().get::<Animal>(),
        Ok(Some(Animal::Cat))
    );

    let t = Animal::static_type();
    assert!(t.is_a(&glib::Type::BaseEnum));
    assert_eq!(t.name(), "TestAnimalType");

    let e = glib::EnumClass::new(t).expect("EnumClass::new failed");
    let v = e.get_value(0).expect("EnumClass::get_value(0) failed");
    assert_eq!(v.get_name(), "Goat");
    assert_eq!(v.get_nick(), "goat");
    let v = e.get_value(1).expect("EnumClass::get_value(1) failed");
    assert_eq!(v.get_name(), "The Dog");
    assert_eq!(v.get_nick(), "dog");
    let v = e.get_value(5).expect("EnumClass::get_value(5) failed");
    assert_eq!(v.get_name(), "The Cat");
    assert_eq!(v.get_nick(), "chat");
    assert_eq!(e.get_value(2), None);
}

#[test]
fn derive_gboxed() {
    #[derive(Clone, Debug, PartialEq, Eq, GBoxed)]
    #[gboxed(type_name = "MyBoxed")]
    struct MyBoxed(String);

    assert_eq!(MyBoxed::get_type().name(), "MyBoxed");
    let b = MyBoxed(String::from("abc"));
    let v = b.to_value();
    let b2 = v.get::<&MyBoxed>().unwrap().unwrap();
    assert_eq!(&b, b2);
}

macro_rules! gflags {
    (
        #[gflags(type_name = $TypeName:literal)]
        struct $Name:ident: $T:ty {
            $(
                #[gflags(name = $Gname:literal, nick = $Nick:literal)]
                const $Flag:ident = $value:expr;
            )+
        }
    ) => {
        bitflags!{
            struct $Name: $T {
                $(
                    const $Flag = $value;
                )+
            }
        }

        impl glib::translate::ToGlib for $Name {
            type GlibType = $T;

            fn to_glib(&self) -> $T {
                self.bits()
            }
        }

        impl glib::translate::FromGlib<u32> for $Name {
            fn from_glib(value: $T) -> Self {
                $Name::from_bits_truncate(value)
            }
        }

        impl<'a> glib::value::FromValueOptional<'a> for $Name {
            unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
                Some(glib::value::FromValue::from_value(value))
            }
        }

        impl<'a> glib::value::FromValue<'a> for $Name {
            unsafe fn from_value(value: &glib::Value) -> Self {
                glib::translate::from_glib(
                    gobject_sys::g_value_get_flags(
                        glib::translate::ToGlibPtr::to_glib_none(value).0))
            }
        }

        impl glib::value::SetValue for $Name {
            unsafe fn set_value(value: &mut glib::Value, this: &Self) {
                gobject_sys::g_value_set_flags(
                    glib::translate::ToGlibPtrMut::to_glib_none_mut(value).0,
                    glib::translate::ToGlib::to_glib(this))
            }
        }

        impl StaticType for $Name {
            fn static_type() -> glib::Type {
              static ONCE: std::sync::Once = std::sync::Once::new();
              static mut TYPE: glib::Type = glib::Type::Invalid;

              ONCE.call_once(|| {
                  // FIXME: hardcoded array size
                  static mut VALUES: [gobject_sys::GFlagsValue; 4] = [
                      $(
                          gobject_sys::GFlagsValue {
                              value: 1,
                              // FIXME: both strings should be \0 terminated
                              value_name: $Gname as *const _ as *const _,
                              value_nick: $Nick as *const _ as *const _,
                          },
                      )+
                      gobject_sys::GFlagsValue {
                          value: 0,
                          value_name: std::ptr::null(),
                          value_nick: std::ptr::null(),
                      },
                  ];

                  let name = std::ffi::CString::new($TypeName).expect("CString::new failed");
                  unsafe {
                      let type_ = gobject_sys::g_flags_register_static(name.as_ptr(), VALUES.as_ptr());
                      TYPE = glib::translate::from_glib(type_);
                  }
              });

              unsafe {
                  assert_ne!(TYPE, glib::Type::Invalid);
                  TYPE
              }
            }
        }
    };
}

#[test]
fn derive_gflags() {
    gflags! {
        #[gflags(type_name = "MyFlags")]
        struct MyFlags: u32 {
            #[gflags(name = "Flag A", nick = "A")]
            const A = 0b00000001;
            #[gflags(name = "Flag B", nick = "B")]
            const B = 0b00000010;
            #[gflags(name = "Flag A and B", nick = "A-B")]
            const AB = Self::A.bits | Self::B.bits;
        }
    }

    assert_eq!(MyFlags::A.bits(), 1);
    assert_eq!(MyFlags::B.bits(), 2);
    assert_eq!(MyFlags::AB.bits(), 3);

    assert_eq!(MyFlags::empty().to_glib(), 0);
    assert_eq!(MyFlags::A.to_glib(), 1);
    assert_eq!(MyFlags::B.to_glib(), 2);
    assert_eq!(MyFlags::AB.to_glib(), 3);

    assert_eq!(MyFlags::from_glib(0), MyFlags::empty());
    assert_eq!(MyFlags::from_glib(1), MyFlags::A);
    assert_eq!(MyFlags::from_glib(2), MyFlags::B);
    assert_eq!(MyFlags::from_glib(3), MyFlags::AB);

    assert_eq!(
        MyFlags::empty().to_value().get::<MyFlags>(),
        Ok(Some(MyFlags::empty()))
    );
    assert_eq!(MyFlags::A.to_value().get::<MyFlags>(), Ok(Some(MyFlags::A)));
    assert_eq!(MyFlags::B.to_value().get::<MyFlags>(), Ok(Some(MyFlags::B)));
    assert_eq!(
        MyFlags::AB.to_value().get::<MyFlags>(),
        Ok(Some(MyFlags::AB))
    );

    let t = MyFlags::static_type();
    assert!(t.is_a(&glib::Type::BaseFlags));
    assert_eq!(t.name(), "MyFlags");

    let e = glib::FlagsClass::new(t).expect("FlagsClass::new failed");
    let v = e.get_value(1).expect("FlagsClass::get_value(1) failed");
    assert_eq!(v.get_name(), "Flag A");
    assert_eq!(v.get_nick(), "A");
}
