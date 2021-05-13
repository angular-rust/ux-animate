use super::KeyPathElement;
use std::fmt;

/// Defines which content to target.
/// The keypath can contain wildcards ('*') with match exactly 1 item.
/// or globstars ('**') which match 0 or more items.
///
/// For example, if your content were arranged like this:
/// Gabriel (Shape Layer)
///     Body (Shape Group)
///         Left Hand (Shape)
///             Fill (Fill)
///             Transform (Transform)
///         ...
/// Brandon (Shape Layer)
///     Body (Shape Group)
///         Left Hand (Shape)
///             Fill (Fill)
///             Transform (Transform)
///         ...
///
///
/// You could:
///     Match Gabriel left hand fill:
///        new KeyPath("Gabriel", "Body", "Left Hand", "Fill");
///     Match Gabriel and Brandon's left hand fill:
///        new KeyPath("*", "Body", Left Hand", "Fill");
///     Match anything with the name Fill:
///        new KeyPath("**", "Fill");
///
///
/// NOTE: Content that are part of merge paths or repeaters cannot currently be resolved with
/// a {@link KeyPath}. This may be fixed in the future.

pub struct KeyPath {
    keys: Vec<String>,
    // resolved_element: Option<Box<dyn KeyPathElement>>,
}

impl KeyPath {
    pub fn new(keys: Vec<String>) -> Self {
        Self {
            // resolved_element: None,
            keys,
        }
    }

    /// Copy constructor. Copies keys as well.
    // KeyPath.copy(KeyPath keyPath)
    //     : keys = keyPath.keys.toList(),
    //         _resolvedElement = keyPath._resolvedElement;

    /// Returns a new KeyPath with the key added.
    /// This is used during keypath resolution. Children normally don't know about all of their parent
    /// elements so this is used to keep track of the fully qualified keypath.
    /// This returns a key keypath because during resolution, the full keypath element tree is walked
    /// and if this modified the original copy, it would remain after popping back up the element tree.
    //@CheckResult

    fn add_key(&self, key: String) {
        unimplemented!()
    }

    /// Return a new KeyPath with the element resolved to the specified {@link KeyPathElement}.
    fn resolve(&self, element: impl KeyPathElement) -> KeyPath {
        unimplemented!()
    }

    // /// Returns a {@link KeyPathElement} that this has been resolved to. KeyPaths get resolved with
    // /// resolveKeyPath on LottieDrawable or LottieAnimationView.
    // fn get_resolved_element(&self) -> impl KeyPathElement {
    //     unimplemented!()
    // }

    /// Returns whether they key matches at the specified depth.
    fn matches(&self, key: String, depth: i64) -> bool {
        unimplemented!()
    }

    /// For a given key and depth, returns how much the depth should be incremented by when
    /// resolving a keypath to children.
    ///
    /// This can be 0 or 2 when there is a globstar and the next key either matches or doesn't match
    /// the current key.
    fn increment_depth_by(&self, key: String, depth: i64) -> i64 {
        unimplemented!()
    }

    /// Returns whether the key at specified depth is fully specific enough to match the full set of
    /// keys in this keypath.
    fn fully_resolves_to(&self, key: String, depth: i64) -> bool {
        unimplemented!()
    }

    /// Returns whether the keypath resolution should propagate to children. Some keypaths resolve
    /// to content other than leaf contents (such as a layer or content group transform) so sometimes
    /// this will return false.
    fn propogate_to_children(&self, key: String, depth: i64) -> bool {
        unimplemented!()
    }

    /// We artificially create some container groups (like a root ContentGroup for the entire animation
    /// and for the contents of a ShapeLayer).
    fn is_container(&self, key: String) -> bool {
        unimplemented!()
    }
    fn ends_with_globstart(&self) -> bool {
        unimplemented!()
    }

    fn keys_to_string(&self) -> String {
        unimplemented!()
    }
}

impl fmt::Display for KeyPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //keys={}, self.keys , resolved={{{}}}, self.resolved_element
        write!(f, "KeyPath[]")
    }
}
