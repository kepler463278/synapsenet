// Proof of Emergence demonstration

use synapsenet_core::ProofOfEmergence;
use synapsenet_economy::NgtLedger;

fn main() {
    println!("=== Proof of Emergence Demo ===\n");

    let poe = ProofOfEmergence::default();
    let mut ledger = NgtLedger::with_poe(poe);

    println!("PoE Formula: NGT(g) = α * N(g) + β * C(g) + γ * log(1 + R(g))");
    println!(
        "Weights: α={}, β={}, γ={}\n",
        ledger.poe.alpha, ledger.poe.beta, ledger.poe.gamma
    );

    // Simulate different grain contributions
    let scenarios = vec![
        ("High novelty, high coherence", 0.9, 0.8, 0),
        ("Medium novelty, medium coherence", 0.5, 0.5, 0),
        ("Low novelty, high coherence", 0.2, 0.8, 0),
        ("High novelty, low coherence", 0.8, 0.2, 0),
        ("Spam (low both)", 0.05, 0.05, 0),
        ("Popular grain (high reuse)", 0.6, 0.6, 20),
    ];

    println!("Scenarios:\n");

    for (i, (desc, novelty, coherence, reuse)) in scenarios.iter().enumerate() {
        let node_pk = [i as u8; 32];

        let ngt = ledger.award(node_pk, *novelty, *coherence, *reuse);

        println!("{}. {}", i + 1, desc);
        println!("   Novelty: {:.2}", novelty);
        println!("   Coherence: {:.2}", coherence);
        println!("   Reuse: {}", reuse);
        println!("   NGT Reward: {:.4}", ngt);
        println!("   Balance: {:.4}\n", ledger.balance(&node_pk));
    }

    println!("Total NGT Supply: {:.4}\n", ledger.total_supply());

    println!("Top holders:");
    for (i, (pk, balance)) in ledger.top_holders(3).iter().enumerate() {
        println!("{}. Node {:?}... - {:.4} NGT", i + 1, &pk[..4], balance);
    }

    println!("\n=== Demo Complete ===");
    println!("\nKey insights:");
    println!("- High novelty + coherence = maximum reward");
    println!("- Spam (low both) = zero reward (anti-spam)");
    println!("- Reuse increases reward logarithmically");
    println!("- Balance = cumulative contributions");
}
