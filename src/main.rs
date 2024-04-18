use basic_desktop_calculator::calculator::CalculatorApp;

fn main() {
    let mut calculator_app = CalculatorApp::new();
    
    calculator_app.init_gui();
    calculator_app.run();
}

