#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short, Address, log};

#[contracttype]
#[derive(Clone)]
pub struct Transcript {
    pub student: Address,
    pub course_name: String,
    pub grade: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum TranscriptKey {
    Record(u64),
    Count,
}

#[contract]
pub struct TranscriptContract;

#[contractimpl]
impl TranscriptContract {
    pub fn add_transcript(env: Env, student: Address, course_name: String, grade: String) -> u64 {
        let mut count: u64 = env.storage().instance().get(&TranscriptKey::Count).unwrap_or(0);
        count += 1;

        let transcript = Transcript {
            student: student.clone(),
            course_name,
            grade,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&TranscriptKey::Record(count), &transcript);
        env.storage().instance().set(&TranscriptKey::Count, &count);

        log!(&env, "Transcript Added for: {}", student);

        count
    }

    pub fn get_transcript(env: Env, record_id: u64) -> Transcript {
        env.storage().instance().get(&TranscriptKey::Record(record_id)).unwrap()
    }

    pub fn total_transcripts(env: Env) -> u64 {
        env.storage().instance().get(&TranscriptKey::Count).unwrap_or(0)
    }
}
