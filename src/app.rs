

// cargo run



// command shift R to refresh and clear cache
// trunk serve


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    text: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            text: String::from(

"I am looking to work in future forward programming languanges like rust.
Here are some of my projects 

- A metaprogramming tool to automatically generate c header files
Built in rust with automated testing.
https://github.com/HenryRoutson/autoheader

- This website, built in rust
https://github.com/HenryRoutson/EguiWebsite


You can read more in my resume

"
            ),
        }
    }
}

impl TemplateApp {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { text } = self;


        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Hello!\n");
            ui.label("My name is Henry Routson,\nyou can see some of my work below.");
            ui.hyperlink("https://github.com/HenryRoutson");

            ui.label("\n");

            ui.heading("Projects\n");

            ui.add(egui::Hyperlink::from_label_and_url("Multithreaded remote procedure call server in c", "https://github.com/HenryRoutson/UniversityProjects/tree/main/C/ComputerSystems/comp30023-2023-project-2-main"));

            ui.add(egui::Hyperlink::from_label_and_url("Virtual Operating system process multiplexing and memory allocation simulation in c", "https://github.com/HenryRoutson/UniversityProjects/tree/main/C/ComputerSystems/comp30023-2023-project-1-main"));

            ui.add(egui::Hyperlink::from_label_and_url("Quadtree in c for high performance data analysis", "https://github.com/HenryRoutson/UniversityProjects/tree/main/C/AlgorithmsAndDataStructures/DS_A2_quadtree"));


            ui.add(egui::Hyperlink::from_label_and_url("Flow free game solver using dijstra and A*", "https://github.com/HenryRoutson/UniversityProjects/tree/main/C/AlgorithmsAndDataStructures/DS_A3_flowFree_solver_dijstra"));

        });

        egui::SidePanel::right("side_panel").min_width(200.0).show(ctx, |ui| {

            ui.label("\n");

            ui.heading("Light / Dark");
            egui::widgets::global_dark_light_mode_switch(ui);

            ui.heading("\nContact");
            ui.label("Phone: +61 419 108 859");
            ui.add(egui::Hyperlink::from_label_and_url("Click here to Email", "mailto:Henry_Rou@protonmail.com"));
            ui.add(egui::Hyperlink::from_label_and_url("LinkedIn", "https://www.linkedin.com/in/henryroutson/"));

            ui.label("\n");

            ui.heading("Resume");
            ui.add(egui::Hyperlink::from_label_and_url("click here", "https://github.com/HenryRoutson/Resume/blob/main/HenryRoutson%20Resume.pdf"));

        });


        egui::Window::new("").show(ctx, |ui| {

            ui.add(
                egui::TextEdit::multiline(text)
                    .desired_width(f32::INFINITY)
                    .font(egui::TextStyle::Monospace)
            );

        });


        /* 
        egui::Window::new("Draw here").show(ctx, |ui| {

  

        });
        */

        
    }


}

