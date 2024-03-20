const COMMANDS: &[&str] = &["graphql", "subscriptions"];

fn main() {
  tauri_plugin::Builder::new(COMMANDS).build();
}

