fn main() {
    println!("Hello world");
}

// use dear_imgui::*;
// use dear_imgui_winit::{Context, GlRenderer, WindowBuilder};
// use winit::{self, Event, Key, KeyState, Keycode, Window, WindowOptions};

// fn main() {
//     // Create a window
//     let mut window = WindowBuilder::new("Rust Crash Reporter", 600, 400)
//         .build()
//         .expect("Unable to open window");

//     // Create an ImGui context
//     let mut ctx = Context::new();

//     // Create a GlRenderer for rendering ImGui to the window
//     let mut renderer = GlRenderer::new(&window, &ctx).expect("Unable to create renderer");

//     // Main loop
//     while let Some(event) = window.next_event() {
//         match event {
//             Event::WindowEvent::Closed => {
//                 break;
//             }
//             Event::KeyEvent { key, state, .. } => {
//                 // Handle key presses (e.g., Escape to quit)
//                 if key == Keycode::Escape && state == KeyState::Press {
//                     break;
//                 }
//             }
//             _ => {}
//         }

//         // Start a new ImGui frame
//         ctx.new_frame(&window);

//         // Show the crash report window
//         show_crash_report_window(&mut ctx, "Example crash message");

//         // Render ImGui draw data to the window
//         renderer.render(&ctx);
//     }
// }

// fn show_crash_report_window(ctx: &mut Context, error_message: &str) {
//     // Window flags
//     const CHECKBOX_FLAGS: i32 = ImGuiWindowFlags::AlwaysAutoResize;

//     if imgui::begin("Rust Crash Reporter", null_ptr::<usize>(), CHECKBOX_FLAGS) {
//         // Title with "We're Sorry" in bold
//         imgui::text_colored("We're Sorry", ImVec4::from_rgb(1.0, 0.0, 0.0));

//         // Error message
//         imgui::text(error_message);

//         // Checkbox and label
//         let mut show_details = imgui::checkbox("Tell us about the crash so we can fix it", true);

//         // Details button
//         if imgui::button("Details...") {
//             println!("Add details");
//         }

//         // Text box for additional details
//         let mut details = String::new();
//         imgui::input_text_multiline("##Details", &mut details, 500, Vec2::new(0.0, 100.0));

//         // Message about submitting the crash report
//         imgui::text("Your crash report will be submitted before you quit or restart.");

//         // Quit and Restart buttons
//         if imgui::button("Quit") {
//             winit::Window::get_primary().expect("No primary window found").close();
//         }
//         if imgui::button("Restart") {
//             println!("Restarting...");
//             // Add code here to restart the application
//         }
//     }

//     imgui::end();
// }
