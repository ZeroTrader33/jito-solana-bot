::windows_core::imp::com_interface!(ID2D1SimplifiedGeometrySink, ID2D1SimplifiedGeometrySink_Vtbl, 0x2cd9069e_12e2_11dc_9fed_001143a055f9);
::windows_core::imp::interface_hierarchy!(ID2D1SimplifiedGeometrySink, ::windows_core::IUnknown);
impl ID2D1SimplifiedGeometrySink {
    pub unsafe fn SetFillMode(&self, fillmode: D2D1_FILL_MODE) {
        (::windows_core::Interface::vtable(self).SetFillMode)(::windows_core::Interface::as_raw(self), fillmode)
    }
    pub unsafe fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT) {
        (::windows_core::Interface::vtable(self).SetSegmentFlags)(::windows_core::Interface::as_raw(self), vertexflags)
    }
    pub unsafe fn BeginFigure(&self, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
        (::windows_core::Interface::vtable(self).BeginFigure)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(startpoint), figurebegin)
    }
    pub unsafe fn AddLines(&self, points: &[D2D_POINT_2F]) {
        (::windows_core::Interface::vtable(self).AddLines)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(points.as_ptr()), points.len().try_into().unwrap())
    }
    pub unsafe fn AddBeziers(&self, beziers: &[D2D1_BEZIER_SEGMENT]) {
        (::windows_core::Interface::vtable(self).AddBeziers)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(beziers.as_ptr()), beziers.len().try_into().unwrap())
    }
    pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
        (::windows_core::Interface::vtable(self).EndFigure)(::windows_core::Interface::as_raw(self), figureend)
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
}
unsafe impl ::core::marker::Send for ID2D1SimplifiedGeometrySink {}
unsafe impl ::core::marker::Sync for ID2D1SimplifiedGeometrySink {}
#[repr(C)]
pub struct ID2D1SimplifiedGeometrySink_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetFillMode: unsafe extern "system" fn(*mut ::core::ffi::c_void, D2D1_FILL_MODE),
    pub SetSegmentFlags: unsafe extern "system" fn(*mut ::core::ffi::c_void, D2D1_PATH_SEGMENT),
    pub BeginFigure: unsafe extern "system" fn(*mut ::core::ffi::c_void, D2D_POINT_2F, D2D1_FIGURE_BEGIN),
    pub AddLines: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const D2D_POINT_2F, u32),
    pub AddBeziers: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const D2D1_BEZIER_SEGMENT, u32),
    pub EndFigure: unsafe extern "system" fn(*mut ::core::ffi::c_void, D2D1_FIGURE_END),
    pub Close: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(4i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_CUBIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(2i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(5i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_LINEAR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(1i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(3i32);
pub const D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(0i32);
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(3i32);
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(1i32);
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(2i32);
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(0i32);
pub const D2D1_BLEND_MODE_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(22i32);
pub const D2D1_BLEND_MODE_COLOR_BURN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(5i32);
pub const D2D1_BLEND_MODE_COLOR_DODGE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(9i32);
pub const D2D1_BLEND_MODE_DARKEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(2i32);
pub const D2D1_BLEND_MODE_DARKER_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(7i32);
pub const D2D1_BLEND_MODE_DIFFERENCE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(18i32);
pub const D2D1_BLEND_MODE_DISSOLVE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(4i32);
pub const D2D1_BLEND_MODE_DIVISION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(25i32);
pub const D2D1_BLEND_MODE_EXCLUSION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(19i32);
pub const D2D1_BLEND_MODE_HARD_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(13i32);
pub const D2D1_BLEND_MODE_HARD_MIX: D2D1_BLEND_MODE = D2D1_BLEND_MODE(17i32);
pub const D2D1_BLEND_MODE_HUE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(20i32);
pub const D2D1_BLEND_MODE_LIGHTEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(3i32);
pub const D2D1_BLEND_MODE_LIGHTER_COLOR: D2D1_BLEND_MODE = D2D1_BLEND_MODE(8i32);
pub const D2D1_BLEND_MODE_LINEAR_BURN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(6i32);
pub const D2D1_BLEND_MODE_LINEAR_DODGE: D2D1_BLEND_MODE = D2D1_BLEND_MODE(10i32);
pub const D2D1_BLEND_MODE_LINEAR_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(15i32);
pub const D2D1_BLEND_MODE_LUMINOSITY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(23i32);
pub const D2D1_BLEND_MODE_MULTIPLY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(0i32);
pub const D2D1_BLEND_MODE_OVERLAY: D2D1_BLEND_MODE = D2D1_BLEND_MODE(11i32);
pub const D2D1_BLEND_MODE_PIN_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(16i32);
pub const D2D1_BLEND_MODE_SATURATION: D2D1_BLEND_MODE = D2D1_BLEND_MODE(21i32);
pub const D2D1_BLEND_MODE_SCREEN: D2D1_BLEND_MODE = D2D1_BLEND_MODE(1i32);
pub const D2D1_BLEND_MODE_SOFT_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(12i32);
pub const D2D1_BLEND_MODE_SUBTRACT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(24i32);
pub const D2D1_BLEND_MODE_VIVID_LIGHT: D2D1_BLEND_MODE = D2D1_BLEND_MODE(14i32);
pub const D2D1_BORDER_MODE_HARD: D2D1_BORDER_MODE = D2D1_BORDER_MODE(1i32);
pub const D2D1_BORDER_MODE_SOFT: D2D1_BORDER_MODE = D2D1_BORDER_MODE(0i32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(1i32);
pub const D2D1_COLORMATRIX_ALPHA_MODE_STRAIGHT: D2D1_COLORMATRIX_ALPHA_MODE = D2D1_COLORMATRIX_ALPHA_MODE(2i32);
pub const D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(11i32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_ATOP: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(7i32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_IN: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(3i32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_OUT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(5i32);
pub const D2D1_COMPOSITE_MODE_DESTINATION_OVER: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(1i32);
pub const D2D1_COMPOSITE_MODE_MASK_INVERT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(12i32);
pub const D2D1_COMPOSITE_MODE_PLUS: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(9i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_ATOP: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(6i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_COPY: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(10i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_IN: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(2i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_OUT: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(4i32);
pub const D2D1_COMPOSITE_MODE_SOURCE_OVER: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(0i32);
pub const D2D1_COMPOSITE_MODE_XOR: D2D1_COMPOSITE_MODE = D2D1_COMPOSITE_MODE(8i32);
pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(0i32);
pub const D2D1_FIGURE_BEGIN_HOLLOW: D2D1_FIGURE_BEGIN = D2D1_FIGURE_BEGIN(1i32);
pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = D2D1_FIGURE_END(1i32);
pub const D2D1_FIGURE_END_OPEN: D2D1_FIGURE_END = D2D1_FIGURE_END(0i32);
pub const D2D1_FILL_MODE_ALTERNATE: D2D1_FILL_MODE = D2D1_FILL_MODE(0i32);
pub const D2D1_FILL_MODE_WINDING: D2D1_FILL_MODE = D2D1_FILL_MODE(1i32);
pub const D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(2i32);
pub const D2D1_PATH_SEGMENT_FORCE_UNSTROKED: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(1i32);
pub const D2D1_PATH_SEGMENT_NONE: D2D1_PATH_SEGMENT = D2D1_PATH_SEGMENT(0i32);
pub const D2D1_TURBULENCE_NOISE_FRACTAL_SUM: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(0i32);
pub const D2D1_TURBULENCE_NOISE_TURBULENCE: D2D1_TURBULENCE_NOISE = D2D1_TURBULENCE_NOISE(1i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE(pub i32);
impl ::windows_core::TypeKind for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_ALPHA_MODE(pub i32);
impl ::windows_core::TypeKind for D2D1_ALPHA_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_ALPHA_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_ALPHA_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_BLEND_MODE(pub i32);
impl ::windows_core::TypeKind for D2D1_BLEND_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_BLEND_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BLEND_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_BORDER_MODE(pub i32);
impl ::windows_core::TypeKind for D2D1_BORDER_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_BORDER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_BORDER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_COLORMATRIX_ALPHA_MODE(pub i32);
impl ::windows_core::TypeKind for D2D1_COLORMATRIX_ALPHA_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_COLORMATRIX_ALPHA_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COLORMATRIX_ALPHA_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_COMPOSITE_MODE(pub i32);
impl ::windows_core::TypeKind for D2D1_COMPOSITE_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_COMPOSITE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_COMPOSITE_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_FIGURE_BEGIN(pub i32);
impl ::windows_core::TypeKind for D2D1_FIGURE_BEGIN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_FIGURE_BEGIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FIGURE_BEGIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_FIGURE_END(pub i32);
impl ::windows_core::TypeKind for D2D1_FIGURE_END {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_FIGURE_END {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FIGURE_END").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_FILL_MODE(pub i32);
impl ::windows_core::TypeKind for D2D1_FILL_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_FILL_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_PATH_SEGMENT(pub i32);
impl ::windows_core::TypeKind for D2D1_PATH_SEGMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_PATH_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_PATH_SEGMENT").field(&self.0).finish()
    }
}
impl D2D1_PATH_SEGMENT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D2D1_PATH_SEGMENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D2D1_PATH_SEGMENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D2D1_PATH_SEGMENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct D2D1_TURBULENCE_NOISE(pub i32);
impl ::windows_core::TypeKind for D2D1_TURBULENCE_NOISE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for D2D1_TURBULENCE_NOISE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D2D1_TURBULENCE_NOISE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: D2D_POINT_2F,
    pub point2: D2D_POINT_2F,
    pub point3: D2D_POINT_2F,
}
impl ::core::marker::Copy for D2D1_BEZIER_SEGMENT {}
impl ::core::clone::Clone for D2D1_BEZIER_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D1_BEZIER_SEGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_BEZIER_SEGMENT").field("point1", &self.point1).field("point2", &self.point2).field("point3", &self.point3).finish()
    }
}
impl ::windows_core::TypeKind for D2D1_BEZIER_SEGMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D1_BEZIER_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.point1 == other.point1 && self.point2 == other.point2 && self.point3 == other.point3
    }
}
impl ::core::cmp::Eq for D2D1_BEZIER_SEGMENT {}
impl ::core::default::Default for D2D1_BEZIER_SEGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for D2D1_COLOR_F {}
impl ::core::clone::Clone for D2D1_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D1_COLOR_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_COLOR_F").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::windows_core::TypeKind for D2D1_COLOR_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D1_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for D2D1_COLOR_F {}
impl ::core::default::Default for D2D1_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: D2D1_COLOR_F,
}
impl ::core::marker::Copy for D2D1_GRADIENT_STOP {}
impl ::core::clone::Clone for D2D1_GRADIENT_STOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D1_GRADIENT_STOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_GRADIENT_STOP").field("position", &self.position).field("color", &self.color).finish()
    }
}
impl ::windows_core::TypeKind for D2D1_GRADIENT_STOP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D1_GRADIENT_STOP {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.color == other.color
    }
}
impl ::core::cmp::Eq for D2D1_GRADIENT_STOP {}
impl ::core::default::Default for D2D1_GRADIENT_STOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::super::Dxgi::Common::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D2D1_PIXEL_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D2D1_PIXEL_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D2D1_PIXEL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D1_PIXEL_FORMAT").field("format", &self.format).field("alphaMode", &self.alphaMode).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::TypeKind for D2D1_PIXEL_FORMAT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D2D1_PIXEL_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.format == other.format && self.alphaMode == other.alphaMode
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D2D1_PIXEL_FORMAT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D2D1_PIXEL_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl ::core::marker::Copy for D2D_COLOR_F {}
impl ::core::clone::Clone for D2D_COLOR_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_COLOR_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_COLOR_F").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::windows_core::TypeKind for D2D_COLOR_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_COLOR_F {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for D2D_COLOR_F {}
impl ::core::default::Default for D2D_COLOR_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_MATRIX_4X3_F {
    pub Anonymous: D2D_MATRIX_4X3_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_4X3_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for D2D_MATRIX_4X3_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union D2D_MATRIX_4X3_F_0 {
    pub Anonymous: D2D_MATRIX_4X3_F_0_0,
    pub m: [f32; 12],
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_4X3_F_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for D2D_MATRIX_4X3_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_MATRIX_4X3_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_4X3_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X3_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_MATRIX_4X3_F_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_MATRIX_4X3_F_0_0").field("_11", &self._11).field("_12", &self._12).field("_13", &self._13).field("_21", &self._21).field("_22", &self._22).field("_23", &self._23).field("_31", &self._31).field("_32", &self._32).field("_33", &self._33).field("_41", &self._41).field("_42", &self._42).field("_43", &self._43).finish()
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_4X3_F_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X3_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X3_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_4X3_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_MATRIX_4X4_F {
    pub Anonymous: D2D_MATRIX_4X4_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_4X4_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for D2D_MATRIX_4X4_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union D2D_MATRIX_4X4_F_0 {
    pub Anonymous: D2D_MATRIX_4X4_F_0_0,
    pub m: [f32; 16],
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_4X4_F_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for D2D_MATRIX_4X4_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_MATRIX_4X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_4X4_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_4X4_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_MATRIX_4X4_F_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_MATRIX_4X4_F_0_0").field("_11", &self._11).field("_12", &self._12).field("_13", &self._13).field("_14", &self._14).field("_21", &self._21).field("_22", &self._22).field("_23", &self._23).field("_24", &self._24).field("_31", &self._31).field("_32", &self._32).field("_33", &self._33).field("_34", &self._34).field("_41", &self._41).field("_42", &self._42).field("_43", &self._43).field("_44", &self._44).finish()
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_4X4_F_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_4X4_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._14 == other._14 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._24 == other._24 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._34 == other._34 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43 && self._44 == other._44
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_4X4_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_4X4_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_MATRIX_5X4_F {
    pub Anonymous: D2D_MATRIX_5X4_F_0,
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_5X4_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for D2D_MATRIX_5X4_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union D2D_MATRIX_5X4_F_0 {
    pub Anonymous: D2D_MATRIX_5X4_F_0_0,
    pub m: [f32; 20],
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F_0 {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_5X4_F_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for D2D_MATRIX_5X4_F_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_MATRIX_5X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
    pub _51: f32,
    pub _52: f32,
    pub _53: f32,
    pub _54: f32,
}
impl ::core::marker::Copy for D2D_MATRIX_5X4_F_0_0 {}
impl ::core::clone::Clone for D2D_MATRIX_5X4_F_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_MATRIX_5X4_F_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_MATRIX_5X4_F_0_0")
            .field("_11", &self._11)
            .field("_12", &self._12)
            .field("_13", &self._13)
            .field("_14", &self._14)
            .field("_21", &self._21)
            .field("_22", &self._22)
            .field("_23", &self._23)
            .field("_24", &self._24)
            .field("_31", &self._31)
            .field("_32", &self._32)
            .field("_33", &self._33)
            .field("_34", &self._34)
            .field("_41", &self._41)
            .field("_42", &self._42)
            .field("_43", &self._43)
            .field("_44", &self._44)
            .field("_51", &self._51)
            .field("_52", &self._52)
            .field("_53", &self._53)
            .field("_54", &self._54)
            .finish()
    }
}
impl ::windows_core::TypeKind for D2D_MATRIX_5X4_F_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_MATRIX_5X4_F_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._14 == other._14 && self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._24 == other._24 && self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._34 == other._34 && self._41 == other._41 && self._42 == other._42 && self._43 == other._43 && self._44 == other._44 && self._51 == other._51 && self._52 == other._52 && self._53 == other._53 && self._54 == other._54
    }
}
impl ::core::cmp::Eq for D2D_MATRIX_5X4_F_0_0 {}
impl ::core::default::Default for D2D_MATRIX_5X4_F_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for D2D_POINT_2F {}
impl ::core::clone::Clone for D2D_POINT_2F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_POINT_2F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_POINT_2F").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::windows_core::TypeKind for D2D_POINT_2F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_POINT_2F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_POINT_2F {}
impl ::core::default::Default for D2D_POINT_2F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
}
impl ::core::marker::Copy for D2D_POINT_2U {}
impl ::core::clone::Clone for D2D_POINT_2U {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_POINT_2U {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_POINT_2U").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::windows_core::TypeKind for D2D_POINT_2U {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_POINT_2U {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_POINT_2U {}
impl ::core::default::Default for D2D_POINT_2U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl ::core::marker::Copy for D2D_RECT_F {}
impl ::core::clone::Clone for D2D_RECT_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_RECT_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_RECT_F").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::windows_core::TypeKind for D2D_RECT_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_RECT_F {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for D2D_RECT_F {}
impl ::core::default::Default for D2D_RECT_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
}
impl ::core::marker::Copy for D2D_RECT_U {}
impl ::core::clone::Clone for D2D_RECT_U {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_RECT_U {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_RECT_U").field("left", &self.left).field("top", &self.top).field("right", &self.right).field("bottom", &self.bottom).finish()
    }
}
impl ::windows_core::TypeKind for D2D_RECT_U {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_RECT_U {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.right == other.right && self.bottom == other.bottom
    }
}
impl ::core::cmp::Eq for D2D_RECT_U {}
impl ::core::default::Default for D2D_RECT_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for D2D_SIZE_F {}
impl ::core::clone::Clone for D2D_SIZE_F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_SIZE_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_SIZE_F").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::windows_core::TypeKind for D2D_SIZE_F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_SIZE_F {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for D2D_SIZE_F {}
impl ::core::default::Default for D2D_SIZE_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
impl ::core::marker::Copy for D2D_SIZE_U {}
impl ::core::clone::Clone for D2D_SIZE_U {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_SIZE_U {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_SIZE_U").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::windows_core::TypeKind for D2D_SIZE_U {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_SIZE_U {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for D2D_SIZE_U {}
impl ::core::default::Default for D2D_SIZE_U {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_VECTOR_2F {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_2F {}
impl ::core::clone::Clone for D2D_VECTOR_2F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_VECTOR_2F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_VECTOR_2F").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::windows_core::TypeKind for D2D_VECTOR_2F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_VECTOR_2F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_2F {}
impl ::core::default::Default for D2D_VECTOR_2F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_VECTOR_3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_3F {}
impl ::core::clone::Clone for D2D_VECTOR_3F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_VECTOR_3F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_VECTOR_3F").field("x", &self.x).field("y", &self.y).field("z", &self.z).finish()
    }
}
impl ::windows_core::TypeKind for D2D_VECTOR_3F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_VECTOR_3F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_3F {}
impl ::core::default::Default for D2D_VECTOR_3F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D_VECTOR_4F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl ::core::marker::Copy for D2D_VECTOR_4F {}
impl ::core::clone::Clone for D2D_VECTOR_4F {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for D2D_VECTOR_4F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D2D_VECTOR_4F").field("x", &self.x).field("y", &self.y).field("z", &self.z).field("w", &self.w).finish()
    }
}
impl ::windows_core::TypeKind for D2D_VECTOR_4F {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for D2D_VECTOR_4F {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}
impl ::core::cmp::Eq for D2D_VECTOR_4F {}
impl ::core::default::Default for D2D_VECTOR_4F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
