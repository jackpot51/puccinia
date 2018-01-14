use puccinia::Puccinia;
use rocket::State;
use rocket_contrib::Template;

#[get("/")]
pub fn index(puccinia: State<Puccinia>) -> Template {
    #[derive(Serialize)]
    struct Account {
        id: String,
        name: String,
        kind: String,
    }

    #[derive(Serialize)]
    struct Context {
        bank: Vec<Account>,
        crypto: Vec<Account>,
        custom: Vec<Account>,
    }

    let mut context = Context {
        bank: Vec::new(),
        crypto: Vec::new(),
        custom: Vec::new()
    };

    {
        for (id, bank) in &puccinia.bank {
            context.bank.push(Account {
                id: id.to_string(),
                name: bank.name().to_string(),
                kind: bank.kind().to_string()
            });
        }
    }

    {
        for (id, crypto) in &puccinia.crypto {
            context.crypto.push(Account {
                id: id.to_string(),
                name: crypto.name().to_string(),
                kind: crypto.kind().to_string()
            });
        }
    }

    {
        for (id, custom) in &puccinia.custom {
            context.custom.push(Account {
                id: id.to_string(),
                name: custom.name().to_string(),
                kind: "custom".to_string()
            });
        }
    }

    Template::render("index", &context)
}
