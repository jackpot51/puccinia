use puccinia::Puccinia;
use rocket::State;
use rocket_contrib::Template;

#[get("/custom/<id>")]
pub fn custom(puccinia: State<Puccinia>, id: String) -> Result<Template, String> {
    #[derive(Serialize)]
    struct Context {
        name: String,
        amount: String,
    }

    let custom = puccinia.custom.get(&id).ok_or(format!("custom '{}' not found", id))?;

    Ok(Template::render("custom", &Context {
        name: custom.name().to_string(),
        amount: custom.amount().to_string()
    }))
}
