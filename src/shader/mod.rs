use std::{
    ffi::{CStr, CString},
    io,
    marker::PhantomData,
    path::Path,
    rc::Rc,
};

use raylib_sys as sys;

use crate::math::{Vector2, Vector3, Vector4};

mod sealed {
    use raylib_sys as sys;
    pub trait ShaderUniformPriv {
        fn set(self, shader: sys::Shader, index: i32);
    }
}

pub trait ShaderUniform: sealed::ShaderUniformPriv {}

macro_rules! impl_shader_uniform {
    ($ty: ty => $($tt: tt)*) => {
        impl sealed::ShaderUniformPriv for $ty {
            $($tt)*
        }
        impl ShaderUniform for $ty {}
    };
}

impl_shader_uniform!(f32 =>
    fn set(self, shader: sys::Shader, index: i32) {
        unsafe { sys::SetShaderValue(shader, index, (&raw const self).cast(), sys::ShaderUniformDataType::SHADER_UNIFORM_FLOAT as _) }
    }
);
impl_shader_uniform!(i32 =>
    fn set(self, shader: sys::Shader, index: i32) {
        unsafe { sys::SetShaderValue(shader, index, (&raw const self).cast(), sys::ShaderUniformDataType::SHADER_UNIFORM_INT as _) }
    }
);
impl_shader_uniform!(u32 =>
    fn set(self, shader: sys::Shader, index: i32) {
        unsafe { sys::SetShaderValue(shader, index, (&raw const self).cast(), sys::ShaderUniformDataType::SHADER_UNIFORM_UINT as _) }
    }
);
impl_shader_uniform!(Vector2 =>
    fn set(self, shader: sys::Shader, index: i32) {
        let items = [self.x, self.y];
        unsafe { sys::SetShaderValue(shader, index, (&raw const items).cast(), sys::ShaderUniformDataType::SHADER_UNIFORM_VEC2 as _) }
    }
);
impl_shader_uniform!(Vector3 =>
    fn set(self, shader: sys::Shader, index: i32) {
        let items = [self.x, self.y, self.z];
        unsafe { sys::SetShaderValue(shader, index, (&raw const items).cast(), sys::ShaderUniformDataType::SHADER_UNIFORM_VEC3 as _) }
    }
);
impl_shader_uniform!(Vector4 =>
    fn set(self, shader: sys::Shader, index: i32) {
        let items = [self.x, self.y, self.z, self.w];
        unsafe { sys::SetShaderValue(shader, index, (&raw const items).cast(), sys::ShaderUniformDataType::SHADER_UNIFORM_VEC4 as _) }
    }
);

#[derive(Debug)]
pub struct ShaderLocation<T> {
    shader: Shader,
    id: u32,
    _phantom: PhantomData<T>,
}

impl<T> ShaderLocation<T> {
    fn from_id(shader: Shader, id: i32) -> Option<Self> {
        if id < 0 {
            None
        } else {
            Some(Self {
                shader,
                id: id as _,
                _phantom: PhantomData,
            })
        }
    }
}

