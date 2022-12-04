mod application;

use application::Application;



fn main() {
	let mut app = Application::new();

	app.render_loop();
}

