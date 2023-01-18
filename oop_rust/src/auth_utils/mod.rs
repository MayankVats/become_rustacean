pub fn login(creds: models::Credentials) {
  crate::database::get_user();
}

fn log_out() {

}

pub mod models;