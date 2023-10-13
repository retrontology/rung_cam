pub struct RungBuffer {
    producer: i32,
    consumer: i32,
}

impl RungBuffer {
    pub fn new() -> RungBuffer {
        RungBuffer{
            producer: 0,
            consumer: 0,
        }
    }
}