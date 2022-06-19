#[macro_use]
mod macros;

use annoy_rs::*;
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jbyte, jclass, jfloatArray, jint, jlong, jlongArray};
us