pub enum VehicleStatus {
    Parked,
    Driving { speed: u32 },
    BrokenDown(String)
}

impl VehicleStatus {
    pub fn is_operational(&self) -> bool {
        match self {
            VehicleStatus::BrokenDown(_) => false,
            _ => true,
        }
    }

    pub fn description(&self) -> String {
        match self {
            VehicleStatus::Parked => "The vehicle is parked.".to_string(),
            VehicleStatus::Driving{ speed } => format!("The vehicle is driving at {speed} km/h."),
            VehicleStatus::BrokenDown(reason) => format!("The vehicle is broken down: {reason}."),
        }
    }
}

pub fn main() {
    let parked = VehicleStatus::Parked;
    assert!(parked.is_operational());
    assert_eq!(parked.description(), "The vehicle is parked.");

    let driving = VehicleStatus::Driving { speed: 80 };
    assert!(driving.is_operational());
    assert_eq!(driving.description(), "The vehicle is driving at 80 km/h.");

    let broken_down = VehicleStatus::BrokenDown("Flat tire".to_string());
    assert!(!broken_down.is_operational());
    assert_eq!(
        broken_down.description(),
        "The vehicle is broken down: Flat tire."
    );
}
