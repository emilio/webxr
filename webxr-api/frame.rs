/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::InputFrame;
use crate::Native;
use crate::Viewer;

use euclid::TypedRigidTransform3D;

/// The per-frame data that is provided by the device.
/// https://www.w3.org/TR/webxr/#xrframe
// TODO: other fields?
#[derive(Clone, Debug)]
#[cfg_attr(feature = "ipc", derive(serde::Serialize, serde::Deserialize))]
pub struct Frame {
    /// The transform from the viewer to native coordinates
    ///
    /// This is equivalent to the pose of the viewer in native coordinates.
    /// This is the inverse of the view matrix.
    pub transform: TypedRigidTransform3D<f32, Viewer, Native>,

    /// Frame information for each connected input source
    pub inputs: Vec<InputFrame>,
}
