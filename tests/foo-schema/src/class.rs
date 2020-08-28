use oo_bindgen::native_function::*;
use oo_bindgen::*;

pub fn define(lib: &mut LibraryBuilder) -> Result<(), BindingError> {
    // Declare the class
    let testclass = lib.declare_class("TestClass")?;

    // Declare each native function
    let testclass_new_func = lib
        .declare_native_function("testclass_new")?
        .param("value", Type::Uint32, "Value")?
        .return_type(ReturnType::new(
            Type::ClassRef(testclass.clone()),
            "New TestClass",
        ))?
        .doc("Create a new TestClass")?
        .build()?;

    let testclass_destroy_func = lib
        .declare_native_function("testclass_destroy")?
        .param(
            "testclass",
            Type::ClassRef(testclass.clone()),
            "Class handle",
        )?
        .return_type(ReturnType::void())?
        .doc("Destroy a test class")?
        .build()?;

    let testclass_get_value_func = lib
        .declare_native_function("testclass_get_value")?
        .param(
            "testclass",
            Type::ClassRef(testclass.clone()),
            "TestClass handle",
        )?
        .return_type(ReturnType::new(Type::Uint32, "Current value"))?
        .doc("Get value")?
        .build()?;

    let testclass_increment_value_func = lib
        .declare_native_function("testclass_increment_value")?
        .param(
            "testclass",
            Type::ClassRef(testclass.clone()),
            "TestClass handle",
        )?
        .return_type(ReturnType::void())?
        .doc("Increment value")?
        .build()?;

    let get_value_cb = lib
        .define_one_time_callback("GetValueCallback", "GetValue callback handler")?
        .callback("on_value", "On value callback")?
        .param("value", Type::Uint32, "Value")?
        .ctx("data")?
        .return_type(ReturnType::void())?
        .build()?
        .ctx("data")?
        .build()?;

    let testclass_get_value_async_func = lib
        .declare_native_function("testclass_get_value_async")?
        .param(
            "testclass",
            Type::ClassRef(testclass.clone()),
            "TestClass handle",
        )?
        .param(
            "cb",
            Type::OneTimeCallback(get_value_cb),
            "Callback to call with the value",
        )?
        .return_type(ReturnType::void())?
        .doc("Get value through a callback")?
        .build()?;

    let testclass_construction_counter = lib
        .declare_native_function("testclass_construction_counter")?
        .return_type(ReturnType::new(
            Type::Uint32,
            "Number of calls to the constructor",
        ))?
        .doc("Get number of calls to the constructor")?
        .build()?;

    // Define the class
    let _testclass = lib
        .define_class(&testclass)?
        .constructor(&testclass_new_func)?
        .destructor(&testclass_destroy_func)?
        .method("GetValue", &testclass_get_value_func)?
        .method("IncrementValue", &testclass_increment_value_func)?
        .async_method("GetValueAsync", &testclass_get_value_async_func)?
        .static_method("ConstructionCounter", &testclass_construction_counter)?
        .doc("TestClass")?
        .build()?;

    Ok(())
}
