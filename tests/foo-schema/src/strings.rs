use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare the class
    let stringclass = lib.declare_class("StringClass")?;

    // Declare each native function
    let stringclass_new_func = lib
        .declare_native_function("string_new")?
        .return_type(ReturnType::Type(Type::ClassRef(stringclass.clone())))?
        .build()?;

    let stringclass_destroy_func = lib
        .declare_native_function("string_destroy")?
        .param("stringclass", Type::ClassRef(stringclass.clone()))?
        .return_type(ReturnType::Void)?
        .build()?;

    let stringclass_echo_func = lib
        .declare_native_function("string_echo")?
        .param("stringclass", Type::ClassRef(stringclass.clone()))?
        .param("value", Type::String)?
        .return_type(ReturnType::Type(Type::String))?
        .build()?;

    let stringclass_length_func = lib
        .declare_native_function("string_length")?
        .param("value", Type::String)?
        .return_type(ReturnType::Type(Type::Uint32))?
        .build()?;

    // Define the class
    let _testclass = lib
        .define_class(&stringclass)?
        .constructor(&stringclass_new_func)?
        .destructor(&stringclass_destroy_func)?
        .method("Echo", &stringclass_echo_func)?
        .static_method("GetLength", &stringclass_length_func)?
        .build();

    Ok(())
}
