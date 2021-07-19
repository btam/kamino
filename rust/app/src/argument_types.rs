pub mod cluster_update {
    use std::str::FromStr;
    use failure::Fail;
    use regex::Regex;

    #[derive(Debug, Fail)]
    pub enum ArgumentValidationError {
        #[fail(display = "Log prefix must not have a ':' or space in it")]
        InvalidLogPrefix,
        #[fail(display = "Invalid number")]
        InvalidNumericInput,
        #[fail(display = "Input was less than the minimum value")]
        LessThanMinimum,
        #[fail(display = "Node name must be a vmss node:  eg k8s-pool-131414-vmss0003ax")]
        InvalidNodeName,
        #[fail(display = "VMSS name must be something like:  eg k8s-pool-131414-vmss")]
        InvalidVmssName,
        #[fail(display = "Subscription IDs must be in the form of a GUID '12345678-1234-1234-1234-1234567890ab'")]
        InvalidSubscriptionId,
        #[fail(display = "Unknown error")]
        UnknownError,
    }

    #[derive(Debug)]
    pub struct LogPrefix(String);

    impl FromStr for LogPrefix {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            if input.contains(":") || input.contains(" ") {
                return Err(ArgumentValidationError::InvalidLogPrefix);
            }
            Ok( Self(String::from_str(input).unwrap()) )
        }
    }
    impl LogPrefix {
        pub fn to_string(self) -> String {
            self.0
        }
    }

    #[derive(Debug)]
    pub struct NewUpdatedNodes(u32);

    impl FromStr for NewUpdatedNodes {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            let min_new_updated_nodes: u32 = 0;

            match value_minimum_check(input, min_new_updated_nodes) {
                Err(ArgumentValidationError::InvalidNumericInput) => return Err(ArgumentValidationError::InvalidNumericInput),
                Err(ArgumentValidationError::LessThanMinimum) => {
                    eprint!("Minimum for new updated nodes is {0} - ", min_new_updated_nodes);
                    return Err(ArgumentValidationError::LessThanMinimum)
                },
                Ok(_) => Ok( Self(u32::from_str(input).unwrap()) ),
                _ => return Err(ArgumentValidationError::UnknownError)
            }
        }
    }
    impl NewUpdatedNodes {
        fn as_u32(self) -> u32 {
            self.0
        }
    }

    #[derive(Debug)]
    pub struct HistorySize(u32);

    impl FromStr for HistorySize {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            let min_history_size: u32 = 2;

            match value_minimum_check(input, min_history_size) {
                Err(ArgumentValidationError::InvalidNumericInput) => return Err(ArgumentValidationError::InvalidNumericInput),
                Err(ArgumentValidationError::LessThanMinimum) => {
                    eprint!("Minimum history size is {0} - ", min_history_size);
                    return Err(ArgumentValidationError::LessThanMinimum)
                },
                Ok(_) => Ok( Self(u32::from_str(input).unwrap()) ),
                _ => return Err(ArgumentValidationError::UnknownError)
            }
        }
    }
    impl HistorySize {
        fn as_u32(self) -> u32 {
            self.0
        }
    }

    #[derive(Debug)]
    pub struct NodeName(String);

    impl FromStr for NodeName {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            match is_vmss_node(input) {
                Err(ArgumentValidationError::InvalidNodeName) => return Err(ArgumentValidationError::InvalidNodeName),
                Ok(_) => Ok( Self(String::from_str(input).unwrap()) ),
                _ => return Err(ArgumentValidationError::UnknownError)
            }
        }
    }

    #[derive(Debug)]
    pub struct VmssName(String);

    impl FromStr for VmssName {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            match is_vmss_name(input) {
                Err(ArgumentValidationError::InvalidVmssName) => return Err(ArgumentValidationError::InvalidVmssName),
                Ok(_) => Ok( Self(String::from_str(input).unwrap()) ),
                _ => return Err(ArgumentValidationError::UnknownError)
            }
        }
    }
    impl VmssName {
        pub fn to_string(self) -> String {
            self.0
        }
    }

    #[derive(Debug)]
    pub struct SubscriptionId(String);

    impl FromStr for SubscriptionId {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            match is_subscription_id(input) {
                Err(ArgumentValidationError::InvalidSubscriptionId) => return Err(ArgumentValidationError::InvalidSubscriptionId),
                Ok(_) => Ok( Self(String::from_str(input).unwrap()) ),
                _ => return Err(ArgumentValidationError::UnknownError)
            }
        }
    }

    #[derive(Debug)]
    pub struct MinimumReadyTime(u32);
    
    impl FromStr for MinimumReadyTime {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            let mut factor: u32 = 1;
            let mut input_numeric = "";
            
            if input.len() > 0 {
                if input.ends_with('s') {
                    input_numeric = input.get(0..(input.len()-1)).unwrap();
                    factor = 1
                }
                else if input.ends_with('m') {
                    input_numeric = input.get(0..(input.len()-1)).unwrap();
                    factor = 60
                }
                else if input.ends_with('h') {
                    input_numeric = input.get(0..(input.len()-1)).unwrap();
                    factor = 60 * 60
                }
                else if input.ends_with('d') {
                    input_numeric = input.get(0..(input.len()-1)).unwrap();
                    factor = 60 * 60 * 24
                }
            }
            if input_numeric.len() == 0 {
                input_numeric = input.clone();
            }

            let value = match input_numeric.parse::<u32>() {
                Ok(value)  => value * factor,
                Err(_) => return Err(ArgumentValidationError::InvalidNumericInput),
            };
            Ok(MinimumReadyTime(value))
        }
    }
    impl MinimumReadyTime {
        pub fn as_u32(self) -> u32 {
            self.0
        }
    }

    #[derive(Debug)]
    pub struct MinimumCandidates(u32);

    impl FromStr for MinimumCandidates {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            let min_candidates: u32 = 1;

            match value_minimum_check(input, min_candidates) {
                Err(ArgumentValidationError::InvalidNumericInput) => return Err(ArgumentValidationError::InvalidNumericInput),
                Err(ArgumentValidationError::LessThanMinimum) => {
                    eprint!("Minimum number of candidate nodes is {0} - ", min_candidates);
                    return Err(ArgumentValidationError::LessThanMinimum)
                },
                Ok(_) => Ok( Self(u32::from_str(input).unwrap()) ),
                _ => return Err(ArgumentValidationError::UnknownError)
            }
        }
    }
    impl MinimumCandidates {
        fn as_u32(self) -> u32 {
            self.0
        }
    }

    #[derive(Debug)]
    pub struct MaximumImageAge(u32);

    impl FromStr for MaximumImageAge {
        type Err = ArgumentValidationError;

        fn from_str(input: &str) -> Result<Self, ArgumentValidationError> {
            let minimum_max_image_age: u32 = 0;

            match value_minimum_check(input, minimum_max_image_age) {
                Err(ArgumentValidationError::InvalidNumericInput) => return Err(ArgumentValidationError::InvalidNumericInput),
                Err(ArgumentValidationError::LessThanMinimum) => {
                    eprint!("Maximum image age in days should be at least {0} - ", minimum_max_image_age);
                    return Err(ArgumentValidationError::LessThanMinimum)
                },
                Ok(_) => Ok( Self(u32::from_str(input).unwrap()) ),
                _ => return Err(ArgumentValidationError::UnknownError)
            }
        }
    }
    impl MaximumImageAge {
        fn as_u32(self) -> u32 {
            self.0
        }
    }

    /// Returns an empty Result if the input is an int of at least minimum value or reports an error otherwise
    fn value_minimum_check(input: &str, minimum: u32) -> Result<(), ArgumentValidationError> {
        let number = match input.parse::<u32>() {
            Ok(number)  => number,
            Err(_) => return Err(ArgumentValidationError::InvalidNumericInput),
        };
        if number < minimum {
            return Err(ArgumentValidationError::LessThanMinimum)
        }

        Ok(())
    }

    /// Returns an empty Result if the node name is a valid vmss node name and returns an error otherwise
    fn is_vmss_node(name: &str) -> Result<(), ArgumentValidationError> {
        let re = Regex::new(r"^\S+-vmss[0-9a-z]{6}$").unwrap();
        match re.is_match(name) {
            true => Ok(()),
            false => Err(ArgumentValidationError::InvalidNodeName)
        }
    }

    /// Returns an empty Result if the name is a valid vmss name and returns an error otherwise
    fn is_vmss_name(name: &str) -> Result<(), ArgumentValidationError> {
        let re = Regex::new(r"^\S+-vmss$").unwrap();
        match re.is_match(name) {
            true => Ok(()),
            false => Err(ArgumentValidationError::InvalidVmssName)
        }
    }

    /// Returns an empty Result if the subscription ID is a valid GUID and returns an error otherwise
    fn is_subscription_id(name: &str) -> Result<(), ArgumentValidationError> {
        let re = Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap();
        match re.is_match(name) {
            true => Ok(()),
            false => Err(ArgumentValidationError::InvalidSubscriptionId)
        }
    }

    /// Returns the name of the SIG for a given unique name (usually the resource group name) for that cluster
    fn get_vmss_sig(name: &str) -> String {
        // SIG names can not have "-" but can have "_"
        // SIG names also must be unique across the subscription even if
        // they are within different resource groups!

        format!("SIG_{0}", name.replace("-", "_"))
    }

    /// Get the sorted list of VMSS that could be set to run prototype pattern
    pub fn get_vmss_set() {
        // TODO: fill in for status and auto-update
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_log_prefix() {
            assert_eq!(LogPrefix::from_str("~!}[/.09az_|").unwrap().to_string(), "~!}[/.09az_|");

            // failure cases:
            assert!(LogPrefix::from_str("~!}[/.09az_|:").is_err());
            assert!(LogPrefix::from_str(" ~!}[/.09az_|").is_err());

        }

        #[test]
        fn test_new_updated_nodes_constructor() {
            assert_eq!(NewUpdatedNodes::from_str("0").unwrap().as_u32(), 0);
            assert_eq!(NewUpdatedNodes::from_str("1028").unwrap().as_u32(), 1028);
            assert_eq!(NewUpdatedNodes::from_str("1234567890").unwrap().as_u32(), 1234567890);

            // failure cases:
            assert!(NewUpdatedNodes::from_str("").is_err());
            assert!(NewUpdatedNodes::from_str("-1").is_err());
            assert!(NewUpdatedNodes::from_str("1A").is_err());
            assert!(NewUpdatedNodes::from_str("17.1").is_err());
            assert!(NewUpdatedNodes::from_str("/").is_err());
            assert!(NewUpdatedNodes::from_str(":").is_err());

        }
        
        // we should not allow struct creation outside of the constructor from_str()
        // this test currently fails (it should panic)
        // #[test]
        // #[should_panic]
        // fn test_new_updated_nodes_constructor_panic_without_new() {
        //     let n = NewUpdatedNodes(25);
        //     assert_eq!(n.as_u32(), 25);
        // }
        
        #[test]
        fn test_value_minimum_check() {
            assert!(value_minimum_check("0", 0).is_ok());
            assert!(value_minimum_check("1234567890", 0).is_ok());

            // false/failure cases
            assert!(value_minimum_check("3", 10).is_err());
            assert!(value_minimum_check("-1", 10).is_err());
            assert!(value_minimum_check("pure alphabetic string", 10).is_err());
            assert!(value_minimum_check("123alphanumeric", 10).is_err());
            assert!(value_minimum_check("/", 10).is_err());
            assert!(value_minimum_check(":", 10).is_err());
        }
        #[test]
        fn test_is_vmss_node() {
            assert!(is_vmss_node("k8s-pool-131414-vmss0003ax").is_ok());
            assert!(is_vmss_node("k8s-pool-131414-vmss000000").is_ok());
            assert!(is_vmss_node("k8s-pool-131414-vmss999999").is_ok());
            assert!(is_vmss_node("k8s-pool-131414-vmssaaaaaa").is_ok());
            assert!(is_vmss_node("k8s-pool-131414-vmsszzzzzz").is_ok());
            assert!(is_vmss_node("A-vmss0003ax").is_ok());
            assert!(is_vmss_node("_-vmss0003az").is_ok());

            // false/failure cases
            assert!(is_vmss_node("k8s-pool-131414-Vmss0003aX").is_err());
            assert!(is_vmss_node("k8s-pool-131414-vmss0003aX").is_err());
            assert!(is_vmss_node("k8s-pool-131414-vmss//////").is_err());
            assert!(is_vmss_node("k8s-pool-131414-vmss::::::").is_err());
            assert!(is_vmss_node("k8s-pool-131414-vmss``````").is_err());
            assert!(is_vmss_node("k8s-pool-131414-vmss{{{{{{").is_err());
            assert!(is_vmss_node("A-vmss0003a").is_err());
            assert!(is_vmss_node("A-vmss00003ax").is_err());
            assert!(is_vmss_node("-vmss0003ax").is_err());
            assert!(is_vmss_node("A-vmss0003a|").is_err());
            assert!(is_vmss_node("Avmss0003ax").is_err());
            assert!(is_vmss_node("vmss0003ax").is_err());
        }
        #[test]
        fn test_is_vmss_name() {
            assert!(is_vmss_name("k8s-pool-131414-vmss").is_ok());
            assert!(is_vmss_name("_-vmss").is_ok());
            assert!(is_vmss_name("").is_ok());
            
            // false/failure cases
            assert!(is_vmss_name("-vmss").is_err());
            assert!(is_vmss_name("k8s-pool-131414-vMSs").is_err());
            assert!(is_vmss_name("Avmss").is_err());
            assert!(is_vmss_name("vmss").is_err());
        }
        #[test]
        fn test_subscription_id_constructor() {
            assert!(SubscriptionId::from_str("12345678-1234-1234-1234-123456789012").is_ok());
            assert!(SubscriptionId::from_str("abcdefab-abcd-abcd-abcd-abcddefabcde").is_ok());
            assert!(SubscriptionId::from_str("123DEFab-aB3D-ABC4-AB3D-ABC4DEFABC99").is_ok());

            // false/failure cases
            assert!(SubscriptionId::from_str("12345678_1234_1234_1234_123456789012").is_err());
            assert!(SubscriptionId::from_str("123456780123401234012340123456789012").is_err());
            assert!(SubscriptionId::from_str("abcdefgh-abcd-abcd-abcd-abcddefghijk").is_err());
        }

        #[test]
        fn test_minimum_ready_time_constructor() {
            assert_eq!(MinimumReadyTime::from_str("0").unwrap().as_u32(), 0);
            assert_eq!(MinimumReadyTime::from_str("3").unwrap().as_u32(), 3);
            assert_eq!(MinimumReadyTime::from_str("3s").unwrap().as_u32(), 3);
            assert_eq!(MinimumReadyTime::from_str("5m").unwrap().as_u32(), 300);
            assert_eq!(MinimumReadyTime::from_str("10h").unwrap().as_u32(), 36000);
            assert_eq!(MinimumReadyTime::from_str("10d").unwrap().as_u32(), 864000);

            // failure cases
            assert!(MinimumReadyTime::from_str("10z").is_err());
            assert!(MinimumReadyTime::from_str("5h3m").is_err());
            assert!(MinimumReadyTime::from_str("5ss").is_err());
            assert!(MinimumReadyTime::from_str("5s ").is_err());
            assert!(MinimumReadyTime::from_str("-1").is_err());
            assert!(MinimumReadyTime::from_str("-1s").is_err());
            assert!(MinimumReadyTime::from_str("-100d").is_err());

        }

        #[test]
        fn test_get_vmss_sig() {
            assert_eq!(get_vmss_sig("kubernetes-westus2-17813"), "SIG_kubernetes_westus2_17813");
            assert_eq!(get_vmss_sig("kubernetes-westus2-17813-_- "), "SIG_kubernetes_westus2_17813___ ");
            assert_eq!(get_vmss_sig(""), "SIG_",);
        }
    }
}

