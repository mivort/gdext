use crate::property_info::PropertyInfoBuilder;
use crate::{sys, sys::interface_fn, GodotExtensionClass};
use gdext_builtin::godot_ffi::GodotFfi;
use gdext_builtin::impl_ffi_as_pointer;
use gdext_builtin::variant::Variant;
use gdext_sys::types::OpaqueObject;
use std::marker::PhantomData;

// TODO which bounds to add on struct itself?
pub struct Obj<T> {
    // Note: this may not be a pointer behind the scenes -- consider using an opaque [u8; SIZE_FROM_JSON]
    opaque: OpaqueObject,
    _marker: PhantomData<*const T>,
}

impl<T: GodotExtensionClass> Obj<T> {
    pub fn new(_rust_obj: T) -> Self {
        todo!()
    }

    fn from_opaque(opaque: OpaqueObject) -> Self {
        print!("Obj::from_opaque: opaque={}", opaque);

        let s = Self {
            opaque,
            _marker: PhantomData,
        };

        println!(", self.opaque={}", s.opaque);
        s
    }

    // explicit deref for testing purposes
    pub fn inner(&self) -> &T {
        //let binding = interface_fn!(object_get_instance_binding);
        todo!()
    }

    pub fn instance_id(&self) -> u64 {
        // Note: bit 'id & (1 << 63)' determines if the instance is ref-counted
        unsafe { interface_fn!(object_get_instance_id)(self.sys()) }
    }

    pub fn from_instance_id(instance_id: u64) -> Option<Self> {
        unsafe {
            let ptr = interface_fn!(object_get_instance_from_id)(instance_id);

            if ptr.is_null() {
                None
            } else {
                Some(Obj::from_sys(ptr))
            }
        }
    }
}

impl<T: GodotExtensionClass> GodotFfi for Obj<T> {
    impl_ffi_as_pointer!();
}

impl<T: GodotExtensionClass> From<&Variant> for Obj<T> {
    fn from(variant: &Variant) -> Self {
        unsafe {
            let opaque = OpaqueObject::with_init(|ptr| {
                let converter = sys::get_cache().variant_to_object;
                converter(ptr, variant.sys());
            });

            Obj::from_opaque(opaque)
        }
    }
}

impl<T: GodotExtensionClass> From<Obj<T>> for Variant {
    fn from(obj: Obj<T>) -> Self {
        unsafe {
            Self::from_sys_init(|opaque_ptr| {
                let converter = sys::get_cache().variant_from_object;
                converter(opaque_ptr, obj.opaque.to_sys());
            })
        }
    }
}

impl<T: GodotExtensionClass> From<&Obj<T>> for Variant {
    fn from(_obj: &Obj<T>) -> Self {
        todo!()
    }
}

impl<T: GodotExtensionClass> PropertyInfoBuilder for Obj<T> {
    fn variant_type() -> gdext_sys::GDNativeVariantType {
        gdext_sys::GDNativeVariantType_GDNATIVE_VARIANT_TYPE_OBJECT
    }
}