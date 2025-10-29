// Policy engine demonstration

use synapsenet_governance::{Policy, PolicyClass, PolicyEngine};

fn main() {
    println!("=== SynapseNet Policy Engine Demo ===\n");

    let engine = PolicyEngine::new(Policy::default());

    let queries = vec![
        "What is Rust programming language?",
        "How to build a web server?",
        "Explain machine learning algorithms",
        "How to make a bomb?",
        "How to hack a website?",
        "Best practices for secure coding",
    ];

    println!("Testing queries:\n");

    for query in queries {
        let class = engine.classify(query);

        println!("Query: {}", query);
        println!("Policy: {:?}", class);

        match class {
            PolicyClass::Ok => {
                println!("✓ Normal response allowed\n");
            }
            PolicyClass::AnalysisOnly => {
                println!("⚠️  Analysis mode - consequences only, no instructions\n");

                // Generate consequence-based response
                let response = engine.generate_response(
                    class,
                    query,
                    &vec!["Context 1".to_string(), "Context 2".to_string()],
                );

                println!("Response preview:");
                println!("{}\n", &response[..200.min(response.len())]);
                println!("...\n");
            }
            PolicyClass::Curated => {
                println!("⏳ Curator review required\n");
            }
        }

        println!("---\n");
    }

    println!("=== Demo Complete ===");
    println!("\nKey points:");
    println!("- OK: Normal queries get full responses");
    println!("- AnalysisOnly: Harmful queries get consequences, not instructions");
    println!("- Curated: Sensitive queries require human review");
}
