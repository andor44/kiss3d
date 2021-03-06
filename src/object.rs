//! Data structure of a scene node.

use std::ptr;
use std::cast;
use std::borrow;
use std::rc::{RcMut, Rc};
use gl;
use gl::types::*;
use nalgebra::na::{Mat3, Mat4, Vec3, Iso3, Rotation, Rotate, Translation, Transformation};
use nalgebra::na;
use resources::shaders_manager::ObjectShaderContext;
use resources::textures_manager;
use resources::textures_manager::Texture;
use mesh::Mesh;

#[path = "error.rs"]
mod error;

type Transform3d = Iso3<f32>;
type Scale3d     = Mat3<GLfloat>;

/// Set of datas identifying a scene node.
pub struct ObjectData {
    priv texture:   Rc<Texture>,
    priv scale:     Scale3d,
    priv transform: Transform3d,
    priv color:     Vec3<f32>,
    priv visible:   bool
}

/// Structure of all 3d objects on the scene. This is the only interface to manipulate the object
/// position, color, vertices and texture.
#[deriving(Clone)]
pub struct Object {
    priv data:    RcMut<ObjectData>,
    priv mesh:    RcMut<Mesh>
}

impl Object {
    #[doc(hidden)]
    pub fn new(mesh:     RcMut<Mesh>,
               r:        f32,
               g:        f32,
               b:        f32,
               texture:  Rc<Texture>,
               sx:       GLfloat,
               sy:       GLfloat,
               sz:       GLfloat) -> Object {
        let data = ObjectData {
            scale:     Mat3::new(sx, 0.0, 0.0,
                                 0.0, sy, 0.0,
                                 0.0, 0.0, sz),
            transform: na::one(),
            color:     Vec3::new(r, g, b),
            texture:   texture,
            visible:   true
        };

        Object {
            data:    RcMut::new(data),
            mesh:    mesh,
        }
    }

    #[doc(hidden)]
    pub fn upload(&self, context: &ObjectShaderContext) {
        do self.data.with_borrow |data| {
            if data.visible {
                let formated_transform:  Mat4<f32> = na::to_homogeneous(&data.transform);
                let formated_ntransform: Mat3<f32> = *data.transform.rotation.submat();

                // we convert the matrix elements
                unsafe {
                    verify!(gl::UniformMatrix4fv(context.transform,
                                                 1,
                                                 gl::FALSE as u8,
                                                 cast::transmute(&formated_transform)));

                    verify!(gl::UniformMatrix3fv(context.ntransform,
                                                 1,
                                                 gl::FALSE as u8,
                                                 cast::transmute(&formated_ntransform)));

                    verify!(gl::UniformMatrix3fv(context.scale, 1, gl::FALSE as u8, cast::transmute(&data.scale)));

                    verify!(gl::Uniform3f(context.color, data.color.x, data.color.y, data.color.z));

                    // FIXME: we should not switch the buffers if the last drawn shape uses the same.
                    self.mesh.with_borrow(|m| m.bind(context.pos, context.normal, context.tex_coord));

                    verify!(gl::ActiveTexture(gl::TEXTURE0));
                    verify!(gl::BindTexture(gl::TEXTURE_2D, self.data.with_borrow(|d| d.texture.borrow().id())));

                    verify!(gl::DrawElements(gl::TRIANGLES,
                                             self.mesh.with_borrow(|m| m.num_pts()) as GLint,
                                             gl::UNSIGNED_INT,
                                             ptr::null()));

                    self.mesh.with_borrow(|m| m.unbind());
                }
            }
        }
    }

    /// Sets the visible state of this object. An invisible object does not draw itself.
    pub fn set_visible(&mut self, visible: bool) {
        self.data.with_mut_borrow(|d| d.visible = visible)
    }

    /// Returns true if this object can be visible.
    pub fn visible(&self) -> bool {
        self.data.with_borrow(|d| d.visible)
    }

    /// Sets the local scaling factor of the object.
    pub fn set_scale(&mut self, sx: f32, sy: f32, sz: f32) {
        do self.data.with_mut_borrow |d| {
            d.scale = Mat3::new(
                sx, 0.0, 0.0,
                0.0, sy, 0.0,
                0.0, 0.0, sz)
        }
    }

    /// Get a write access to the geometry mesh. Return true if the geometry needs to be
    /// re-uploaded to the GPU.
    pub fn modify_mesh(&mut self, f: &fn(&mut Mesh) -> bool) {
        do self.mesh.with_mut_borrow |m| {
            if f(m) {
                // FIXME: find a way to upload only the modified parts.
                m.upload()
            }
        }
    }

