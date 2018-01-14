use puccinia::Puccinia;
use rocket::State;
use rocket_contrib::Template;

#[get("/crypto/<id>")]
pub fn crypto(puccinia: State<Puccinia>, id: String) -> Result<Template, String> {
    #[derive(Serialize)]
    struct Context {
        name: String,
        address: String,
        units: String,
        unit_price: String,
        market_value: String,
    }

    let crypto = puccinia.crypto.get(&id).ok_or(format!("crypto '{}' not found", id))?;

    let units = crypto.amount()?;
    let unit_price = crypto.rate()?;

    Ok(Template::render("crypto", &Context {
        name: crypto.name().to_string(),
        address: crypto.address().to_string(),
        units: format!("{}", units),
        unit_price: format!("{}", unit_price),
        market_value: format!("{}", units * unit_price),
    }))
}
