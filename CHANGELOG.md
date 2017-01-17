## 0.16.0

- `BufferSlice{,Mut}` now implements `IntoIterator`.
- Some internal changes.
- Vertices (`Vertex`) are now aligned based on what decides the Rust compiler. This is very
  important, especially because of the version 0.15.0 adding non-32-bit vertex components: alignment
  and padding is now completely handled for you and you have nothing to care about.
- Added some documentation here and there.
- Changed the meaning of the semantic maps (uniforms). It is now required to provide a `Uniform` to
  build a new `Sem`. This is an improvement in the sense that the *unsafe* zone is restricted to the
  declaration of uniforms for a given program. This *unsafe* zone will be covered in a next release
  by a macro to make it safe.
- `texturee::Unit` cannot be used in a uniform block context (it doesn’t have sense on the GLSL
  side).
- Added some more types to `UniformBlock`. This trait is not very useful yet, but it’s required to
  make a `Buffer` readable from GLSL.

## 0.15.0

- Texture and framebuffers have several functions that can fail with new errors.
- Added buffer mapping. See `BufferSlice` and `BufferSliceMut` for further details.
- `Tessellation` is now called `Tess` for simplicity (because it’s used **a lot**).
- `VertexComponentFormat::comp_size` is now a `usize`.
- The `Vertex` trait now accepts a lot more of new types (among changes, added support for
  non-32-bit vertex components).

## 0.14.0

- `UniformWarning::TypeMismatch` now includes the name of the uniform which type mismatches with the
  requested on.
- `Pipeline::Pipe` is now exported by the common interface.
- `Uniform::sem` doesn’t require `&self` anymore.
- `Uniform::new` is now a const fn.

### 0.13.1

- Added `Uniform::sem()` function to create `Sem` out of `Uniform<C, T>` in a simpler way.

## 0.13.0

- Changed the pipeline workflow by introducing `Pipe` objects.
- Removed strong typing in shader programs (`Program<C, T>` is now `Program<C>`).
- Removed strong typing in shader stages (`Stage<C, T>` is now `Stage<C>`).

### 0.12.1

- Added `Binding` and `Unit` in the default export-list.

## 0.12.0

- Added attribute-less tessellations.
- Enhanced shader-related documentation.
- Removed `Slot`.

## 0.11.0

- Added the first, constraint-only, `UniformBlock` trait. Used to restrict types that can be passed
  around in uniform buffers.
- Added *proxied* types – i.e. `TextureProxy` and `UniformBufferProxy`. Those are used to build sets
  that can passed to shaders.
- Uniform buffers can now be sent to shaders via the `buffer::Binding` type. Put your buffer in a
  uniform buffer set and there you go.
- Textures are not `Uniformable` anymore. You have to put the texture in a texture set and use a
  `texture::Unit`, which is `Uniformable`.
- Added `buffer::Binding`.
- Added `texture::Unit`.
- `map_uniform` now takes `&str` instead of `String`.
- Cleaned up documentation.
- Fixed internal bugs.

## 0.10.0

- Fixed the type of the uniform errors in the uniform interface builder function.

### 0.9.1

- Documentation updates.

## 0.9.0

- Several textures can now be passed as uniforms to shaders. The interface is pretty instable as it
  might change in the future, because for now, the user has to pass the texture units each textures
  should be bound to.

## 0.8.0

- Documentation is now available on docs.rs.
- Replaced references to `Vec` by slices.

## 0.7.0

- Uniforms have new semantics; mapping them cannot fail anymore but it can be emitted warnings.

### 0.6.4

- Backends now have more information to work with about uniforms. That enables using reification
  techniques to validate types, dimensions, sizes, etc…

### 0.6.3

- Added `get_raw_texels` to `Texture`.

### 0.6.2

- Added `upload_part_raw` and `upload_raw` to `Texture`, enabling to upload raw texels instead of
  texels directly.
- Added `RawEncoding` to `Pixel`.

### 0.6.1

- Added documentation field in Cargo.toml.

## 0.6.0

- Removed `Default` implementation for `Framebuffer` and added a new `default()` method, taking the
  size of the `Framebuffer`.
- Added `RenderablePixel` trait bound on `Slot` implementations for `ColorPixel`.
- Added `RenderablePixel`.
- Removed the need of the **core** crate.
- Removed `UniformName`.
- We can now have textures as uniforms.
- New uniform system to accept value depending on the backend.
- Using `AsRef` instead of ATexture in `update_textures`.
- Changed the meaning of mipmaps (now it’s the number of extra mipmaps).
- Using `usize` instead of `u32` for mipmaps.
- Added `Dimensionable` and `Layerable` in the interface.

### 0.5.3

- Added `update_textures` into `HasUniform`.
- Fixed signature of `UniformUpdate::update`.
- Fixed trait bound on `UniformUpdate::{contramap, update}`.

### 0.5.2

- Added `UniformUpdate`.
- Added `Uniformable` in the public interfarce shortcut.

### 0.5.1

- Removed `run_pipeline` and added `Pipeline::run`.

## 0.5.0

- Fixed uniform interfaces in `ShadingCommand` and `RenderCommand` with existential quantification.
- Renamed `FrameCommand` into `Pipeline`.
- Several patch fixes.
- Added travis CI support.
- Added documentation for `Program`.

## 0.4.0

- Changed the whole `Program` API to make it safer. Now, it closely looks like the Haskell version
  of `luminance`. The idea is that the user cannot have `Uniform`s around anymore as they’re held by
  `Program`s. Furthermore, the *uniform interface* is introduced. Because Rust doesn’t have a
  **“rank-2 types”** mechanism as Haskell, `ProgramProxy` is introduced to emulate such a behavior.
- Added a bit of documentation for shader programs.

### 0.3.1

- Removed `rw`.

## 0.3.0

- Removed A type parameter form `Buffer`. It was unnecessary safety that was never actually used.
- Added documentation around.

### 0.2.1

- Exposed `Vertex` directly in `luminance`.

## 0.2.0

- Changed `Negative*` blending factors to `*Complement`.

## 0.1.0

- Initial revision.