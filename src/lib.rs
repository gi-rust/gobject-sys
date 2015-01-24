// Copyright (C) 2013-2015  Mikhail Zabaluev <mikhail.zabaluev@gmail.com>
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

#![crate_name = "gobject-2_0-sys"]
#![crate_type = "lib"]

#![allow(missing_copy_implementations)]

extern crate "glib-2_0-sys" as glib;

use glib::types::{gboolean, gchar, gdouble, gfloat, gint, glong, gpointer};
use glib::types::{gsize, guchar, guint, gulong};

pub type GType = gsize;

#[repr(C)]
pub struct GTypeInstance;

#[repr(C)]
pub struct GValue {
    pub g_type: GType,
    // If the data enum layout turns out to be different on some arch,
    // we'd need arch-specific redefinitions
    data: [u64; 2]
}

extern {
    pub fn g_object_ref(obj: gpointer) -> gpointer;
    pub fn g_object_unref(obj: gpointer);
    pub fn g_type_check_instance_is_a(instance: *const GTypeInstance, iface_type: GType) -> gboolean;
    pub fn g_type_name(t: GType) -> *const gchar;
    pub fn g_value_copy(src: *const GValue, dst: *mut GValue);
    pub fn g_value_get_boolean(value: *const GValue) -> gboolean;
    pub fn g_value_get_char(value: *const GValue) -> gchar;
    pub fn g_value_get_double(value: *const GValue) -> gdouble;
    pub fn g_value_get_float(value: *const GValue) -> gfloat;
    pub fn g_value_get_int(value: *const GValue) -> gint;
    pub fn g_value_get_int64(value: *const GValue) -> i64;
    pub fn g_value_get_long(value: *const GValue) -> glong;
    pub fn g_value_get_object(value: *const GValue) -> gpointer;
    pub fn g_value_get_schar(value: *const GValue) -> i8;
    pub fn g_value_get_string(value: *const GValue) -> *const gchar;
    pub fn g_value_get_uchar(value: *const GValue) -> guchar;
    pub fn g_value_get_uint(value: *const GValue) -> guint;
    pub fn g_value_get_uint64(value: *const GValue) -> u64;
    pub fn g_value_get_ulong(value: *const GValue) -> gulong;
    pub fn g_value_init(value: *mut GValue, type_id: GType) -> *mut GValue;
    pub fn g_value_set_boolean(value: *mut GValue, v_boolean: gboolean);
    pub fn g_value_set_char(value: *mut GValue, v_char: gchar);
    pub fn g_value_set_double(value: *mut GValue, v_double: gdouble);
    pub fn g_value_set_float(value: *mut GValue, v_float: gfloat);
    pub fn g_value_set_int(value: *mut GValue, v_int: gint);
    pub fn g_value_set_int64(value: *mut GValue, v_int: i64);
    pub fn g_value_set_long(value: *mut GValue, v_long: glong);
    pub fn g_value_set_object(value: *mut GValue, v_object: gpointer);
    pub fn g_value_set_schar(value: *mut GValue, v_char: i8);
    pub fn g_value_set_string(value: *mut GValue, v_string: *const gchar);
    pub fn g_value_set_static_string(value: *mut GValue, v_string: *const gchar);
    pub fn g_value_set_uchar(value: *mut GValue, v_char: guchar);
    pub fn g_value_set_uint(value: *mut GValue, v_uint: guint);
    pub fn g_value_set_uint64(value: *mut GValue, v_uint: u64);
    pub fn g_value_set_ulong(value: *mut GValue, v_ulong: gulong);
    pub fn g_value_take_string(value: *mut GValue, v_string: *mut gchar);
    pub fn g_value_unset(value: *mut GValue);
}
