use super::KeyPath;
use crate::runtime::lottie::value::LottieValueCallback;
// import '../value/lottie_value_callback.dart';

/// Any item that can be a part of a {@link KeyPath} should implement this.
pub trait KeyPathElement {
    /// Called recursively during keypath resolution.
    ///
    /// The overridden method should just call:
    ///        MiscUtils.resolveKeyPath(keyPath, depth, accumulator, currentPartialKeyPath, this);
    ///
    /// @param keyPath The full keypath being resolved.
    /// @param depth The current depth that this element should be checked at in the keypath.
    /// @param accumulator A list of fully resolved keypaths. If this element fully matches the
    ///                    keypath then it should add itself to this list.
    /// @param currentPartialKeyPath A keypath that contains all parent element of this one.
    ///                              This element should create a copy of this and append itself
    ///                              with KeyPath#addKey when it adds itself to the accumulator
    ///                              or propagates resolution to its children.
    fn resolve_key_path(
        &self,
        key_path: KeyPath,
        depth: i64,
        accumulator: Vec<KeyPath>,
        current_partial_pey_path: KeyPath,
    );

    /// The overridden method should handle appropriate properties and set value callbacks on their
    /// animations.
    fn add_value_callback<T>(&self, property: T, callback: LottieValueCallback<T>);
}
