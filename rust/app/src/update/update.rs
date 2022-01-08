use structopt::StructOpt;
use failure::Error;
// use crate::argument_types;
// use crate::argument_types::cluster_update::NewUpdatedNodes;
use crate::argument_types::cluster_update::*;

/// Set up new/updated prototype
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CmdUpdate {
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
    // max_history: u32,
    max_history: HistorySize,

    /// Grace period in seconds for drain
    #[structopt(long, default_value = "300")]
    grace_period: u32,

    /// Force using target node even if drain fails (default: False)
    #[structopt(long)]
    force: bool,

    // Arguments only found in update

    /// Target node to use as the prototype source
    #[structopt(long)]
    target_node: NodeName,
}

impl CmdUpdate {
    pub fn run(self) -> Result<(), Error> {
        println!("Placeholder for kamino update");
        println!("{:?}", self);
        Ok(())
    }
}

