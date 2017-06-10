
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use v2::json::{camera, mesh, scene, skin, Extras, Index};

/// A node in the node hierarchy.  When the node contains `skin`, all `mesh.primitives` must contain `JOINTS_0` and `WEIGHTS_0` attributes.  A node can have either a `matrix` or any combination of `translation`/`rotation`/`scale` (TRS) properties. TRS properties are converted to matrices and postmultiplied in the `T * R * S` order to compose the transformation matrix; first the scale is applied to the vertices, then the rotation, and then the translation. If none are provided, the transform is the identity. When a node is targeted for animation (referenced by an animation.channel.target), only TRS properties may be present; `matrix` will not be present..
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Node {
    /// The index of the camera referenced by this node.
    pub camera: Option<Index<camera::Camera>>,
    
    /// The indices of this node's children.
    pub children: Option<Vec<Index<scene::Node>>>,

    /// Extension specific data.
    #[serde(default)]
    pub extensions: NodeExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// 4x4 column-major transformation matrix.
    #[serde(default = "node_matrix_default")]
    pub matrix: [f32; 16],

    /// The index of the mesh in this node.
    pub mesh: Option<Index<mesh::Mesh>>,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,
    
    /// The node's unit quaternion rotation in the order (x, y, z, w), where w is the scalar.
    #[serde(default = "node_rotation_default")]
    pub rotation: [f32; 4],

    /// The node's non-uniform scale.
    #[serde(default = "node_scale_default")]
    pub scale: [f32; 3],

    /// The node's translation.
    #[serde(default)]
    pub translation: [f32; 3],
    
    /// The index of the skin referenced by this node.
    pub skin: Option<Index<skin::Skin>>,
    
    /// The weights of the instantiated Morph Target. Number of elements must match number of Morph Targets of used mesh.
    pub weights: Option<Vec<f32>>,
}

/// Extension specific data for `Node`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NodeExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}

fn node_matrix_default() -> [f32; 16] {
    [1.0, 0.0, 0.0, 0.0, 
     0.0, 1.0, 0.0, 0.0, 
     0.0, 0.0, 1.0, 0.0, 
     0.0, 0.0, 0.0, 1.0]
}

fn node_rotation_default() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn node_scale_default() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

/// The root `Node`s of a scene.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Scene {
    /// Extension specific data.
    #[serde(default)]
    pub extensions: SceneExtensions,
    
    /// Optional application specific data.
    #[serde(default)]
    pub extras: Extras,
    
    /// Optional user-defined name for this object.
    pub name: Option<String>,

    /// The indices of each root node.
    pub nodes: Vec<Index<Node>>,
}

/// Extension specific data for `Scene`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SceneExtensions {
    #[serde(default)]
    _allow_unknown_fields: (),
}