// examples/logging.rs

#[derive(Debug)]
#[allow(unused)]
struct Data {
    x: i32,
    y: (f32, f32),
    z: String,
}

fn main() {
    let data = Data {
        x: 10,
        y: (3.15, 2.72),
        z: "Hello World!".to_string(),
    };

    tiny_logger::trace!("Trace message");
    tiny_logger::trace!("Trace with data: {:?}", data);

    tiny_logger::debug!("Debug message");
    tiny_logger::debug!("Debug with data: {:?}", data);

    tiny_logger::info!("Info message");
    tiny_logger::info!("Info with data: {:?}", data);

    tiny_logger::warn!("Warn message");
    tiny_logger::warn!("Warn with data: {:?}", data);

    tiny_logger::error!("Error message");
    tiny_logger::error!("Error with data: {:?}", data);

    tiny_logger::assert!(true, "Assert message");

    tiny_logger::die!("Die message");
}
