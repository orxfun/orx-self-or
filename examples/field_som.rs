use orx_self_or::SoM;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct User {
    user_name: String,
    roles: Vec<String>,
}

impl User {
    fn new(user_name: String, roles: Vec<String>) -> Self {
        Self { user_name, roles }
    }
}

struct UserService<M>
where
    M: SoM<HashMap<String, User>>,
{
    // M can either be (i) &mut HashMap<_, _> or (ii) HashMap<_, _>
    users: M,
}

impl<M> UserService<M>
where
    M: SoM<HashMap<String, User>>,
{
    fn user(&self, user_name: &str) -> Option<&User> {
        self.users.get_ref().get(user_name)
    }

    fn user_mut(&mut self, user_name: &str) -> Option<&mut User> {
        self.users.get_mut().get_mut(user_name)
    }
}

fn main() {
    let mut users: HashMap<String, User> = [
        (
            "john".to_string(),
            User::new("john".to_string(), vec!["admin".to_string()]),
        ),
        (
            "doe".to_string(),
            User::new("doe".to_string(), vec!["guest".to_string()]),
        ),
    ]
    .into_iter()
    .collect();

    // HOLDING MUT REF: user service having a mut reference to "users"
    let mut service = UserService { users: &mut users };

    assert_eq!(
        service.user("john"),
        Some(&User::new("john".to_string(), vec!["admin".to_string()]))
    );

    service.user_mut("doe").unwrap().roles.clear();
    assert_eq!(service.user("doe").map(|x| x.roles.len()), Some(0));
    assert_eq!(users.get("doe").map(|x| x.roles.len()), Some(0));

    // OWNING: user service owning "users"
    let mut service = UserService { users };
    assert_eq!(
        service.user("john"),
        Some(&User::new("john".to_string(), vec!["admin".to_string()]))
    );

    service
        .user_mut("doe")
        .unwrap()
        .roles
        .push("guest".to_string());
    assert_eq!(service.user("doe").map(|x| x.roles.len()), Some(1));
}
