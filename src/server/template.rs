use handlebars::Handlebars;
use std::path::Path;

pub fn create_templates() -> Handlebars {
    let tmpl_path = Path::new("templates");
    let mut templates = Handlebars::new();
    match tmpl_path.read_dir() {
        Ok(tmpls) => {
            for template in tmpls {
                let path = template.ok().map(|x| x.path());
                let name = path.as_ref().and_then(|x| x.file_stem()).and_then(|x| x.to_str());
                if let Some(name) = name {
                    eprintln!("registering handlebar template '{}'", name);
                    if let Err(why) = templates.register_template_file(name, tmpl_path) {
                        eprintln!("unable to register template '{}': {}", name, why);
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
