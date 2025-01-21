use std::any::Any;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use crate::routing::inter::routing_table::{RestartListener, RoutingTable};
use crate::utils::node::Node;
use crate::utils::uid::UID;

pub struct MRoutingTable {

}

impl MRoutingTable {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl RoutingTable for MRoutingTable {

    fn get_update_public_ip_consensus(&self) -> fn(Arc<Mutex<dyn RoutingTable>>, IpAddr, IpAddr) {
        Self::update_public_ip_consensus
    }

    fn update_public_ip_consensus(routing_table: Arc<Mutex<dyn RoutingTable>>, source: IpAddr, addr: IpAddr) {
        todo!()
    }

    fn set_external_address(&mut self, address: IpAddr) {
        todo!()
    }

    fn get_consensus_external_address(&self) -> IpAddr {
        todo!()
    }

    fn insert(&mut self, n: Node) {
        todo!()
    }

    fn derive_uid(&mut self) {
        todo!()
    }

    fn get_derived_uid(&self) -> UID {
        todo!()
    }

    fn is_secure_only(&self) -> bool {
        todo!()
    }

    fn set_secure_only(&mut self, secure_only: bool) {
        todo!()
    }

    fn add_restart_listener(&mut self, listener: RestartListener) {
        todo!()
    }

    fn remove_restart_listener(&mut self, index: usize) {
        todo!()
    }

    fn has_queried(&self, n: &Node, now: u128) -> bool {
        todo!()
    }

    fn bucket_uid(&self, k: &UID) -> usize {
        todo!()
    }

    fn all_nodes(&self) -> Vec<Node> {
        todo!()
    }

    fn find_closest(&self, k: &UID, r: usize) -> Vec<Node> {
        todo!()
    }

    fn bucket_size(&self, i: usize) -> usize {
        todo!()
    }

    fn all_unqueried_nodes(&self) -> Vec<Node> {
        todo!()
    }

    fn get_restart(&self) -> fn(Arc<Mutex<dyn RoutingTable>>) {
        Self::restart
    }

    fn restart(routing_table: Arc<Mutex<dyn RoutingTable>>) {
        todo!()
    }

    fn upcast(&self) -> &dyn RoutingTable {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
