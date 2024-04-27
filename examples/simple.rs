use std::{collections::HashSet, iter::FromIterator, str::FromStr, thread::park, time::Duration};

use bluster::{
    gatt::{
        characteristic::{Characteristic, Properties, Read, Secure},
        descriptor::{self, Descriptor},
        service::Service,
    },
    Peripheral,
};
use dbus::Message;
use futures::channel::mpsc::Sender;
use tokio::time::sleep;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let peripheral = Peripheral::new().await.unwrap();
    let properties = Properties::new(None, None, None, None);

    let service_uuid = Uuid::from_str("7b3b63bc-5b5c-4292-9e6a-03e1b387d68f").unwrap();
    let characteristic = Characteristic::new(
        Uuid::from_str("8b8e1f10-ab76-4651-87f2-a581f8029c70").unwrap(),
        properties,
        Some(vec![1, 2, 3]),
        vec![Descriptor::new(
            service_uuid,
            descriptor::Properties::new(None, None),
            Some(vec![4, 5, 6]),
        )]
        .into_iter()
        .collect(),
    );
    let characteristics: HashSet<_> = vec![characteristic].into_iter().collect();
    let service = Service::new(service_uuid, false, characteristics);
    peripheral.add_service(&service).unwrap();
    let peripheral_name = "Bluster Simple";
    peripheral
        .start_advertising(peripheral_name, &[service_uuid])
        .await
        .unwrap();
    println!(
        "Started advertising! Connect to {:?} to try it out!",
        peripheral_name
    );
    sleep(Duration::from_secs(1)).await;
    async_ctrlc::CtrlC::new().unwrap().await;
    peripheral.stop_advertising().await.unwrap();
    println!("Stopped advertising!");
}
