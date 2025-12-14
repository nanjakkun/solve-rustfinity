/*
Create an enum VehicleStatus with the following variants:

    Parked — a unit variant representing a parked vehicle.
    Driving { speed: u32 } — a named field variant representing a vehicle driving at a certain speed.
    BrokenDown(String) — a tuple variant with a String describing the reason for the breakdown.

Implement the following methods for VehicleStatus:

    is_operational(&self) -> bool:
        Returns true if the vehicle is either Parked or Driving.
        Returns false if the vehicle is BrokenDown.

    description(&self) -> String:
        Returns "The vehicle is parked." for Parked.
        Returns "The vehicle is driving at {speed} km/h." for Driving { speed }.
        Returns "The vehicle is broken down: {reason}." for BrokenDown(reason).
*/

pub enum VehicleStatus {
    // Define the VehicleStatus variants here
    Parked,
    Driving { speed: u32 },
    BrokenDown(String),
}

impl VehicleStatus {
    pub fn is_operational(&self) -> bool {
        // Your code here...
        match self {
            VehicleStatus::BrokenDown(reason) => false,
            _ => true,
        }
    }

    pub fn description(&self) -> String {
        match self {
            VehicleStatus::Parked => "The vehicle is parked.".to_string(),
            VehicleStatus::Driving { speed } => format!("The vehicle is driving at {speed} km/h."),
            VehicleStatus::BrokenDown(reason) => format!("The vehicle is broken down: {reason}."),
        }
    }
}

// Example use case
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
