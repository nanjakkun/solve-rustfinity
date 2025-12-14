/*
Create an enum TrafficLight with three variants:

    Red representing the red light.
    Yellow representing the yellow light.
    Green representing the green light.

Write a function light_action that takes a TrafficLight and returns a string describing the action associated with the light.

    For Red, return "Stop".
    For Yellow, return "Caution".
    For Green, return "Go".
*/
pub enum TrafficLight {
    // Define enum variants here
    Red,
    Yellow,
    Green,
}

pub fn light_action(light: &TrafficLight) -> &'static str {
    // Your code here...
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Caution",
        TrafficLight::Green => "Go",
    }
}
