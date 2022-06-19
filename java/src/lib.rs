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
//   (JNIEnv *, jclass, jstrin