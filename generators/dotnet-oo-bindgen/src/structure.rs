use oo_bindgen::*;
use oo_bindgen::native_struct::*;
use oo_bindgen::formatting::*;
use heck::{CamelCase, MixedCase};
use crate::*;

pub(crate) fn generate(f: &mut impl Printer, native_struct: &StructHandle, lib: &Library) -> FormattingResult<()> {
    let struct_name = native_struct.name().to_camel_case();
    let struct_native_name = format!("{}Native", struct_name);

    print_license(f, &lib.license)?;

    f.writeln("using System;")?;
    f.writeln("using System.Runtime.InteropServices;")?;
    f.newline()?;

    namespaced(f, &lib.name, |f| {
        f.writeln(&format!("public struct {}", struct_name))?;
        blocked(f, |f| {
            // Write .NET structure elements
            for el in native_struct.elements() {
                let dotnet_type = DotnetType(&el.element_type);
                f.writeln(&format!("public {} {};", dotnet_type.as_dotnet_type(), el.name.to_camel_case()))?;
            }

            f.newline()?;

            // Write methods
            for method in &native_struct.methods {
                f.writeln(&format!("public {} {}(", DotnetReturnType(&method.native_function.return_type).as_dotnet_type(), method.name.to_camel_case()))?;
                f.write(
                    &method.native_function.parameters.iter().skip(1)
                        .map(|param| format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name.to_mixed_case()))
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    f.writeln(&format!("{}? _self = this;", struct_name))?;
                    call_native_function(f, &method.native_function, "return ", Some("_self".to_string()), false)
                })?;
            }

            f.newline()?;

            for method in &native_struct.static_methods {
                f.writeln(&format!("public static {} {}(", DotnetReturnType(&method.native_function.return_type).as_dotnet_type(), method.name.to_camel_case()))?;
                f.write(
                    &method.native_function.parameters.iter()
                        .map(|param| format!("{} {}", DotnetType(&param.param_type).as_dotnet_type(), param.name.to_mixed_case()))
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
                f.write(")")?;

                blocked(f, |f| {
                    call_native_function(f, &method.native_function, "return ", None, false)
                })?;
            }

            Ok(())
        })?;

        f.newline()?;

        // Write native struct
        f.writeln("[StructLayout(LayoutKind.Sequential)]")?;
        f.writeln(&format!("internal struct {}", struct_native_name))?;
        blocked(f, |f| {
            // Write native elements
            for el in native_struct.elements() {
                let dotnet_type = DotnetType(&el.element_type);
                f.writeln(&format!("{} {};", dotnet_type.as_native_type(), el.name.to_camel_case()))?;
            }

            f.newline()?;

            // Convert from .NET to native
            f.writeln(&format!("internal static {} ToNative({} self)", struct_native_name, struct_name))?;
            blocked(f, |f| {
                f.writeln(&format!("{} result;", struct_native_name))?;
                for el in native_struct.elements() {
                    let el_name = el.name.to_camel_case();

                    let dotnet_type = DotnetType(&el.element_type);
                    if let Some(conversion) = dotnet_type.conversion() {
                        conversion.convert_to_native(f, &format!("self.{}", el_name), &format!("result.{} = ", el_name))?;
                    } else {
                        f.writeln(&format!("result.{} = self.{};", el_name, el_name))?;
                    }
                }
                f.writeln("return result;")
            })?;

            f.newline()?;

            // Convert from native to .NET
            f.writeln(&format!("internal static {} FromNative({} native)", struct_name, struct_native_name))?;
            blocked(f, |f| {
                f.writeln(&format!("{} result;", struct_name))?;
                for el in native_struct.elements() {
                    let el_name = el.name.to_camel_case();

                    let dotnet_type = DotnetType(&el.element_type);
                    if let Some(conversion) = dotnet_type.conversion() {
                        conversion.convert_from_native(f, &format!("native.{}", el_name), &format!("result.{} = ", el_name))?;
                    } else {
                        f.writeln(&format!("result.{} = native.{};", el_name, el_name))?;
                    }
                }
                f.writeln("return result;")
            })?;

            f.newline()?;

            // Finalizer
            f.writeln("internal void Dispose()")?;
            blocked(f, |f| {
                for el in native_struct.elements() {
                    let el_name = el.name.to_camel_case();

                    let dotnet_type = DotnetType(&el.element_type);
                    if let Some(conversion) = dotnet_type.conversion() {
                        conversion.convert_to_native_cleanup(f, &format!("this.{}", el_name))?;
                    }
                }
                Ok(())
            })
        })
    })
}
