use oo_bindgen::*;
use oo_bindgen::class::*;
use oo_bindgen::formatting::*;
use crate::*;

pub fn generate(f: &mut dyn Printer, class: &ClassHandle, lib: &Library) -> FormattingResult<()> {
    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        let static_specifier = if class.is_static() { "static " } else { "" };
        f.writeln(&format!("public {}class {}", static_specifier, class.name()))?;
        if class.destructor.is_some() {
            f.write(": IDisposable")?;
        }

        blocked(f, |f| {
            if !class.is_static() {
                f.writeln("private IntPtr self;")?;
                if class.destructor.is_some() {
                    f.writeln("private bool disposed = false;")?;
                }
                f.newline()?;

                f.writeln(&format!("internal {}(IntPtr self)", class.name()))?;
                blocked(f, |f| {
                    f.writeln("this.self = self;")
                })?;
                f.newline()?;
            }

            if let Some(constructor) = &class.constructor {
                generate_constructor(f, class.name(), constructor)?;
                f.newline()?;
            }

            if let Some(destructor) = &class.destructor {
                generate_destructor(f, class.name(), destructor)?;
                f.newline()?;
            }

            for method in &class.methods {
                generate_method(f, method)?;
                f.newline()?;
            }

            for method in &class.static_methods {
                generate_static_method(f, method)?;
                f.newline()?;
            }

            Ok(())
        })
    })
}

fn generate_constructor(f: &mut dyn Printer, classname: &str, constructor: &NativeFunctionHandle) -> FormattingResult<()> {
    f.writeln(&format!("public {}(", classname))?;
    f.write(
        &constructor.parameters.iter()
            .map(|param| format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name))
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &constructor, "this.self = ", false)
    })
}

fn generate_destructor(f: &mut dyn Printer, classname: &str, destructor: &NativeFunctionHandle) -> FormattingResult<()> {
    // Public Dispose method
    f.writeln("public void Dispose()")?;
    blocked(f, |f| {
        f.writeln("Dispose(true);")?;
        f.writeln("GC.SuppressFinalize(this);")
    })?;

    f.newline()?;

    // Finalizer
    f.writeln(&format!("~{}()", classname))?;
    blocked(f, |f| {
        f.writeln("Dispose(false);")
    })?;

    f.newline()?;

    // The IDisposable implementation
    f.writeln("protected virtual void Dispose(bool disposing)")?;
    blocked(f, |f| {
        f.writeln("if (this.disposed)")?;
        f.writeln("    return;")?;
        f.newline()?;
        f.writeln(&format!("{}.{}(this.self);", NATIVE_FUNCTIONS_CLASSNAME, destructor.name))?;
        f.newline()?;
        f.writeln("this.disposed = true;")
    })
}

fn generate_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    f.writeln(&format!("public {} {}(", DotnetReturnType(&method.native_function.return_type).as_dotnet_type(), method.name))?;
    f.write(
        &method.native_function.parameters.iter().skip(1)
            .map(|param| format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name))
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", true)
    })
}

fn generate_static_method(f: &mut dyn Printer, method: &Method) -> FormattingResult<()> {
    f.writeln(&format!("public static {} {}(", DotnetReturnType(&method.native_function.return_type).as_dotnet_type(), method.name))?;
    f.write(
        &method.native_function.parameters.iter()
            .map(|param| format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name))
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(")")?;

    blocked(f, |f| {
        call_native_function(f, &method.native_function, "return ", false)
    })
}

fn call_native_function(f: &mut dyn Printer, method: &NativeFunction, return_destination: &str, first_param_is_self: bool) -> FormattingResult<()> {
    // Write the type conversions
    &method.parameters.iter()
        .map(|param| {
            if let Some(converter) = DotnetType(&param.param_type).conversion() {
                return converter.convert_to_native(f, &param.name, &format!("var {}Native = ", param.name));
            }
            Ok(())
        }).collect::<FormattingResult<()>>()?;

    // Call the native function
    f.newline()?;
    if let ReturnType::Type(return_type) = &method.return_type {
        if let Some(_) = DotnetType(&return_type).conversion() {
            f.write(&format!("var _nativeResult = {}.{}(", NATIVE_FUNCTIONS_CLASSNAME, method.name))?;
        } else {
            f.write(&format!("{}{}.{}(", return_destination, NATIVE_FUNCTIONS_CLASSNAME, method.name))?;
        }
    } else {
        f.write(&format!("{}.{}(", NATIVE_FUNCTIONS_CLASSNAME, method.name))?;
    }

    f.write(
        &method.parameters.iter().enumerate()
            .map(|(idx, param)| {
                if idx == 0 && first_param_is_self {
                    "this.self".to_string()
                } else{
                    DotnetParameter(param).arg()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    )?;
    f.write(");")?;

    // Convert the result (if required)
    if let ReturnType::Type(return_type) = &method.return_type {
        if let Some(converter) = DotnetType(&return_type).conversion() {
            converter.convert_from_native(f, "_nativeResult", return_destination)?;
        }
    }

    Ok(())
}

pub struct DotnetParameter<'a>(&'a Parameter);

impl<'a> DotnetParameter<'a> {
    fn arg(&self) -> String {
        match &self.0.param_type {
            Type::Bool => self.0.name.to_string(),
            Type::Uint8 => self.0.name.to_string(),
            Type::Sint8 => self.0.name.to_string(),
            Type::Uint16 => self.0.name.to_string(),
            Type::Sint16 => self.0.name.to_string(),
            Type::Uint32 => self.0.name.to_string(),
            Type::Sint32 => self.0.name.to_string(),
            Type::Uint64 => self.0.name.to_string(),
            Type::Sint64 => self.0.name.to_string(),
            Type::Float => unimplemented!(),
            Type::Double => unimplemented!(),
            Type::String => unimplemented!(),
            Type::Struct(_) => self.0.name.to_string(),
            Type::StructRef(_) => format!("ref {}", self.0.name.to_string()),
            Type::Enum(_) => self.0.name.to_string(),
            Type::ClassRef(_) => format!("{}.self", self.0.name.to_string()),
            Type::Interface(_) => format!("{}Native", self.0.name),
            Type::Duration(mapping) => match mapping {
                DurationMapping::Milliseconds => format!("{}Native", self.0.name),
                DurationMapping::Seconds => format!("{}Native", self.0.name),
                DurationMapping::SecondsFloat => format!("{}Native", self.0.name),
            }
        }
    }
}
