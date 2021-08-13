// use command::command;

pub mod node_selection {
    use std::convert::TryInto;
    use std::str::FromStr;
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    use regex::Match;
    use regex::Matches;
    use regex::Regex;
    // use serde::{Deserialize, Serialize};
    use serde_json::{Result, Value};
    use json::JsonValue;
    use log::*;

    use crate::argument_types::cluster_update::*;

    // fn vmss_prototype_auto_update(sub_args) {

    // }
    

    /// Get the sorted list of VMSS that could be set to run prototype pattern
    pub fn get_vmss_set() {
        // TODO: fill in for status and auto-update
    }

    /// Get a list of nodes in the current cluster
    // pub fn get_nodes() -> Vec<NodeName> {
    // }

    fn get_nodes_mock(input: String) -> Vec<NodeName> {
        let f = File::open("./redacted-nodes1.json"); // try ? operator
        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let mut buf_reader = BufReader::new(f);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents);
        let parsed = json::parse(&contents).unwrap();
        // parsed["items"].members().collect();

        // TODO this is just placeholder
        vec![NodeName::from_str("k8s-agentpool1-12345678-vmss000f6i").unwrap()]
    }
    fn get_vmss_set_mock(input: String) -> Vec<NodeName> { // TODO check
        // TODO this is just placeholder
        vec![NodeName::from_str("k8s-agentpool1-12345678-vmss000f6i").unwrap()]
    }

    /// Returns an ordered list of candidates, with the longest healthy first.
    /// Candidates are node names for the "prototype" node.
    /// This may be an empty list which would mean that there are no valid candidates.
    // :param vmss:  The name of the VMSS we are operating on
    // :param node_data:  The Nodes items from kubernetes in JSON
    // self_node: The node we are running on 
    // :param node_ignore_names:  The list of node names to ignore
    // :param node_ignore_annotations:  The list of node annotations that, if existing, will cause the node to be ignored
    // :param minimum_ready_time:  Minimum ready time before a node is considered
    // :param pending_reboot_annotation:  The annotation on a node that signals it is pending a reboot
    // :param latest_patch_annotation:  The annotation on a node that holds the date/time of the last OS patch update
    // :latest_image:  The version string of the current latest image (or None, if OS patch should not be considered)
    // :return:  An list of those nodes that passed the qualifying criteria
    fn get_candidate_nodes( vmss: VmssName, node_data: JsonValue, node_ignore_names: Option<Vec<NodeName>>, node_ignore_annotations: String,
                            minimum_ready_time: MinimumReadyTime, pending_reboot_annotation: String, last_patch_annotation: String, latest_image: String) -> Vec<NodeName> {
        let candidates: Vec<NodeName> = Vec::new();
        let vmss_name = vmss.to_string();
        let node_ignore_names_exists = node_ignore_names.is_some();
        // let node_ignore_names_vec = node_ignore_names.get_or_insert(Vec::new());
        let node_ignore_names_vec = node_ignore_names.unwrap_or_else(|| Vec::new());
        // let node_ignore_names_vec = match node_ignore_names.is_some() {
        //     true => node_ignore_names.expect(),
        //     false => vec![NodeName::from_str("").unwrap()],
        // };
        for node in node_data["items"].members() {
            let metadata = &node["metadata"];
            // If not a node in target VMSS, continue...
            let node_name = &metadata["name"].to_string();            
            if !node_name.contains(&vmss_name) {
                debug!("IGNORED: Node {0} not part of vmss {1}", node_name, vmss_name);
                continue
            }

            // TODO: pass this value as a parameter
            // if node_name == os.getenv('NODE_ID', None):
                // logging.debug('IGNORED: Node {0} is the same as the node we are running on'.format(node_name))
                // continue

            // get the node we are running on so we don't select it as a target
            if node_name == os.getenv("NODE_ID") { //:self_node // TODO figure this out
                debug!("IGNORED: Node {0} is the same as the node we are running on", node_name);
            }

            let node_name_obj = NodeName::from_str(node_name);
            if node_ignore_names_exists && node_name_obj.is_ok() && node_ignore_names_vec.contains(&node_name_obj.unwrap()) {
                debug!("IGNORED: Node {0} is in the set of specific nodes to ignore", node_name);
                continue
            }

            if node["spec"] == "unschedulable" {
                debug!("IGNORED: Node {0} as it is marked unschedulable", node_name);
                continue
            }

            // Filtering out masters normally is not needed unless they are, for
            // some reason, within a VMSS.  We still don't want to make fresh images
            // here as masters are a bit special.
            if metadata["labels"]["kubernetes.io/role"] == "master" {
                debug!("IGNORED: Node {0} is a control plane node", node_name);
                continue
            }

            let annotations = &metadata["annotations"];
            let node_ignore_annotations_vec: Vec<&str> = node_ignore_annotations.split(" ").collect();
            println!("node_ignore_annotations: '{0}'", node_ignore_annotations);
            // TODO: make this a function that returns a bool
            let mut skip_for_annotation = false;
            for annotation in annotations.entries() {
                let annotation_value = match annotation.1.as_str() {
                    Some(annotation_value) => annotation_value,
                    None => continue
                };
                // TODO: it's about if the annotation is set; so "LatestOSPatch" or "ZombieKiller"
                if node_ignore_annotations_vec.contains(&annotation_value) {
                    debug!("IGNORED: Node {0} is annotated as needing to be ignored: {1}", node_name, annotation_value);
                    skip_for_annotation = true;
                    break;
                }
                if pending_reboot_annotation == annotation_value {
                    debug!("IGNORED: Node {0} is annotated as pending reboot: {1}", node_name, pending_reboot_annotation);
                    skip_for_annotation = true;
                    break;
                }
            }
            if skip_for_annotation {
                continue
            }

            if !latest_image.is_empty() {
                let last_patch = &annotations["last_patch_annotation"];
                if last_patch.is_empty() {
                    debug!("IGNORED: Node {0} does not have a last patch annotation: {1}", node_name, last_patch_annotation);
                    continue
                }

                let re = Regex::new(r"\d\d\d\d-\d\d-\d\d").unwrap();
                if re.is_match(&last_patch.to_string()) { // TODO: check this, logic could be wrong
                    debug!("IGNORED: Node {0} last patch annotation does not have a valid format: {1}={2}", node_name, last_patch_annotation, last_patch);
                    continue
                }

                // patch_version = last_patch_date_match.group(0).replace('-', '.')
                // re.find_iter(&last_patch.to_string()).collect::<Match>().get(0);
                re.find_iter(&last_patch.to_string()).next();
                        // .collect::<Match>().get(0);
                // if patch_version <= latest_image:
                //     logging.debug('IGNORED: Node {0} latest patch of {1} not newer that latest image version {2}'.format(node_name, patch_version, latest_image))
                //     continue
            }

            // ready_time = None
            // for condition in node.get('status', {}).get('conditions', []):
            //     if condition.get('type', '') == 'Ready':
            //         if condition.get('status', '') == 'True':
            //             ready_time = int(mktime_from_kubernetes(condition['lastHeartbeatTime']) - mktime_from_kubernetes(condition['lastTransitionTime']))
            //         break
            // if not ready_time:
            //     logging.debug('IGNORED: Node {0} is not ready'.format(node_name))
            //     continue

            // if ready_time < minimum_ready_time:
            //     logging.debug('IGNORED: Node {0} has been ready for only {1}s and the minimum is {2}s'.format(node_name, ready_time, minimum_ready_time))
            //     continue

            // logging.debug('CANDIDATE: Node {0} ready for {1}s'.format(node_name, ready_time))
            // candidates.append({'node': node_name, 'ready': ready_time})
// '
            // TODO bookmark
        }


        // candidates.sort(key=lambda data: data['ready'], reverse=True)
        // return [candidate['node'] for candidate in candidates]

        // TODO this is just placeholder
        vec![NodeName::from_str("k8s-agentpool1-12345678-vmss000f6i").unwrap()]
    }
    
    #[cfg(test)]
    mod test {
        // use std::path::Path;

        // use json::JsonValue;

        use super::*;

        /*
        #[test]
        fn test_serde_json() {
            // fake version of get_nodes() that loads the json file
            // TODO ...

            let f = File::open("./redacted-nodes1.json"); // try ? operator
            let f = match f {
                Ok(file) => file,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };
            let mut buf_reader = BufReader::new(f);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents);

            let parsed = serde_json::from_str(&content).unwrap();
            let mut node_names: String = String::new();
            for item in parsed["items"].members() {
                // println!("{0}", item["metadata"]["name"].to_string());
                node_names += &item["metadata"]["name"].to_string();
                node_names += " ";
            }
            assert_eq!(node_names.split(" ").collect::<Vec<&str>>()[0], parsed["items"][0]["metadata"]["name"]);
            // println!("node_names: {0}", node_names);
            // assert_eq!(parsed["payload"][0][0], "awesome");

            /***** TEMPORARY TESTS: *******/

            // let t = &parsed["items"][0]["metadata"]["annotations"]["last_patch_annotation"];
            // // let t = &parsed["items"][0]["metadata"]["annotations"]["LatestOSPatch"];

            // if !t.is_null() {
            //     println!("last patch annotation: {}", t);
            // }
            // else {
            //     println!("no t!");
            // }

            let node_ignore_annotations = "food 2021-07-04T02:56:07+00:00";

            let annotations = &parsed["items"][0]["metadata"]["annotations"];
            // ignore = [annotation for annotation in node_ignore_annotations if annotation in annotations]
            // if ignore {
            //     debug!("IGNORED: Node {0} is annotated as needing to be ignored: {1}", node_name, ignore);
            //     continue
            // }
            for annotation in annotations.members() {
                if node_ignore_annotations.contains(&annotation.to_string()) {
                    continue
                }
            }

            println!("name: {}", parsed["items"][0]["metadata"]["name"]);
            println!("annotations: {:?}", annotations.members());
            println!("annotations: {:?}", &parsed["items"][0]["metadata"]["annotations"]);

            let v1 = annotations.members().filter(|annotation| node_ignore_annotations.contains(&annotation.to_string())).collect::<Vec<&JsonValue>>();
            if !v1.is_empty() {
                println!("contains the thing!");
            }
            else {
                println!("doesn't contain!");
            }

        }
        */

        #[test]
        fn test_get_nodes_mock() {
            // fake version of get_nodes() that loads the json file
            // TODO ...

            let f = File::open("./redacted-nodes1.json"); // try ? operator
            let f = match f {
                Ok(file) => file,
                Err(error) => panic!("Problem opening the file: {:?}", error),
            };
            let mut buf_reader = BufReader::new(f);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents);

            let parsed = json::parse(&contents).unwrap();
            let mut node_names: String = String::new();
            for item in parsed["items"].members() {
                // println!("{0}", item["metadata"]["name"].to_string());
                node_names += &item["metadata"]["name"].to_string();
                node_names += " ";
            }
            assert_eq!(node_names.split(" ").collect::<Vec<&str>>()[0], parsed["items"][0]["metadata"]["name"]);
            // println!("node_names: {0}", node_names);
            // assert_eq!(parsed["payload"][0][0], "awesome");

            /***** TEMPORARY TESTS: *******/

            // let t = &parsed["items"][0]["metadata"]["annotations"]["last_patch_annotation"];
            // // let t = &parsed["items"][0]["metadata"]["annotations"]["LatestOSPatch"];

            // if !t.is_null() {
            //     println!("last patch annotation: {}", t);
            // }
            // else {
            //     println!("no t!");
            // }

            let node_ignore_annotations = "food {\"secrets-store.csi.k8s.io\":\"k8s-agentpool1-12345678-vmss000f6i\"} 2021-07-04T02:56:07+00:00 true";
            
            let annotations = &parsed["items"][0]["metadata"]["annotations"];

            let node_ignore_annotations_vec: Vec<&str> = node_ignore_annotations.split(" ").collect();
            println!("node_ignore_annotations: '{0}'", node_ignore_annotations);
            for annotation in annotations.entries() {
                println!("{:?}", annotation);
                // let annotation_value = match annotation.1.as_str() {
                let annotation_value = annotation.0; // this might be an error
                // TODO: why is this match here?
                // let annotation_value = match annotation.0 {
                //     Some(annotation_value) => annotation_value, // TODO: fix this
                //     None => {
                //         println!("                          FAILURE TO PARSE ================");
                //         continue
                //     },
                // };
                // if node_ignore_annotations_vec.contains(&annotation.1.as_str().unwrap()) {
                //     println!("      got a hit with current annotation in loop: {:?}", annotation.1.as_str().unwrap());
                // }
                if node_ignore_annotations_vec.contains(&annotation_value) {
                    println!("      got a hit with current annotation in loop: {:?}", &annotation_value);
                }
            }

            let latest_image: String = String::from_str("latest image name").unwrap();

            if !latest_image.is_empty() {
                // let last_patch = &annotations["last_patch_annotation"];
                let last_patch = &annotations["node.alpha.kubernetes.io/ttl"];
                if last_patch.is_empty() {
                    println!("IGNORED: Node {0} does not have a last patch annotation: {1}", "node_name", "last_patch_annotation");
                    // continue
                }
                else {
                    println!("last_patch is not empty: {:?}", last_patch);
                }
            }
        }

        #[test]
        fn test_get_candidate_nodes() {
            // fake version of get_nodes() that loads the json file
            // TODO ...

            /* HARDCODE:
                    vmss
                    node_ignore_names - process into hardcode
                    node_ignore_annotations - process into hardcode
                    sub_args.minimum_ready_time,
                    sub_args.pending_reboot_annotation,
                    sub_args.last_patch_annotation,
                    latest_image
            */
            // TEST SETUP:
            //
            let vmss = VmssName::from_str("k8s-agentpool1-12345678-vmss").unwrap();
            let nodes = get_nodes_mock(String::from_str("").unwrap());
            let vmss_names = get_vmss_set_mock(String::from_str("").unwrap());

            let node_ignore_names: Option<Vec<NodeName>> = Some(Vec::new());
            let node_ignore_annotations = String::from_str("").unwrap();
            let minimum_ready_time = MinimumReadyTime::from_str("60").unwrap();
            let pending_reboot_annotation = String::from_str("").unwrap();
            let last_patch_annotation = String::from_str("").unwrap();
            let latest_image = "0000.00.00";

            // get_candidate_nodes(vmss, nodes, node_ignore_names, node_ignore_annotations, minimum_ready_time, pending_reboot_annotation, last_patch_annotation, latest_image);
        }
    }
}