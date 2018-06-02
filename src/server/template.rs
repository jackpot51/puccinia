use handlebars::Handlebars;
use std::path::Path;

pub fn create_templates() -> Handlebars {
    let mut templates = Handlebars::new();
    match Path::new("templates").read_dir() {
        Ok(tmpls) => {
            for template_res in tmpls {
                if let Ok(template) = template_res {
                    let path = template.path();
                    if let Some(name) = path.file_stem().and_then(|x| x.to_str()) {
                        eprintln!("registering handlebar template '{}' as '{}'", path.display(), name);
                        if let Err(why) = templates.register_template_file(name, &path) {
                            eprintln!("unable to register template '{}' as '{}': {}", path.display(), name, why);
                        }
                    }
                }
            }
        }
        Err(why) => {
            eprintln!("unable to open templates directory: {}", why);
        }
    }

    templates
}
