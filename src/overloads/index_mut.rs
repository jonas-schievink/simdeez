use super::*;
impl IndexMut<usize> for I32x4 {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut I32x4, &mut [i32; 4]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for I32x4_41 {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut I32x4_41, &mut [i32; 4]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for I32x8 {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&mut I32x8, &mut [i32; 8]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for I64x2 {
    fn index_mut(&mut self, i: usize) -> &mut i64 {
        debug_assert!(i < 2);
        let arr = unsafe { mem::transmute::<&mut I64x2, &mut [i64; 2]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for I64x2_41 {
    fn index_mut(&mut self, i: usize) -> &mut i64 {
        debug_assert!(i < 2);
        let arr = unsafe { mem::transmute::<&mut I64x2_41, &mut [i64; 2]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for I64x4 {
    fn index_mut(&mut self, i: usize) -> &mut i64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut I64x4, &mut [i64; 4]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for F32x4 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F32x4, &mut [f32; 4]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for F64x2 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F64x2, &mut [f64; 2]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for F32x8 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        debug_assert!(i < 8);
        let arr = unsafe { mem::transmute::<&mut F32x8, &mut [f32; 8]>(self) };
        &mut arr[i]
    }
}
impl IndexMut<usize> for F64x4 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        debug_assert!(i < 4);
        let arr = unsafe { mem::transmute::<&mut F64x4, &mut [f64; 4]>(self) };
        &mut arr[i]
    }
}
