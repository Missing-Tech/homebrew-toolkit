use clap::{Arg, Command};
use std::fs;
use std::path::Path;

fn main() {
    let matches = Command::new("toolkit")
        .version("0.1.2")
        .author("Joe Grabski <joe.grabski@clearwatt.co.uk>")
        .about("Generates C# .NET service boilerplate")
        .arg(
            Arg::new("service")
                .help("The name of the service to generate")
                .required(true),
        )
        .arg(
            Arg::new("directory")
                .help("The target directory where the service will be created")
                .required(true),
        )
        .arg(
            Arg::new("include-validator")
                .short('v')
                .long("include-validator")
                .help("Include a validator for the request object")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Get the service name and target directory from the command line arguments
    let service_name: &str = matches.get_one::<String>("service").unwrap();
    let directory: &str = matches.get_one::<String>("directory").unwrap();
    let include_validator = matches.get_flag("include-validator");

    // Create the full folder path for the service
    let folder_name = format!("{}/{}", directory, service_name);
    if !Path::new(&folder_name).exists() {
        fs::create_dir_all(&folder_name).expect("Failed to create directory");
    }

    // Construct the namespace based on the directory (replace '/' or '\' with '.')
    let namespace = directory.replace("/", ".").replace("\\", ".");

    // Create the files with the dynamic namespace
    generate_interface(service_name, &folder_name, &namespace);
    generate_request(service_name, &folder_name, &namespace);
    generate_response(service_name, &folder_name, &namespace);
    generate_implementation(service_name, &folder_name, &namespace);

    // Optionally generate the validator
    if include_validator {
        generate_validator(service_name, &folder_name, &namespace);
    }

    println!(
        "Boilerplate for {} created successfully in {}!",
        service_name, folder_name
    );
}

fn generate_interface(service_name: &str, folder_name: &str, namespace: &str) {
    let content = format!(
        "namespace {1}.{0};

/// <summary>
/// Service to 
/// </summary>
public interface I{0}: ICommandHandler<{0}Request> {{

}}",
        service_name, namespace
    );
    write_file(folder_name, &format!("I{}.cs", service_name), &content);
}

fn generate_request(service_name: &str, folder_name: &str, namespace: &str) {
    let content = format!(
        "namespace {1}.{0};
public class {0}Request {{
    // Define request properties here
}}
",
        service_name, namespace
    );
    write_file(
        folder_name,
        &format!("{}Request.cs", service_name),
        &content,
    );
}

fn generate_response(service_name: &str, folder_name: &str, namespace: &str) {
    let content = format!(
        "namespace {1}.{0};
public class {0}Response {{
    // Define response properties here
}}",
        service_name, namespace
    );
    write_file(
        folder_name,
        &format!("{}Response.cs", service_name),
        &content,
    );
}

fn generate_implementation(service_name: &str, folder_name: &str, namespace: &str) {
    let content = format!(
        "using Audacia.Commands;

namespace {1}.{0};

/// <inheritdoc/>
public class {0} : I{0} {{
    public {0}Service() {

    }

    public async Task<CommandResult<{0}Response>> HandleAsync({0}Request request) {{
        // Implement the service logic here
        return new {0}Response();
    }}
}}",
        service_name, namespace
    );
    write_file(folder_name, &format!("{}.cs", service_name), &content);
}

fn generate_validator(service_name: &str, folder_name: &str, namespace: &str) {
    let content = format!(
        "using FluentValidation;

namespace {1}.{0};
public class {0}Validator : AbstractValidator<{0}Request> {{

    public {0}Validator()
    {{
        // Implement validation logic here
    }}
}}",
        service_name, namespace
    );
    write_file(
        folder_name,
        &format!("{}Validator.cs", service_name),
        &content,
    );
}

fn write_file(folder_name: &str, file_name: &str, content: &str) {
    let file_path = format!("{}/{}", folder_name, file_name);
    fs::write(file_path, content).expect("Failed to write file");
}
