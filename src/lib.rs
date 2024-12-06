#![no_std]

use core::f64;

/// Atmospheric zones based on NASA's 1960s model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtmosphereZone {
    Troposphere,
    LowerStratosphere,
    UpperStratosphere,
}

    /// Determine atmosphere zone based on altitude.
    /// 
    /// # Parameters
    /// - 'altitude_m': Altitude in meters.
    /// 
    /// # Returns
    /// Atmospheric zone.
    pub fn determine_zone(altitude_m: f64) -> AtmosphereZone {
        if altitude_m <= 11000.0 {
            AtmosphereZone::Troposphere
        } else if altitude_m <= 20000.0 {
            AtmosphereZone::LowerStratosphere
        } else {
            AtmosphereZone::UpperStratosphere
        }
    }

    pub fn calculate_altitude(zone: AtmosphereZone, temperature_c: f64, pressure_kpa: f64) -> Option<f64> {
        match zone {
            AtmosphereZone::Troposphere => {
                let t = 15.04f64; // Sea level standard temperature in Celsius
                let p = 101.29f64 * ((t + 273.1f64) / 288.08f64).powf(5.256f64); // Pressure at sea level in kPa
    
                if pressure_kpa > p || pressure_kpa <= 22.65f64 {
                    return None; // Pressure is out of range for Troposphere
                }
    
                // Calculate altitude
                let altitude = ((288.08f64 / (temperature_c + 273.1f64)).powf(1.0f64 / 5.256f64) - 1.0f64) * 288.08f64 / 0.00649f64;
                Some(altitude)
            }
            AtmosphereZone::LowerStratosphere => {
                let t = -56.56f64; // Constant temperature in Celsius
                let p = 22.65f64 * (-0.000157f64 * 11_000.0f64).exp(); // Pressure at 11 000 m in kPa
    
                if pressure_kpa > p || pressure_kpa <= 2.488f64 {
                    return None; // Pressure is out of range for lower Stratosphere
                }
    
                let altitude = 11_000.0f64 + (pressure_kpa / 22.65f64).ln() / -0.000157f64;
                Some(altitude)
            }
            AtmosphereZone::UpperStratosphere => {
                let t = -131.21f64 + 0.00299f64 * (25_000.0f64 - 25_000.0f64); // Constant temperature in Celsius
                let p = 2.488f64 * ((t + 273.1f64) / 216.6f64).powf(-11.388f64); // Pressure at 25 000 m in kPa
    
                if pressure_kpa > p {
                    return None; // Pressure is out of range for Upper Stratosphere
                }
    
                let altitude = 25_000.0f64 + (pressure_kpa / 2.488f64).powf(-1.0f64 / 11.388f64) * (216.6f64 / 273.15f64);
                Some(altitude)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_determine_zone() {
            // Test determining the atmospheric zone based on altitude
            assert_eq!(determine_zone(5000.0), AtmosphereZone::Troposphere); // Altitude within Troposphere
            assert_eq!(determine_zone(15000.0), AtmosphereZone::LowerStratosphere); // Altitude within Lower Stratosphere
            assert_eq!(determine_zone(30000.0), AtmosphereZone::UpperStratosphere); // Altitude within Upper Stratosphere
        }
    
        #[test]
        fn test_calculate_altitude_troposphere() {
            // Test altitude calculation within the Troposphere
            let zone = AtmosphereZone::Troposphere;
            let temperature_c = 10.0; // Example temperature in Celsius
            let pressure_kpa = 90.0; // Example pressure in kPa
    
            let altitude = calculate_altitude(zone, temperature_c, pressure_kpa);
    
            // Check if the function returns some altitude value
            assert!(altitude.is_some());
    
            // Verify the calculated altitude is close to an expected range (within ±500 m of 2000 m)
            assert!((altitude.unwrap() - 2000.0).abs() < 500.0);
        }
    
        #[test]
        fn test_calculate_altitude_lower_stratosphere() {
            // Test altitude calculation within the Lower Stratosphere
            let zone = AtmosphereZone::LowerStratosphere;
            let temperature_c = -56.5; // Constant temperature in Celsius for this zone
            let pressure_kpa = 20.0; // Example pressure in kPa
    
            let altitude = calculate_altitude(zone, temperature_c, pressure_kpa);
    
            // Check if the function returns some altitude value
            assert!(altitude.is_some());
    
            // Verify the calculated altitude is close to an expected range (within ±500 m of 12000 m)
            assert!((altitude.unwrap() - 12000.0).abs() < 500.0);
        }
    
        #[test]
        fn test_calculate_altitude_upper_stratosphere() {
            // Test altitude calculation within the Upper Stratosphere
            let zone = AtmosphereZone::UpperStratosphere;
            let temperature_c = -55.0; // Example temperature in Celsius
            let pressure_kpa = 1.0; // Example pressure in kPa
    
            let altitude = calculate_altitude(zone, temperature_c, pressure_kpa);
    
            // Check if the function returns some altitude value
            assert!(altitude.is_some());
    
            // Verify the calculated altitude is close to an expected range (within ±2000 m of 26000 m)
            assert!((altitude.unwrap() - 26000.0).abs() < 2000.0);
        }
    
        #[test]
        fn test_invalid_pressure() {
            // Test invalid pressure for the Troposphere
            let zone = AtmosphereZone::Troposphere;
            let temperature_c = 10.0; // Example temperature in Celsius
            let pressure_kpa = 200.0; // Pressure too high for the Troposphere
    
            let altitude = calculate_altitude(zone, temperature_c, pressure_kpa);
    
            // Ensure the function returns None for invalid input
            assert!(altitude.is_none());
        }
    
        #[test]
        fn test_invalid_zone_pressure() {
            // Test invalid pressure for the Lower Stratosphere
            let zone = AtmosphereZone::LowerStratosphere;
            let temperature_c = -56.5; // Constant temperature in Celsius for this zone
            let pressure_kpa = 100.0; // Pressure too high for the Lower Stratosphere
    
            let altitude = calculate_altitude(zone, temperature_c, pressure_kpa);
    
            // Ensure the function returns None for invalid input
            assert!(altitude.is_none());
        }
    }    

fn main() {
    let zone = AtmosphereZone::Troposphere; // Determined in advance
    let temperature_c = 10.0; // Temperature in Celsius
    let pressure_kpa = 90.0; // Pressure in kPa

    match calculate_altitude(zone, temperature_c, pressure_kpa) {
        Some(altitude) => println!("Altitude: {:.2} m", altitude),
        None => println!("Invalid input for the given zone."),
    }
}