//! Graphics context.
//!
//! # Graphics context and backends
//!
//! A graphics context is an external type typically implemented by other crates and which provides
//! support for backends. Its main scope is to unify all possible implementations of backends
//! behind a single trait: [`GraphicsContext`]. A [`GraphicsContext`] really only requires two items
//! to be implemented:
//!
//! - The type of the backend to use — [`GraphicsContext::Backend`]. That type will often be used
//!   to access the GPU, cache costly operations, etc.
//! - A method to get a mutable access to the underlying backend — [`GraphicsContext::backend`].
//!
//! Most of the time, if you want to work with _any_ windowing implementation, you will want to
//! use a type variable such as `C: GraphicsContext`. If you want to work with any context
//! supporting a specific backend, use `C: GraphicsContext<Backend = YourBackendType`. Etc.
//!
//! This crate doesn’t provide you with creating such contexts. Instead, you must do it yourself
//! or rely on crates doing it for you.
//!
//! # Default implementation of helper functions
//!
//! By default, graphics contexts automatically get several methods implemented on them. Those
//! methods are helper functions available to write code in a more elegant and compact way than
//! passing around mutable references on the context. Often, it will help you not having to
//! use type ascription, too, since the [`GraphicsContext::Backend`] type is known when calling
//! those functions.
//!
//! Instead of:
//!
//! ```ignore
//! use luminance::context::GraphicsContext as _;
//! use luminance::buffer::Buffer;
//!
//! let buffer: Buffer<SomeBackendType, u8> = Buffer::from_slice(&mut context, slice).unwrap();
//! ```
//!
//! You can simply do:
//!
//! ```ignore
//! use luminance::context::GraphicsContext as _;
//!
//! let buffer = context.new_buffer_from_slice(slice).unwrap();
//! ```

use crate::backend::buffer::Buffer as BufferBackend;
use crate::backend::color_slot::ColorSlot;
use crate::backend::depth_slot::DepthSlot;
use crate::backend::framebuffer::Framebuffer as FramebufferBackend;
use crate::backend::shader::Shader;
use crate::backend::tess::Tess as TessBackend;
use crate::backend::texture::Texture as TextureBackend;
use crate::buffer::{Buffer, BufferError};
use crate::framebuffer::{Framebuffer, FramebufferError};
use crate::pipeline::PipelineGate;
use crate::pixel::Pixel;
use crate::shader::{ProgramBuilder, Stage, StageError, StageType};
use crate::tess::{Deinterleaved, Interleaved, TessBuilder, TessVertexData};
use crate::texture::{Dimensionable, Sampler, Texture, TextureError};
use crate::vertex::Semantics;

/// Class of graphics context.
///
/// Graphics context must implement this trait to be able to be used throughout the rest of the
/// crate.
pub unsafe trait GraphicsContext: Sized {
  /// Internal type used by the backend to cache, optimize and store data. This roughly represents
  /// the GPU data / context a backend implementation needs to work correctly.
  type Backend: ?Sized;

  /// Access the underlying backend.
  fn backend(&mut self) -> &mut Self::Backend;

  /// Create a new pipeline gate
  fn new_pipeline_gate(&mut self) -> PipelineGate<Self> {
    PipelineGate::new(self)
  }

  /// Create a new buffer.
  ///
  /// See the documentation of [`Buffer::new`] for further details.
  fn new_buffer<T>(&mut self, len: usize) -> Result<Buffer<Self::Backend, T>, BufferError>
  where
    Self::Backend: BufferBackend<T>,
    T: Copy + Default,
  {
    Buffer::new(self, len)
  }

  /// Create a new buffer from a slice.
  ///
  /// See the documentation of [`Buffer::from_vec`] for further details.
  fn new_buffer_from_vec<T, X>(&mut self, vec: X) -> Result<Buffer<Self::Backend, T>, BufferError>
  where
    Self::Backend: BufferBackend<T>,
    X: Into<Vec<T>>,
    T: Copy,
  {
    Buffer::from_vec(self, vec)
  }

  /// Create a new buffer by repeating a value.
  ///
  /// See the documentation of [`Buffer::repeat`] for further details.
  fn new_buffer_repeating<T>(
    &mut self,
    len: usize,
    value: T,
  ) -> Result<Buffer<Self::Backend, T>, BufferError>
  where
    Self::Backend: BufferBackend<T>,
    T: Copy,
  {
    Buffer::repeat(self, len, value)
  }

  /// Create a new framebuffer.
  ///
  /// See the documentation of [`Framebuffer::new`] for further details.
  fn new_framebuffer<D, CS, DS>(
    &mut self,
    size: D::Size,
    mipmaps: usize,
    sampler: Sampler,
  ) -> Result<Framebuffer<Self::Backend, D, CS, DS>, FramebufferError>
  where
    Self::Backend: FramebufferBackend<D>,
    D: Dimensionable,
    CS: ColorSlot<Self::Backend, D>,
    DS: DepthSlot<Self::Backend, D>,
  {
    Framebuffer::new(self, size, mipmaps, sampler)
  }

  /// Create a new shader stage.
  ///
  /// See the documentation of [`Stage::new`] for further details.
  fn new_shader_stage<R>(
    &mut self,
    ty: StageType,
    src: R,
  ) -> Result<Stage<Self::Backend>, StageError>
  where
    Self::Backend: Shader,
    R: AsRef<str>,
  {
    Stage::new(self, ty, src)
  }

  /// Create a new shader program.
  ///
  /// See the documentation of [`ProgramBuilder::new`] for further details.
  fn new_shader_program<Sem, Out, Uni>(&mut self) -> ProgramBuilder<Self, Sem, Out, Uni>
  where
    Self::Backend: Shader,
    Sem: Semantics,
  {
    ProgramBuilder::new(self)
  }

  /// Create a [`TessBuilder`].
  ///
  /// See the documentation of [`TessBuilder::new`] for further details.
  fn new_tess(&mut self) -> TessBuilder<Self::Backend, (), (), (), Interleaved>
  where
    Self::Backend: TessBackend<(), (), (), Interleaved>,
  {
    TessBuilder::new(self)
  }

  /// Create a [`TessBuilder`] with deinterleaved memory.
  ///
  /// See the documentation of [`TessBuilder::new`] for further details.
  fn new_deinterleaved_tess<V, W>(&mut self) -> TessBuilder<Self::Backend, V, (), W, Deinterleaved>
  where
    Self::Backend: TessBackend<V, (), W, Deinterleaved>,
    V: TessVertexData<Deinterleaved>,
    W: TessVertexData<Deinterleaved>,
  {
    TessBuilder::new(self)
  }

  /// Create a new texture.
  ///
  /// Feel free to have a look at the documentation of [`Texture::new`] for further details.
  fn new_texture<D, P>(
    &mut self,
    size: D::Size,
    mipmaps: usize,
    sampler: Sampler,
  ) -> Result<Texture<Self::Backend, D, P>, TextureError>
  where
    Self::Backend: TextureBackend<D, P>,
    D: Dimensionable,
    P: Pixel,
  {
    Texture::new(self, size, mipmaps, sampler)
  }
}
