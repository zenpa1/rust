#[derive(Debug)]
enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
struct TelemetryPacket {
    id: u32,
    severity: Severity,
    damage: u32,
    label: String,
}

fn main() {
    let mut vec: Vec<TelemetryPacket> = vec![
        TelemetryPacket {
            id: 1,
            severity: Severity::Low,
            damage: 5,
            label: String::from("Magic Basic Attack"),
        },
        TelemetryPacket {
            id: 2,
            severity: Severity::Medium,
            damage: 20,
            label: String::from("Fireball"),
        },
        TelemetryPacket {
            id: 3,
            severity: Severity::High,
            damage: 50,
            label: String::from("Meteor Strike"),
        },
        TelemetryPacket {
            id: 4,
            severity: Severity::Low,
            damage: 8,
            label: String::from("Physical Basic Attack"),
        },
        TelemetryPacket {
            id: 5,
            severity: Severity::High,
            damage: 47,
            label: String::from("Shadow Piercer"),
        },
        TelemetryPacket {
            id: 6,
            severity: Severity::Critical,
            damage: 127,
            label: String::from("Ultimate: Cross-fade Slash"),
        },
    ];

    let mut vec_iter = vec.iter_mut().peekable();
    println!("Peeking first packet: {:?}", vec_iter.peek());

    // println!("next() still shows the first packet: {:?}", vec_iter.next());
    // println!("Peeking second packet: {:?}", vec_iter.peek());
    // println!("Peeking second packet again: {:?}", vec_iter.peek());
    // println!("Peeking second packet again: {:?}", vec_iter.peek());

    let header_batch = vec_iter.by_ref().take(2).collect::<Vec<_>>();
    println!(
        "The header batch contains the first 2 packets: {:#?}",
        header_batch
    );
    println!(
        "While the original iterator variable (vec_iter) still contains the rest: {:#?}",
        vec_iter
    );

    // Using the remaining elements of the original iterator
    let remaining_batch: Vec<&mut TelemetryPacket> = vec_iter
        .filter(|element: &&mut TelemetryPacket| match element.severity {
            Severity::Low => false,
            Severity::Medium => true,
            Severity::High => true,
            Severity::Critical => true,
        })
        .map(|element: &mut TelemetryPacket| {
            element.damage *= 2;
            element
        })
        .collect();

    println!(
        "The remaining batch that filtered out low-severity packets and doubled the damage value of each packet contains the following: {:#?}",
        remaining_batch
    );

    let total_damage: u32 = remaining_batch
        .iter()
        .map(|element: &&mut TelemetryPacket| element.damage)
        .sum();
    println!("Total doubled damage sums up to: {}", total_damage);
}
