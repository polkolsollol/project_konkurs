use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::{borrow::Borrow, cell::RefCell, clone, ptr::null};
use ic_cdk_macros::{init, query, update};

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    password: String,
    gmail: String,
    role: String
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct Awaria {
    nazwa: String,
    komentarze: Vec<String>,
    owner: User,
    przyczyna: String,
    opis: String,
    miejsce: String
}

thread_local! {
    static UZYTKOWNICY: RefCell<Vec<User>> = RefCell::new(Vec::new());
    static AWARIE: RefCell<Vec<Awaria>> = RefCell::new(Vec::new());
}

const SPECJALNY_KOD_ADMINA: &str = "RF_09";

#[init]
fn init() {
    UZYTKOWNICY.with(|user_storage| {
        *user_storage.borrow_mut() = Vec::new();
    });
}

#[update]
fn dodaj_uzytkownika(username: String, password: String, mut role: String, gmail: String) -> String {
    UZYTKOWNICY.with(|user_storage| {
        let mut users = user_storage.borrow_mut();
        if users.iter().any(|user| user.username == username) {
            return "Użytkownik o tej nazwie już istnieje.".to_string();
        }

        // Rozdziel hasło na części
        let parts: Vec<&str> = password.split(":code:").collect();
        if parts.len() == 2 {
            let (password_part, special_code) = (parts[0], parts[1]);

            // Sprawdź, czy specjalny kod jest poprawny
            if special_code == SPECJALNY_KOD_ADMINA {
                role = "admin".to_string();
            }

            // Utwórz nowego użytkownika z odpowiednią rolą
            users.push(User {
                username,
                password: password_part.to_string(),
                role,
                gmail,
            });

            "Użytkownik z rolą admin dodany pomyślnie.".to_string()
        } else {
            users.push(User {
                username,
                password: password,
                role,
                gmail,
            });
            "Użytkownik dodany pomyślnie.".to_string()
        }
    })
}
#[query]
fn zaloguj(username: String, password: String) -> (bool, Option<User>) {
    UZYTKOWNICY.with(|user_storage| {
        let users = user_storage.borrow();
        for user in users.iter() {
            if user.username.to_lowercase() == username.to_lowercase() && user.password == password {
                return (true, Some(user.clone()));
            }
        }
        (false, None)
    })
}

#[update]
fn dodaj_awarie(qnazwa: String, qowner: User, qprzyczyna: String, qopis: String, qmiejsce: String) -> usize {
    let awaria = Awaria {
        nazwa: qnazwa,
        komentarze: vec![],
        owner: qowner,
        przyczyna: qprzyczyna,
        opis: qopis,
        miejsce: qmiejsce
    };

    let id = AWARIE.with(|awarie| {
        let mut awarie = awarie.borrow_mut();
        let id = awarie.len();
        awarie.push(awaria);
        id
    });

    id
}

#[query]
fn odczytaj_awarie() -> Vec<Awaria> {
    AWARIE.with(|awarie_storage| {
        let awarie = awarie_storage.borrow().clone();
        return awarie;
    })
}

#[update]
fn usun_awarie(id_awarii: usize) {
    AWARIE.with(|awarie| {
        let mut awarie_mut = awarie.borrow_mut();
        if id_awarii < awarie_mut.len() {
            awarie_mut.remove(id_awarii);
        } else {
            ic_cdk::api::trap("Index out of bounds");
        }
    });
}

#[update]
fn dodaj_komentarz(id_awarii: usize, komentarz: String) {
    AWARIE.with(|awarie| {
        let mut awarie_mut = awarie.borrow_mut();
        if let Some(awaria) = awarie_mut.get_mut(id_awarii) {
            awaria.komentarze.push(komentarz);
        } else {
            ic_cdk::api::trap("Index out of bounds");
        }
    });
}

#[update]
fn edytuj_komentarz(id_awarii: usize, id_komentarza: usize, nowy_komentarz: String) {
    AWARIE.with(|awarie| {
        let mut awarie_mut = awarie.borrow_mut();
        if let Some(awaria) = awarie_mut.get_mut(id_awarii) {
            if id_komentarza < awaria.komentarze.len() {
                awaria.komentarze[id_komentarza] = nowy_komentarz;
            } else {
                ic_cdk::api::trap("Comment index out of bounds");
            }
        } else {
            ic_cdk::api::trap("Awaria index out of bounds");
        }
    });
}

#[update]
fn usun_komentarz(id_awarii: usize, id_komentarza: usize) {
    AWARIE.with(|awarie| {
        let mut awarie_mut = awarie.borrow_mut();
        if let Some(awaria) = awarie_mut.get_mut(id_awarii) {
            if id_komentarza < awaria.komentarze.len() {
                awaria.komentarze.remove(id_komentarza);
            } else {
                ic_cdk::api::trap("Comment index out of bounds");
            }
        } else {
            ic_cdk::api::trap("Awaria index out of bounds");
        }
    });
}

#[update]
fn edytuj_awarie(id_awarii: usize, nowa_awaria: Vec<String>) {
    AWARIE.with(|awarie| {
        let mut binding = awarie.borrow_mut();
        if let Some(existing) = binding.get_mut(id_awarii) {
            existing.nazwa = nowa_awaria[0].clone();
            existing.przyczyna = nowa_awaria[1].clone();
            existing.opis = nowa_awaria[2].clone();
            existing.miejsce = nowa_awaria[3].clone();
        } else {
            ic_cdk::api::trap("Index out of bounds");
        }
    });
}

#[update]
fn nadaj_role(username: String, role: String)
{
    UZYTKOWNICY.with(|user_storage| {
        let mut users = user_storage.borrow_mut();
        for user in users.iter_mut() {
            if user.username.to_lowercase() == username.to_lowercase() {
                user.role = role;
                return true;
            }
        }
        return false;
    });
}

#[update]
fn zmien_haslo(username: String, new_password: String)
{
    UZYTKOWNICY.with(|user_storage| {
        let mut users = user_storage.borrow_mut();
        for user in users.iter_mut() {
            if user.username.to_lowercase() == username.to_lowercase() {
                user.password = new_password;
                return true;
            }
        }
        return false;
    });
}