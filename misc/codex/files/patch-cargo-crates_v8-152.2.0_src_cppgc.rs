Backport Codex's V8 fork at 350c5a859a9b3c84ab8701b3ad51e3b7699b30c0.
Keep Member and WeakMember storage aligned with their C++ representation.
Use the crates.io source distribution, which includes the required submodules.

--- cargo-crates/v8-152.2.0/src/cppgc.rs.orig
+++ cargo-crates/v8-152.2.0/src/cppgc.rs
@@ -451,11 +451,26 @@
   }
 }

+// C++ stores a compressed uint32_t or an uncompressed pointer. The binding
+// size selects the alignment; MaybeUninit keeps the slot opaque to Rust.
+trait MemberStorage {
+  type Aligned;
+}
+
+impl MemberStorage for [u8; 4] {
+  type Aligned = std::mem::MaybeUninit<u32>;
+}
+impl MemberStorage for [u8; 8] {
+  type Aligned = std::mem::MaybeUninit<u64>;
+}
+
 macro_rules! member {
   ($( # $attr:tt )* $name:ident) => {
     paste::paste! {
       #[repr(transparent)]
-      struct [< $name Inner >]([u8; crate::binding:: [< cppgc__ $name _SIZE >]]);
+      struct [< $name Inner >](
+        <[u8; crate::binding:: [< cppgc__ $name _SIZE >]] as MemberStorage>::Aligned,
+      );

       impl [< $name Inner >] {
         fn new(ptr: *mut RustObj) -> Self {
@@ -584,6 +599,36 @@
   WeakMember
 }

+#[cfg(test)]
+mod member_layout_tests {
+  use super::*;
+  use std::mem::{align_of, size_of};
+
+  struct Node;
+
+  // SAFETY: Node owns no managed pointers.
+  unsafe impl GarbageCollected for Node {
+    fn trace(&self, _: &mut Visitor) {}
+    fn get_name(&self) -> &'static CStr {
+      c"Node"
+    }
+  }
+
+  fn check<T>(cpp_size: usize) {
+    assert_eq!(size_of::<T>(), cpp_size);
+    assert_eq!(align_of::<T>(), cpp_size);
+  }
+
+  #[test]
+  fn cppgc_member_storage_matches_cpp_alignment() {
+    // Do not construct members: this must fail safely on a misaligned layout.
+    check::<MemberInner>(crate::binding::cppgc__Member_SIZE);
+    check::<WeakMemberInner>(crate::binding::cppgc__WeakMember_SIZE);
+    check::<Member<Node>>(crate::binding::cppgc__Member_SIZE);
+    check::<WeakMember<Node>>(crate::binding::cppgc__WeakMember_SIZE);
+  }
+}
+
 macro_rules! persistent {
   ($( # $attr:tt )* $name:ident) => {
     paste::paste! {
