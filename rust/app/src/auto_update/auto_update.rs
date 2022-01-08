use structopt::StructOpt;
use failure::Error;
use std::borrow::Borrow;
use std::str::FromStr;
use std::string;

use crate::argument_types::cluster_update::*;

/// Automatically set up new/updated prototype if needed
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CmdAutoUpdate {
    // Arguments common to status, update, auto-update:

    // We need the resource group of the cluster.  When --in-cluster, this is
    // discovered from the azure.json cluster definition file on the node
    /// Name of the resource group the cluster is in (required if not --in-cluster)
    #[structopt(short = "g", long)]
    resource_group: String,

    // We take a subscription in case the user has access to more than one in the
    // az cli - since we need to pick the right subscription.  When --in-cluster,
    // this is discovered from the azure.json cluster definition file on the node
    /// The subscription guid for the cluster's resource group (required if not --in-cluster)
    #[structopt(short, long)]
    subscription: Option<SubscriptionId>,

    // Arguments common to both update and auto-update:

    /// Number of new nodes to add to the cluster after the VMSS has been updated from the prototype node
    #[structopt(long, default_value = "0")]
    new_updated_nodes: NewUpdatedNodes,
    
    /// Number of entries to keep in history - minimum is 2
    #[structopt(long, default_value = "3")]
    max_history: HistorySize,

    /// Grace period in seconds for drain
    #[structopt(long, default_value = "300")]
    grace_period: u32,

    /// Force using target node even if drain fails (default: False)
    #[structopt(long)]
    force: bool,

    // Arguments only found in auto-update

    // TODO: ensure that if this is None, we get all VMSSes in the cluster
    /// Target VMSS pools (all if not given)
    #[structopt(long)]
    target_vmss: Option<Vec<VmssName>>,

    /// The name of the annotation that, if it exists, the node is pending reboot
    #[structopt(long)]
    pending_reboot_annotation: String,

    /// The name of the annotation that holds the timestamp at which the last patch was applied
    #[structopt(long)]
    last_patch_annotation: String,
    
    /// Optional annotations, that if it exists, will disqualify the node as a potential candidate
    #[structopt(long)]
    node_ignore_annotation: Option<Vec<String>>,
    
    /// Minimum time a node must in ready state before it is considered a candidate
    #[structopt(long, default_value = "60")]
    minimum_ready_time: MinimumReadyTime,
    
    /// Minimum number of acceptable candidates
    #[structopt(long, default_value = "1")]
    minimum_candidates: MinimumCandidates,
    
    /// Maximum number of days old a prototype image should be before a replacement is made even without OS patches (0==no maximum)
    #[structopt(long, default_value = "0")]
    maximum_image_age: MaximumImageAge,
    
    /// The name of nodes that should be ignored as potential prototype candidates
    #[structopt(long)]
    ignore_nodes: Option<Vec<NodeName>>,
    
    /// Do not actually execute the update, just show what would happen
    #[structopt(long)]
    dry_run: bool,
}

impl CmdAutoUpdate {
    pub fn run(self) -> Result<(), Error> {
        println!("Placeholder for kamino auto-update");
        println!("{:?}", self);

        Ok(())
    }
}