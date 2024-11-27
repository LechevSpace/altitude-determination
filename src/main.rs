use core::f64;

/// Atmospheric zones based on NASA's 1960s model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtmosphereZone {
    Troposphere,
    LowerStratosphere,
    UpperStratosphere,
}

/// Altitude calculator using NASA's atmospheric model.
pub struct AltitudeCalculator;

impl AltitudeCalculator {
    /// Determine atmosphere zone based on altitude.
    /// 
    /// # Parameters
    /// - 'altitude_m': Altitude in meters.
    /// 
    /// # Returns
    /// Atmospheric zone.
    pub fn determine_zone(altitude_m: f64) -> AtmosphereZone {
        match altitude_m {
            0.0..=11000.0 => AtmosphereZone::Troposphere,
            11000.0..=25000.0 => AtmosphereZone::LowerStratosphere,
            _ => AtmosphereZone::UpperStratosphere,
        }
    }

    /// Calculate altitude based on atmospheric zone, temperature, and pressure.
    /// 
    /// # Parameters
    /// - 'temperature_c': Temperature in degrees Celsius.
    /// - 'pressure_hpa': Pressure in hectopascals (hPa).
    /// - 'altitude_m': Detected height in meters.
    /// 
    /// # Returns
    /// Altitude in meters.
    pub fn calculate_altitude(temperature_c: f64, pressure_hpa: f64, altitude_m: f64) -> f64 {
        let pressure_kpa = pressure_hpa / 10.0; // Convert hPa to kPa
        let zone = Self::determine_zone(altitude_m);

        match zone {
            AtmosphereZone::Troposphere => {
                // Troposphere: h < 11 000 m
                let t = 15.04 - 0.00649 * altitude_m; // Temperature in Celsius
                let p = 101.29 * ((t + 273.1) / 288.08).powf(5.256); // Pressure in kPa
                if (pressure_kpa - p).abs() < 0.01 {
                    altitude_m
                } else {
                    panic!("Mismatch in pressure for Troposphere!");
                }
            }
            AtmosphereZone::LowerStratosphere => {
                // Lower Stratosphere: 11 000 m < h <= 25 000 m
                let t = -56.46; // Constant temperature in Celsius (maybe because of the Tropopause)
                let p = 22.65 * (-0.000157 * (altitude_m - 11000.0)).exp(); // Pressure in kPa
                if (pressure_kpa - p).abs() < 0.01 {
                    altitude_m
                } else {
                    panic!("Mismatch in pressure for Lower Stratosphere!");
                }
            }
            AtmosphereZone::UpperStratosphere => {
                // Upper Stratosphere: h > 25 000 m
                let t = -131.21 + 0.00299 * (altitude_m - 25000.0); // Temperature in Celsius
                let p = 2.488 * ((t + 273.1) / 216.6).powf(-11.388); // Pressure in kPa
                if (pressure_kpa - p).abs() < 0.01 {
                    altitude_m
                } else {
                    panic!("Mismatch in pressure for Upper Stratosphere!");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_zone() {
        assert_eq!(AltitudeCalculator::determine_zone(5000.0), AtmosphereZone::Troposphere);
        assert_eq!(
            AltitudeCalculator::determine_zone(15000.0),
            AtmosphereZone::LowerStratosphere
        );
        assert_eq!(
            AltitudeCalculator::determine_zone(30000.0),
            AtmosphereZone::UpperStratosphere
        );
    }

    #[test]
    fn test_calculate_altitude() {
        // Example: Troposhpere
        let altitude_m = 5000.0;
        let pressure_hpa = 540.48; // Pressure at 5000 m from the model
        let temperature_c = 15.04 - 0.00649 * altitude_m;
        let calculated_altitude = AltitudeCalculator::calculate_altitude(temperature_c, pressure_hpa, altitude_m);
        
        let expected_altitude = altitude_m;
        assert!(
            (calculated_altitude - expected_altitude).abs() < 1.0, // Allow small numerical error
            "Calculated: {}, Expected: {}",
            calculated_altitude,
            expected_altitude
        );

        // Example: Lower Stratosphere
        let altitude_m = 15000.0;
        let pressure_hpa = 120.9; // Pressure at 15 000 m
        let temperature_c = -56.46; // Constant for the zone
        let calculated_altitude = AltitudeCalculator::calculate_altitude(temperature_c, pressure_hpa, altitude_m);
        
        let expected_altitude = altitude_m;
        assert!(
            (calculated_altitude - expected_altitude).abs() < 1.0,
            "Calculated: {}, Expected: {}",
            calculated_altitude,
            expected_altitude
        );

        // Example: Upper Stratosphere
        let altitude_m = 30000.0;
        let pressure_hpa = 24.9; // Approximate pressure at 30 000 m
        let temperature_c = -131.21 + 0.00299 * (altitude_m - 25000.0);
        let calculated_altitude = AltitudeCalculator::calculate_altitude(temperature_c, pressure_hpa, altitude_m);
        
        let expected_altitude = altitude_m;
        assert!(
            (calculated_altitude - expected_altitude).abs() < 1.0,
            "Calculated: {}, Expected: {}",
            calculated_altitude,
            expected_altitude
        );
    }

}

fn main() {
    let temperature_c = 15.04; // Celsius
    let pressure_hpa = 1013.25; // hPa (sea level)
    let altitude_m = 0.0; // Initial altitude in meters

    // Calculate the altitude
    let calculated_altitude = AltitudeCalculator::calculate_altitude(temperature_c, pressure_hpa, altitude_m);
    println!("Calculated Altitude: {:.2} meters", calculated_altitude);

     // Test another example at 5000 meters (Troposphere)
     let temperature_c_5000 = -17.41; // Approx temperature at 5000m
     let pressure_hpa_5000 = 540.48; // Approx pressure at 5000m
     let altitude_5000 = 5000.0; // Expected altitude

     let calculated_altitude_5000 =
        AltitudeCalculator::calculate_altitude(temperature_c_5000, pressure_hpa_5000, altitude_5000);

    println!(
        "Calculated Altitude at 5000m: {:.2} meters",
        calculated_altitude_5000
    );

    // Example in Lower Stratosphere (15000m)
    let temperature_c_15000 = -56.46; // Approx constant temperature in Lower Stratosphere
    let pressure_hpa_15000 = 120.9; // Approx pressure at 15000m
    let altitude_15000 = 15000.0; // Expected altitude

    let calculated_altitude_15000 =
        AltitudeCalculator::calculate_altitude(temperature_c_15000, pressure_hpa_15000, altitude_15000);

    println!(
        "Calculated Altitude at 15000m: {:.2} meters",
        calculated_altitude_15000
    );

    // Example in Upper Stratosphere (30000m)
    let temperature_c_30000 = -116.26; // Approx temperature at 30000m
    let pressure_hpa_30000 = 24.9; // Approx pressure at 30000m
    let altitude_30000 = 30000.0; // Expected altitude

    let calculated_altitude_30000 =
        AltitudeCalculator::calculate_altitude(temperature_c_30000, pressure_hpa_30000, altitude_30000);

    println!(
        "Calculated Altitude at 30000m: {:.2} meters",
        calculated_altitude_30000
    );
}