impl<T: ShaderUniform> ShaderLocation<T> {
    pub fn set(&mut self, t: T) {
        t.set(*self.shader.0, self.id as _);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Shader(Rc<sys::Shader>);

impl Drop for Shader {
    fn drop(&mut self) {
        if let Some(tex) = Rc::get_mut(&mut self.0) {
            unsafe { sys::UnloadShader(*tex) };
        }
    }
}

impl Shader {
    pub(crate) fn from_sys(shader: sys::Shader) -> Option<Self> {
        Self::is_valid(shader).then_some(Self(Rc::new(shader)))
    }

    pub(crate) fn is_valid(shader: sys::Shader) -> bool {
        unsafe { sys::IsShaderValid(shader) }
    }

    // not pub clone
    pub(crate) fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    /// Load shaders directly from [`CStr`]s
    ///
    /// Prefer [`Self::load_from_memory`], if possible.
    pub fn load_from_c_strings(
        vertex_shader: Option<&CStr>,
        fragment_shader: Option<&CStr>,
    ) -> Option<Self> {
        Self::from_sys(unsafe {
            sys::LoadShaderFromMemory(
                vertex_shader.map(CStr::as_ptr).unwrap_or_default(),
                fragment_shader.map(CStr::as_ptr).unwrap_or_default(),
            )
        })
    }

    pub fn load_from_memory(
        vertex_shader: Option<impl AsRef<str>>,
        fragment_shader: Option<impl AsRef<str>>,
    ) -> Option<Self> {
        Self::load_from_c_strings(
            vertex_shader
                .map(|s| CString::new(s.as_ref()).unwrap())
                .as_deref(),
            fragment_shader
                .map(|s| CString::new(s.as_ref()).unwrap())
                .as_deref(),
        )
    }

    pub fn load(
        vertex_shader: Option<impl AsRef<Path>>,
        fragment_shader: Option<impl AsRef<Path>>,
    ) -> io::Result<Self> {
        let vertex = vertex_shader
            .map(crate::util::read_to_c_string)
            .transpose()?;
        let fragment = fragment_shader
            .map(crate::util::read_to_c_string)
            .transpose()?;

        if let Some(image) = Self::load_from_c_strings(vertex.as_deref(), fragment.as_deref()) {
            Ok(image)
        } else {
            Err(std::io::Error::other("Unable to load image"))
        }
    }

    pub fn get_location<T: ShaderUniform>(
        &self,
        uniform_name: impl AsRef<str>,
    ) -> Option<ShaderLocation<T>> {
        let id = unsafe {
            sys::GetShaderLocation(
                *self.0,
                CString::new(uniform_name.as_ref())
                    .expect("Infallible")
                    .as_ptr(),
            )
        };

        ShaderLocation::from_id(self.clone(), id)
    }
}

pub struct ShaderModeGuard<'a>(PhantomData<&'a ()>);

impl Drop for ShaderModeGuard<'_> {
    fn drop(&mut self) {
        unsafe { sys::EndShaderMode() }
    }
}

impl Shader {
    fn begin_mode(&self) -> ShaderModeGuard<'_> {
        unsafe { sys::BeginShaderMode(*self.0) }
        ShaderModeGuard(PhantomData)
    }

    pub fn with<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        let guard = self.begin_mode();
        f();
        drop(guard);
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! __shader_inner {
    ($({$($tt: tt)*})*) => {
        const {
            unsafe {
                ::core::ffi::CStr::from_bytes_with_nul_unchecked(
                    concat!(
                        $(
                            stringify!($($tt)*),
                            "\n",
                        )*
                        "\0"
                    ).as_bytes()
                )
            }
        }
    };
    ($lit: literal) => {
        const {
            unsafe {
                ::core::ffi::CStr::from_bytes_with_nul_unchecked(
                    concat!($lit, "\0").as_bytes()
                )
            }
        }
    };
}

/// A handy way to create a shader.  Because some shader items require a newline, the top-level
/// items must be wrapped with `{}` and will be joined using newlines:
///
/// ```rust
/// # use rl::shader;
/// shader! {
///     vertex {
///         { #version 330}
///         {
///             void main() {
///                 ...
///             }
///         }
///     }
///     fragment {
///         { #version 330}
///         {
///             void main() {
///                 ...
///             }
///         }
///     }
/// }
/// # ;
/// ```
///
/// # Note
///
/// This does compile or validate shaders at compile-time, but does bake them into the binary.
#[macro_export]
macro_rules! shader {
    {vertex { $($vert:tt)* }$(,)?} => {
        $crate::shader::Shader::load_from_c_strings(
            Some($crate::__shader_inner!($($vert)*)),
            None,
        )
    };
    {fragment { $($frag:tt)* }$(,)?} => {
        $crate::shader::Shader::load_from_c_strings(
            None,
            Some($crate::__shader_inner!($($frag)*)),
        )
    };
    {vertex { $($vert:tt)* }$(,)? fragment { $($frag:tt)* }$(,)?} => {
        $crate::shader::Shader::load_from_c_strings(
            Some($crate::__shader_inner!($($vert)*)),
            Some($crate::__shader_inner!($($frag)*)),
        )
    };
}
