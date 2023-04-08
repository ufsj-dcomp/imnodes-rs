#![deny(missing_docs)]

/*!
Bindings for [imnodes](https://github.com/Nelarius/imnodes) using [cimnodes](https://github.com/cimgui/cimnodes)
*/

use imnodes_sys as sys;

/// export all low level functions
#[cfg(feature = "include_low_level_bindings")]
pub mod internal {
    pub use imnodes_sys::*;
}

mod context;
pub use context::*;

mod helpers;
pub use helpers::*;

mod styling;
pub use styling::*;

mod scopes;
pub use scopes::*;

// maybe wrap those (same decision as in implot-rs)
pub use sys::{ImNodesStyle, ImVec2};

/// Identifier for Attributes in nodes
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct AttributeId(pub i32);

impl Into<i32> for AttributeId {
    fn into(self) -> i32 {
        self.0
    }
}

/// The node's position can be expressed in three coordinate systems:
/// * screen space coordinates, -- the origin is the upper left corner of the window.
/// * editor space coordinates -- the origin is the upper left corner of the node editor window
/// * grid space coordinates, -- the origin is the upper left corner of the node editor window,
///
/// translated by the current editor panning vector (see [EditorContext::get_panning()] and
/// [EditorContext::reset_panning()])
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum CoordinateSystem {
    /// probably what you want
    ///
    /// the origin is the upper left corner of the window
    ScreenSpace,
    /// the origin is the upper left corner of the node editor window
    EditorSpace,
    /// the origin is the upper left corner of the node editor window
    GridSpace,
}

/// Identifier for a Node
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeId(pub i32);

impl NodeId {
    /// Enable or disable the ability to click and drag a specific node.
    #[doc(alias = "SetNodeDraggable")]
    pub fn set_draggable(&self, draggable: bool) -> &Self {
        unsafe { sys::imnodes_SetNodeDraggable(self.0, draggable) };
        self
    }

    /// EditorContextMoveToNode
    #[doc(alias = "EditorContextMoveToNode")]
    pub fn move_editor_to(&self) -> &Self {
        unsafe { sys::imnodes_EditorContextMoveToNode(self.0) };
        self
    }

    /// get the size of the node
    #[doc(alias = "GetNodeDimensions")]
    pub fn get_dimensions(&self) -> ImVec2 {
        let mut dimension = ImVec2 { x: 0.0, y: 0.0 };
        unsafe { sys::imnodes_GetNodeDimensions(&mut dimension as _, self.0) };
        dimension
    }

    /// move the node
    #[doc(
        alias = "SetNodeScreenSpacePos",
        alias = "SetNodeEditorSpacePos",
        alias = "SetNodeGridSpacePos"
    )]
    pub fn set_position(&self, x: f32, y: f32, coordinate_sytem: CoordinateSystem) -> &Self {
        let pos = ImVec2 { x, y };
        match coordinate_sytem {
            CoordinateSystem::ScreenSpace => unsafe {
                sys::imnodes_SetNodeScreenSpacePos(self.0, pos)
            },
            CoordinateSystem::EditorSpace => unsafe {
                sys::imnodes_SetNodeEditorSpacePos(self.0, pos)
            },
            CoordinateSystem::GridSpace => unsafe {
                sys::imnodes_SetNodeGridSpacePos(self.0, pos)
            },
        };
        self
    }

    /// get the position of the node
    #[doc(
        alias = "GetNodeScreenSpacePos",
        alias = "GetNodeEditorSpacePos",
        alias = "GetNodeGridSpacePos"
    )]
    pub fn get_position(&self, coordinate_sytem: CoordinateSystem) -> ImVec2 {
        let mut pos = ImVec2 { x: 0.0, y: 0.0 };

        match coordinate_sytem {
            CoordinateSystem::ScreenSpace => unsafe {
                sys::imnodes_GetNodeScreenSpacePos(&mut pos as _, self.0)
            },
            CoordinateSystem::EditorSpace => unsafe {
                sys::imnodes_GetNodeEditorSpacePos(&mut pos as _, self.0)
            },
            CoordinateSystem::GridSpace => unsafe {
                sys::imnodes_GetNodeGridSpacePos(&mut pos as _, self.0)
            },
        };

        pos
    }
}

