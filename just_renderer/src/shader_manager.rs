use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    path::PathBuf,
};

pub struct ShaderManager {
    root_path: PathBuf,
    cache: HashMap<String, String>,
}

impl ShaderManager {
    pub fn new(root_path: String) -> (Self, Result<(), String>) {
        let mut manager = Self {
            root_path: root_path.into(),
            cache: HashMap::new(),
        };
        let initialization_result = manager.process_shaders();
        (manager, initialization_result)
    }

    fn resolve_includes(
        &self,
        shader: &str,
        resolved: &mut HashSet<String>,
        output: &mut String,
    ) -> Result<(), String> {
        let mut lines = shader.lines();
        let first_line = lines.next().unwrap();
        if (first_line).starts_with("//#include") {
            for include in first_line.split_whitespace().skip(1) {
                let include_path = self.root_path.join(include).with_extension("util.wgsl");
                match File::open(&include_path) {
                    Result::Ok(mut file) => {
                        resolved.insert(include.to_owned());
                        let mut buf = String::new();
                        let _ = file.read_to_string(&mut buf).map_err(|err| {
                            format!(
                                "Couldn't read include file: {:?} with error: {:?}",
                                include_path, err
                            )
                        })?;

                        self.resolve_includes(&buf, resolved, output)?;
                        output.push_str(&buf);
                    }
                    Result::Err(err) => {
                        return Result::Err(format!(
                            "Couldn't find shader include: {} with error: {}",
                            include, err
                        ));
                    }
                }
            }
        }

        Result::Ok(())
    }

    pub fn get_shader(&self, id: &str) -> Result<String, String> {
        let shader_path = self.root_path.join(id);
        log::info!("Loading shader: {:?}", shader_path);
        let shader_source = std::fs::read_to_string(shader_path).unwrap();

        let mut output_buffer = String::with_capacity(shader_source.capacity());
        let mut resolved_includes = HashSet::<String>::new();

        self.resolve_includes(&shader_source, &mut resolved_includes, &mut output_buffer)?;
        output_buffer.push_str(&shader_source);

        Result::Ok(output_buffer)
    }

    fn process_shaders(&mut self) -> Result<(), String> {
        Result::Ok(())
    }
}