    /// Sets the color of the object. Colors components must be on the range `[0.0, 1.0]`.
    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        do self.data.with_mut_borrow |d| {
            d.color.x = r;
            d.color.y = g;
            d.color.z = b;
        }
    }

    /// Sets the texture of the object.
    ///
    /// # Arguments
    ///   * `path` - relative path of the texture on the disk
    pub fn set_texture(&mut self, path: &str) {
        self.data.with_mut_borrow(|d| d.texture = textures_manager::singleton().add(path));
    }

    /// Move and orient the object such that it is placed at the point `eye` and have its `x` axis
    /// oriented toward `at`.
    pub fn look_at(&mut self, eye: &Vec3<f32>, at: &Vec3<f32>, up: &Vec3<f32>) {
        self.data.with_mut_borrow(|d| d.transform.look_at(eye, at, up))
    }

    /// Move and orient the object such that it is placed at the point `eye` and have its `z` axis
    /// oriented toward `at`.
    pub fn look_at_z(&mut self, eye: &Vec3<f32>, at: &Vec3<f32>, up: &Vec3<f32>) {
        self.data.with_mut_borrow(|d| d.transform.look_at_z(eye, at, up))
    }
}

impl Transformation<Transform3d> for Object {
    fn transformation(&self) -> Transform3d {
        self.data.with_borrow(|d| d.transform.clone())
    }

    fn inv_transformation(&self) -> Transform3d {
        self.data.with_borrow(|d| d.transform.inv_transformation())
    }

    fn append_transformation(&mut self, t: &Transform3d) {
        self.data.with_mut_borrow(|d| d.transform.append_transformation(t))
    }

    fn append_transformation_cpy(_: &Object, _: &Transform3d) -> Object {
        fail!("Cannot clone an object.")
    }

    fn prepend_transformation(&mut self, t: &Transform3d) {
        self.data.with_mut_borrow(|d| d.transform.prepend_transformation(t))
    }

    fn prepend_transformation_cpy(_: &Object, _: &Transform3d) -> Object {
        fail!("Cannot clone an object.")
    }

    fn set_transformation(&mut self, t: Transform3d) {
        self.data.with_mut_borrow(|d| d.transform.set_transformation(t))
    }
}

impl na::Transform<Vec3<f32>> for Object {
    fn transform(&self, v: &Vec3<f32>) -> Vec3<f32> {
        self.data.with_borrow(|d| d.transform.transform(v))
    }

    fn inv_transform(&self, v: &Vec3<f32>) -> Vec3<f32> {
        self.data.with_borrow(|d| d.transform.inv_transform(v))
    }
} 

impl Rotation<Vec3<f32>> for Object {
    fn rotation(&self) -> Vec3<f32> {
        self.data.with_borrow(|d| d.transform.rotation())
    }

    fn inv_rotation(&self) -> Vec3<f32> {
        self.data.with_borrow(|d| d.transform.inv_rotation())
    }

    fn append_rotation(&mut self, t: &Vec3<f32>) {
        self.data.with_mut_borrow(|d| d.transform.append_rotation(t))
    }

    fn append_rotation_cpy(_: &Object, _: &Vec3<f32>) -> Object {
        fail!("Cannot clone an object.")
    }

    fn prepend_rotation(&mut self, t: &Vec3<f32>) {
        self.data.with_mut_borrow(|d| d.transform.prepend_rotation(t))
    }

    fn prepend_rotation_cpy(_: &Object, _: &Vec3<f32>) -> Object {
        fail!("Cannot clone an object.")
    }

    fn set_rotation(&mut self, r: Vec3<f32>) {
        self.data.with_mut_borrow(|d| d.transform.set_rotation(r))
    }
}

impl Rotate<Vec3<f32>> for Object {
    fn rotate(&self, v: &Vec3<f32>) -> Vec3<f32> {
        self.data.with_borrow(|d| d.transform.rotate(v))
    }

    fn inv_rotate(&self, v: &Vec3<f32>) -> Vec3<f32> {
        self.data.with_borrow(|d| d.transform.inv_rotate(v))
    }
} 

impl Translation<Vec3<f32>> for Object {
    fn translation(&self) -> Vec3<f32> {
        self.data.with_borrow(|d| d.transform.translation())
    }

    fn inv_translation(&self) -> Vec3<f32> {
        self.data.with_borrow(|d| d.transform.inv_translation())
    }

    fn append_translation(&mut self, t: &Vec3<f32>) {
        self.data.with_mut_borrow(|d| d.transform.append_translation(t))
    }

    fn append_translation_cpy(_: &Object, _: &Vec3<f32>) -> Object {
        fail!("Cannot clone an object.")
    }

    fn prepend_translation(&mut self, t: &Vec3<f32>) {
        self.data.with_mut_borrow(|d| d.transform.prepend_translation(t))
    }

    fn prepend_translation_cpy(_: &Object, _: &Vec3<f32>) -> Object {
        fail!("Cannot clone an object.")
    }

    fn set_translation(&mut self, t: Vec3<f32>) {
        self.data.with_mut_borrow(|d| d.transform.set_translation(t))
    }
}

impl Eq for Object {
    fn eq(&self, other: &Object) -> bool {
        self.data.with_borrow(|d1| other.data.with_borrow(|d2| borrow::ref_eq(d1, d2)))
    }
}
