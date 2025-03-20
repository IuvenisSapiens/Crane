use crane_core::models::qwen25::Model;
use crane_core::models::qwen25::TextGeneration;

use crane_core::models::DType;
use crane_core::models::Device;

fn main() {
    println!("Hello, world!");

    let dtype = DType::F32;
    let device = Device::Cpu;

    let model = Model::new("checkpoints/Qwen2.5-0.5B-Instruct", &device, &dtype).unwrap();
    let tokenizer = model.tokenizer().clone();

    let mut pipe = TextGeneration::new(
        model,
        tokenizer,
        1024,
        Some(0.67),
        Some(1.0),
        1.1,
        1,
        &device,
    );

    pipe.run("who are you?", 235).unwrap();
}