impl Into<i32> for NodeId {
    fn into(self) -> i32 {
        self.0
    }
}

/// either input or output pin
///
/// like attribute_id in the original source
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PinId(pub i32);

impl PinId {
    /// TODO test
    ///
    /// Did the user start dragging a new link from a pin?
    #[doc(alias = "IsLinkStarted")]
    pub fn is_start_of_link(&self, scope: &OuterScope) -> bool {
        Some(*self) == scope.from_where_link_started()
    }

    /// TODO test
    ///
    /// Did the user drop the dragged link before attaching it to a pin?
    /// There are two different kinds of situations to consider when handling this event:
    /// 1) a link which is created at a pin and then dropped
    /// 2) an existing link which is detached from a pin and then dropped
    ///
    /// Use the including_detached_links flag to control whether this function triggers when the user detaches a link and drops it.
    #[doc(alias = "IsLinkDropped")]
    pub fn dropped_link(&self, including_detached_links: bool, scope: &OuterScope) -> bool {
        Some(*self) == scope.from_where_link_dropped(including_detached_links)
    }
}

/// Id for an input
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct InputPinId(pub i32);

impl Into<i32> for InputPinId {
    fn into(self) -> i32 {
        self.0
    }
}

impl Into<PinId> for InputPinId {
    fn into(self) -> PinId {
        PinId(self.0)
    }
}

/// Id for an output
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct OutputPinId(pub i32);

impl Into<i32> for OutputPinId {
    fn into(self) -> i32 {
        self.0
    }
}

impl Into<PinId> for OutputPinId {
    fn into(self) -> PinId {
        PinId(self.0)
    }
}

/// Id for a link
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct LinkId(pub i32);

impl LinkId {
    /// checks if the link of this LinkId got removed
    #[doc(alias = "IsLinkDestroyed")]
    pub fn is_removed(&self, scope: &OuterScope) -> bool {
        Some(*self) == scope.get_dropped_link()
    }
}

impl Into<i32> for LinkId {
    fn into(self) -> i32 {
        self.0
    }
}

/// makes it possible to detect if the mouse if at the positoin of the element
pub trait Hoverable {
    /// The following functions return true if a UI element is being hovered over by the mouse cursor.
    /// Assigns the id of the UI element being hovered over to the function argument.
    ///
    /// there is also [`crate::scopes::EditorScope::is_hovered()`] which does not depend on the scope
    #[doc(
        alias = "IsPinHovered",
        alias = "IsNodeHovered",
        alias = "IsLinkHovered"
    )]
    fn is_hovered(&self, _: &OuterScope) -> bool;
}

impl Hoverable for OutputPinId {
    /// isPinHovered
    #[doc(alias = "IsPinHovered")]
    fn is_hovered(&self, scope: &OuterScope) -> bool {
        Some(PinId(self.0)) == scope.get_hovered_pin()
    }
}

impl Hoverable for InputPinId {
    /// isPinHovered
    #[doc(alias = "IsPinHovered")]
    fn is_hovered(&self, scope: &OuterScope) -> bool {
        Some(PinId(self.0)) == scope.get_hovered_pin()
    }
}

impl Hoverable for NodeId {
    /// isNodeHovered
    #[doc(alias = "IsNodeHovered")]
    fn is_hovered(&self, _: &OuterScope) -> bool {
        Some(*self) == get_hovered_node()
    }
}

impl Hoverable for LinkId {
    /// isLinkHovered
    #[doc(alias = "IsLinkHovered")]
    fn is_hovered(&self, scope: &OuterScope) -> bool {
        Some(*self) == scope.get_hovered_link()
    }
}

/// IsNodeHovered
#[doc(alias = "IsNodeHovered")]
pub fn get_hovered_node() -> Option<NodeId> {
    let mut id: i32 = -1;
    let ok = unsafe { sys::imnodes_IsNodeHovered(&mut id as _) };
    if ok {
        Some(NodeId(id))
    } else {
        None
    }
}

#[allow(missing_docs)]
/// the cpp code makes sure to put the input and output types in the right fields
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Link {
    pub start_node: NodeId,
    pub end_node: NodeId,
    pub start_pin: OutputPinId,
    pub end_pin: InputPinId,
    pub craeated_from_snap: bool,
}
