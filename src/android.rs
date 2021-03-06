use jni::JNIEnv;
use jni::objects::{JClass};
use jni::sys::{jstring, jlong, jbyteArray, jint};

use crate::converter::Converter;
use crate::dimensions::Dimensions;
use crate::buffer_2d::Buffer2d;

mod android_lib;

#[no_mangle]
pub unsafe extern
fn Java_com_demont93_camera_1x_1app_RustBindings_newConverter(
    _: JNIEnv,
    _: JClass,
    output_height: jint,
    output_width: jint,
    input_height: jint,
    input_width: jint,
    cpixel_height: jint,
    cpixel_width: jint,
) -> jlong
{
    Box::into_raw(Box::new(Converter::new(
        &Dimensions {
            height: output_height as usize,
            width: output_width as usize,
        },
        &Dimensions {
            height: input_height as usize,
            width: input_width as usize,
        },
        true,
    ))) as i64
}

#[no_mangle]
pub unsafe extern
fn Java_com_demont93_camera_1x_1app_RustBindings_convert(
    env: JNIEnv,
    _: JClass,
    converter_i64: jlong,
    buffer: jbyteArray,
) -> jstring
{
    let converter = converter_i64 as *mut Converter;
    let dims = (*converter).image_settings();
    let buffer = env.convert_byte_array(buffer).unwrap();
    let buffer = Buffer2d::new(
        *dims,
        buffer,
    );
    let result = (*converter).convert_one(&buffer);

    let final_string = result.buffer
        .chunks(result.dimensions.width)
        .map(|chunk| {
            chunk.iter().map(|m| m.0).collect::<String>()
        })
        .collect::<Vec<String>>()
        .as_slice()
        .join("\n");
    env.new_string(final_string).unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern
fn Java_com_demont93_camera_1x_1app_RustBindings_dropConverter(
    _: JNIEnv,
    _: JClass,
    converter_i64: jlong,
) {
    let converter = Box::from_raw(converter_i64 as *mut Converter);
}
