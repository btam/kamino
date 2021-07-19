use structopt::StructOpt;
use failure::Error;

use crate::argument_types::cluster_update::SubscriptionId;

/// Show status of VMSS Prototype Pattern
#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct CmdStatus {
    // Arguments common to status, update, auto-update:

    /// Name of the resource group the cluster is in (required if not --in-cluster)
    // We need the resource group of the cluster.  When --in-cluster, this is
    // discovered from the azure.json cluster definition file on the node
    #[structopt(short = "g", long)]
    resource_group: String,

    /// The subscription guid for the cluster's resource group (required if not --in-cluster)
    // We take a subscription in case the user has access to more than one in the
    // az cli - since we need to pick the right subscription.  When --in-cluster,
    // this is discovered from the azure.json cluster definition file on the node
    #[structopt(short, long)]
    subscription: Option<SubscriptionId>,
}

impl CmdStatus {
    pub fn run(self) -> Result<(), Error> {
        println!("Placeholder for kamino status");
        println!("{:?}", self);
        Ok(())
    }
}