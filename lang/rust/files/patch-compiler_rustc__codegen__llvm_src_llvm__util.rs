LLVM 23 removed AMX-TF32. Do not query or pass this feature to LLVM 23+.
https://github.com/llvm/llvm-project/pull/207673

--- compiler/rustc_codegen_llvm/src/llvm_util.rs.orig
+++ compiler/rustc_codegen_llvm/src/llvm_util.rs
@@ -272,6 +272,8 @@
         },
         Arch::X86 | Arch::X86_64 => {
             match s {
+                // LLVM 23 removed support for AMX-TF32.
+                "amx-tf32" if major >= 23 => None,
                 "sse4.2" => Some(LLVMFeature::with_dependencies(
                     "sse4.2",
                     smallvec![TargetFeatureFoldStrength::EnableOnly("crc32")],
