use orx_self_or::SoR;
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
    M: SoR<HashMap<String, User>>,
{
    // M can either be (i) &mut HashMap<_, _> or (ii) HashMap<_, _>
    users: M,
}

impl<M> UserService<M>
where
    M: SoR<HashMap<String, User>>,
{
    fn user(&self, user_name: &str) -> Option<&User> {
        self.users.get_ref().get(user_name)
    }
}

fn main() {
    let users: HashMap<String, User> = [
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

    // HOLDING REF: user service having a shared reference to "users"
    let service = UserService { users: &users };

    assert_eq!(
        service.user("john"),
        Some(&User::new("john".to_string(), vec!["admin".to_string()]))
    );

    // OWNING: user service owning "users"
    let service = UserService { users };
    assert_eq!(
        service.user("john"),
        Some(&User::new("john".to_string(), vec!["admin".to_string()]))
    );
}
