#[cfg(test)]
mod tests {
    use crate::stats::fitting::Fitting;
    use crate::stats::sensor::*;
    use once_cell::sync::Lazy;

    macro_rules! assert_sensor_eq {
        ($expected:expr, $actual:expr) => {
            approx::assert_relative_eq!(
                $expected.targeting_range,
                $actual.targeting_range,
                epsilon = 0.01
            );
            assert_eq!($expected.scan_res, $actual.scan_res);
            approx::assert_relative_eq!(
                $expected.sensor_strength,
                $actual.sensor_strength,
                epsilon = 0.01
            );
            assert_eq!(
                $expected.max_locked_targets,
                $actual.max_locked_targets
            );
        };
    }
    macro_rules! assert_fitting_eq {
        ($expected:expr, $actual:expr) => {
            approx::assert_relative_eq!(
                $expected.cpu,
                $actual.cpu,
                epsilon = 0.01
            );
            approx::assert_relative_eq!(
                $expected.pg,
                $actual.pg,
                epsilon = 0.01
            );
            assert_eq!($expected.calibration, $actual.calibration);
            approx::assert_relative_eq!(
                $expected.cargo,
                $actual.cargo,
                epsilon = 0.01
            );
        };
    }

    pub static SENSOR_STATS: Lazy<Sensor> =
        Lazy::new(|| Sensor::new(50.0, 200, 32.0, 5));
    pub static FITTING_STATS: Lazy<Fitting> =
        Lazy::new(|| Fitting::new(250.0, 250.0, 400, 375.0));
    mod stat_modifications_are_of_the_types {
        use crate::stats::fitting::*;
        use crate::stats::sensor::*;
        use crate::stats::tests::tests::{FITTING_STATS, SENSOR_STATS};
        use crate::stats::ModificationType;
        use crate::stats::Stat;

        #[test]
        fn additive() {
            let modification = SensorModifications::new(
                ModificationType::default(),
                ModificationType::Additive(50),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Sensor::new(50.0, 250, 32.0, 5);
            let actual = SENSOR_STATS.apply(vec![&modification]);
            assert_sensor_eq!(expected, actual);
        }

        #[test]
        fn multiplicative() {
            let modification = SensorModifications::new(
                ModificationType::default(),
                ModificationType::Multiplicative(1.2),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Sensor::new(50.0, 240, 32.0, 5);
            let actual = SENSOR_STATS.apply(vec![&modification]);
            assert_sensor_eq!(expected, actual);
        }

        #[test]
        fn subtractive_fitting_costs() {
            let modification = FittingModifications::new(
                ModificationType::FittingCost(50.0),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Fitting::new(200.0, 250.0, 400, 375.0);
            let actual = FITTING_STATS.apply(vec![&modification]);
            assert_fitting_eq!(expected, actual);
        }
    }
    mod stat_modifications_are_applied {
        use crate::stats::fitting::*;
        use crate::stats::sensor::*;
        use crate::stats::tests::tests::{FITTING_STATS, SENSOR_STATS};
        use crate::stats::ModificationType;
        use crate::stats::Stat;

        #[test]
        fn additive_before_multiplicative() {
            let mod_add = SensorModifications::new(
                ModificationType::Additive(50),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let mod_multi = SensorModifications::new(
                ModificationType::Multiplicative(2),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Sensor::new(200.0, 200, 32.0, 5);
            let actual = SENSOR_STATS.apply(vec![&mod_multi, &mod_add]);
            assert_sensor_eq!(expected, actual);
        }

        #[test]
        fn multiplicative_before_fitting() {
            let mod_multi = FittingModifications::new(
                ModificationType::Multiplicative(1.2),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let mod_fitting = FittingModifications::new(
                ModificationType::FittingCost(100.0),
                ModificationType::default(),
                ModificationType::default(),
                ModificationType::default(),
            );
            let expected = Fitting::new(200.0, 250.0, 400, 375.0);
            let actual = FITTING_STATS.apply(vec![&mod_fitting, &mod_multi]);
            assert_fitting_eq!(expected, actual);
        }
    }
}
