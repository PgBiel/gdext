/*
 * Copyright (c) godot-rust; Bromeon and contributors.
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::framework::{expect_panic, itest};
use godot::builtin::{PackedByteArray, PackedFloat32Array, PackedStringArray};

#[itest]
fn packed_array_default() {
    assert_eq!(PackedByteArray::default().len(), 0);
}

#[itest]
fn packed_array_new() {
    assert_eq!(PackedByteArray::new().len(), 0);
}

#[itest]
fn packed_array_from_iterator() {
    let array = PackedByteArray::from_iter([1, 2]);

    assert_eq!(array.len(), 2);
    assert_eq!(array.get(0), 1);
    assert_eq!(array.get(1), 2);
}

#[itest]
fn packed_array_from() {
    let array = PackedByteArray::from(&[1, 2]);

    assert_eq!(array.len(), 2);
    assert_eq!(array.get(0), 1);
    assert_eq!(array.get(1), 2);
}

#[itest]
fn packed_array_to_vec() {
    let array = PackedByteArray::new();
    assert_eq!(array.to_vec(), Vec::<u8>::new());
    let array = PackedByteArray::from(&[1, 2]);
    assert_eq!(array.to_vec(), vec![1, 2]);
}

/*
#[itest(skip)]
fn packed_array_into_iterator() {
    let array = PackedByteArray::from(&[1, 2]);
    let mut iter = array.into_iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}
*/

#[itest]
fn packed_array_eq() {
    assert_eq!(
        PackedByteArray::from(&[1, 2]),
        PackedByteArray::from(&[1, 2])
    );
    assert_ne!(
        PackedByteArray::from(&[1, 2]),
        PackedByteArray::from(&[1, 1])
    );
    assert_ne!(
        PackedFloat32Array::from(&[f32::NAN]),
        PackedFloat32Array::from(&[f32::NAN])
    );
}

#[itest]
fn packed_array_clone() {
    let mut array = PackedByteArray::from(&[1, 2]);
    #[allow(clippy::redundant_clone)]
    let clone = array.clone();
    array.set(0, 3);

    assert_eq!(clone.get(0), 1);
}

#[itest]
fn packed_array_subarray() {
    let array = PackedByteArray::from(&[1, 2, 3]);
    let subarray = array.subarray(1, 2);

    assert_eq!(subarray.to_vec(), vec![2]);
}

#[itest]
fn packed_array_as_slice() {
    let a = PackedByteArray::from(&[1, 2, 3]);
    #[allow(clippy::redundant_clone)]
    let b = a.clone();

    let slice_a = a.as_slice();
    let slice_b = b.as_slice();

    assert_eq!(slice_a, &[1, 2, 3]);
    assert_eq!(slice_a, slice_b);
    assert_eq!(
        slice_a.as_ptr(),
        slice_b.as_ptr(),
        "copy-on-write without modification returns aliased slice"
    );

    let empty = PackedStringArray::new();
    assert_eq!(empty.as_slice(), &[]);
}

#[itest]
fn packed_array_as_mut_slice() {
    let a = PackedByteArray::from(&[1, 2, 3]);
    let mut b = a.clone();

    let slice_a = a.as_slice();
    let slice_b = b.as_mut_slice(); // triggers CoW

    assert_eq!(slice_a, &mut [1, 2, 3]);
    assert_eq!(slice_a, slice_b);
    assert_ne!(
        slice_a.as_ptr(),
        slice_b.as_ptr(),
        "copy-on-write with modification must return independent slice"
    );

    let mut empty = PackedStringArray::new();
    assert_eq!(empty.as_mut_slice(), &mut []);
}

#[itest]
fn packed_array_get() {
    let array = PackedByteArray::from(&[1, 2]);

    assert_eq!(array.get(0), 1);
    assert_eq!(array.get(1), 2);
    expect_panic("Array index 2 out of bounds: length is 2", || {
        array.get(2);
    });
}

#[itest]
fn packed_array_binary_search() {
    let array = PackedByteArray::from(&[1, 3]);

    assert_eq!(array.binary_search(0), 0);
    assert_eq!(array.binary_search(1), 0);
    assert_eq!(array.binary_search(2), 1);
    assert_eq!(array.binary_search(3), 1);
    assert_eq!(array.binary_search(4), 2);
}

#[itest]
fn packed_array_find() {
    let array = PackedByteArray::from(&[1, 2, 1]);

    assert_eq!(array.find(0, None), None);
    assert_eq!(array.find(1, None), Some(0));
    assert_eq!(array.find(1, Some(1)), Some(2));
}

#[itest]
fn packed_array_rfind() {
    let array = PackedByteArray::from(&[1, 2, 1]);

    assert_eq!(array.rfind(0, None), None);
    assert_eq!(array.rfind(1, None), Some(2));
    assert_eq!(array.rfind(1, Some(1)), Some(0));
}

#[itest]
fn packed_array_set() {
    let mut array = PackedByteArray::from(&[1, 2]);

    array.set(0, 3);
    assert_eq!(array.get(0), 3);

    expect_panic("Array index 2 out of bounds: length is 2", move || {
        array.set(2, 4);
    });
}

#[itest]
fn packed_array_push() {
    let mut array = PackedByteArray::from(&[1, 2]);

    array.push(3);

    assert_eq!(array.len(), 3);
    assert_eq!(array.get(2), 3);
}

#[itest]
fn packed_array_insert() {
    let mut array = PackedByteArray::from(&[1, 2]);

    array.insert(0, 3);
    assert_eq!(array.to_vec(), vec![3, 1, 2]);

    array.insert(3, 4);
    assert_eq!(array.to_vec(), vec![3, 1, 2, 4]);
}

#[itest]
fn packed_array_extend() {
    let mut array = PackedByteArray::from(&[1, 2]);
    let other = PackedByteArray::from(&[3, 4]);
    array.extend_array(&other);
    assert_eq!(array.to_vec(), vec![1, 2, 3, 4]);
}

#[itest]
fn packed_array_sort() {
    let mut array = PackedByteArray::from(&[2, 1]);
    array.sort();
    assert_eq!(array.to_vec(), vec![1, 2]);
}

#[itest]
fn packed_array_reverse() {
    let mut array = PackedByteArray::from(&[1, 2]);
    array.reverse();
    assert_eq!(array.to_vec(), vec![2, 1]);
}

#[itest]
fn packed_array_format() {
    let a = PackedByteArray::from(&[2, 1]);
    assert_eq!(format!("{a}"), "[2, 1]");

    let a = PackedByteArray::new();
    assert_eq!(format!("{a}"), "[]");
}
