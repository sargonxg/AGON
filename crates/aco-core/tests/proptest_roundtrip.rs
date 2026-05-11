//! Property tests: every primitive round-trips through JSON.

use aco_core::*;
use proptest::prelude::*;

fn arb_id() -> impl Strategy<Value = Id> {
    any::<[u8; 32]>().prop_map(Id)
}

fn arb_actor_kind() -> impl Strategy<Value = ActorKind> {
    prop_oneof![
        Just(ActorKind::Person),
        Just(ActorKind::Organisation),
        Just(ActorKind::Coalition),
        Just(ActorKind::State),
        Just(ActorKind::Group),
        Just(ActorKind::Unknown),
    ]
}

fn arb_provenance() -> impl Strategy<Value = Provenance> {
    (any::<[u8; 16]>(), 0.0f32..=1.0f32).prop_map(|(fp, c)| Provenance {
        extractor: "test".into(),
        prompt_version: "v0".into(),
        prompt_fingerprint: fp,
        spans: vec![],
        confidence: c,
        created_at: chrono::Utc::now(),
        defeasibility: Defeasibility { class: DefeasibilityClass::Defeasible, justification: None },
        derivation: Derivation::Extracted { extractor: "test".into() },
    })
}

fn arb_actor() -> impl Strategy<Value = Actor> {
    (arb_id(), "[a-zA-Z]{3,16}", arb_actor_kind(), 0.0f32..=1.0f32, arb_provenance()).prop_map(
        |(id, name, kind, agency, prov)| Actor {
            id,
            canonical_name: name,
            aliases: vec![],
            kind,
            roles: vec![],
            agency_score: agency,
            prov,
        },
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn id_hex_roundtrip(bytes in any::<[u8; 32]>()) {
        let id = Id(bytes);
        let s = id.to_hex();
        let back: Id = s.parse().unwrap();
        prop_assert_eq!(id, back);
    }

    #[test]
    fn actor_serde_roundtrip(actor in arb_actor()) {
        let j = serde_json::to_string(&actor).unwrap();
        let back: Actor = serde_json::from_str(&j).unwrap();
        prop_assert_eq!(actor, back);
    }

    #[test]
    fn id_canonical_determinism(s in "[a-zA-Z0-9 ]{1,64}") {
        let a = Id::from_canonical(&s);
        let b = Id::from_canonical(&s);
        prop_assert_eq!(a, b);
    }
}

#[test]
fn smoke_all_primitives_serde() {
    let prov = Provenance {
        extractor: "x".into(),
        prompt_version: "v1".into(),
        prompt_fingerprint: [0u8; 16],
        spans: vec![EvidenceSpan {
            chunk_id: "c1".into(),
            char_start: 0,
            char_end: 4,
            quote: "abcd".into(),
        }],
        confidence: 0.8,
        created_at: chrono::Utc::now(),
        defeasibility: Defeasibility { class: DefeasibilityClass::Strict, justification: None },
        derivation: Derivation::Extracted { extractor: "x".into() },
    };
    let id = Id::from_canonical("smoke");

    let actor = Actor {
        id,
        canonical_name: "A".into(),
        aliases: vec![],
        kind: ActorKind::Person,
        roles: vec![],
        agency_score: 0.5,
        prov: prov.clone(),
    };
    let _: Actor = serde_json::from_str(&serde_json::to_string(&actor).unwrap()).unwrap();

    let claim = Claim {
        id,
        speaker: id,
        addressed_to: None,
        proposition: "p".into(),
        modality: Modality::Asserted,
        speech_act: SpeechAct::Assertive,
        polarity: Polarity::Positive,
        stance: Stance::Endorses,
        intensity: 0.5,
        interval: TemporalInterval::Unknown,
        prov: prov.clone(),
    };
    let _: Claim = serde_json::from_str(&serde_json::to_string(&claim).unwrap()).unwrap();

    let interest = Interest {
        id,
        holder: id,
        description: "i".into(),
        category: InterestCategory::Material,
        priority: 0.5,
        stated: true,
        utility_proxy: None,
        prov: prov.clone(),
    };
    let _: Interest = serde_json::from_str(&serde_json::to_string(&interest).unwrap()).unwrap();

    let constraint = Constraint {
        id,
        binds: vec![id],
        source: "c".into(),
        modality: Deontic::Obligation,
        content: "x".into(),
        formal: None,
        prov: prov.clone(),
    };
    let _: Constraint = serde_json::from_str(&serde_json::to_string(&constraint).unwrap()).unwrap();

    let leverage = Leverage {
        id,
        holder: id,
        target: id,
        mechanism: LeverageKind::Authority,
        magnitude: 0.5,
        activation_cost: 0.3,
        credibility: 0.7,
        description: "L".into(),
        prov: prov.clone(),
    };
    let _: Leverage = serde_json::from_str(&serde_json::to_string(&leverage).unwrap()).unwrap();

    let commitment = Commitment {
        id,
        committed_by: id,
        committed_to: vec![id],
        content: "do x".into(),
        deadline: None,
        conditionals: vec![],
        status: CommitmentStatus::Pending,
        verification: None,
        prov: prov.clone(),
    };
    let _: Commitment = serde_json::from_str(&serde_json::to_string(&commitment).unwrap()).unwrap();

    let event = Event {
        id,
        event_type: "meeting".into(),
        participants: vec![Participant { actor: id, role: Role::Agent }],
        interval: TemporalInterval::Unknown,
        place: None,
        causes: vec![],
        effects: vec![],
        prov: prov.clone(),
    };
    let _: Event = serde_json::from_str(&serde_json::to_string(&event).unwrap()).unwrap();

    let narrative = Narrative {
        id,
        author: id,
        frame: "f".into(),
        claims: vec![],
        events: vec![],
        villain: None,
        hero: None,
        victim: None,
        coherence: 0.5,
        prov,
    };
    let _: Narrative = serde_json::from_str(&serde_json::to_string(&narrative).unwrap()).unwrap();
}
