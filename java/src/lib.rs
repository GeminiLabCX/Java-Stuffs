#[macro_use]
mod macros;

use annoy_rs::*;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyte, jclass, jfloatArray, jint, jlong, jlongArray};
use jni::JNIEnv;
use std::error::Error;
use std::mem;

/*
 * Class:     com_github_hanabi1224_RuAnnoy_NativeMethods
 * Method:    loadIndex
 * Signature: (Ljava/lang/String;IB)J
 */
// JNIEXPORT jlong JNICALL Java_com_github_hanabi1224_RuAnnoy_NativeMethods_loadIndex
//   (JNIEnv *, jclass, jstring, jint, jbyte);
ffi_fn! {
    fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_loadIndex(
        env: JNIEnv,
        class: JClass,
        path: JString,
        dimension: jint,
        index_type: jbyte,
    ) -> jlong {
        let result = Java_com_github_hanabi1224_RuAnnoy_NativeMethods_loadIndex_inner(
            env, class, path, dimension, index_type,
        );
        match result {
            Ok(pointer) => pointer,
            _ => 0,
        }
    }
}

#[allow(non_snake_case)]
fn Java_com_github_hanabi1224_RuAnnoy_NativeMethods_loadIndex_inner(
    env: JNIEnv,
    _class: JClass,
    path: JString,
    dimension: jint,
    index_type: jbyte,
) -> Result<jlong, Box<dyn Error>> {
    let ru_path: String = env.get_string(path)?.into();
    let ru_index_type: IndexType = unsafe { mem::transmute(index_type) };
    let index = AnnoyIndex::load(dimension as 