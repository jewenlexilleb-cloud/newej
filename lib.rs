#![no_std]
use soroban_sdk::{contract, contractimpl, log, Env, Symbol, Address};

// Define storage keys
#[derive(Clone, Copy)]
enum StorageKey {
    GradeData,
}

// Structure to hold grade information
#[derive(Clone, PartialEq, Eq)]
#[soroban_sdk::contracttype]
pub struct GradeRecord {
    student: Address,
    prelim: u32,
    midterm: u32,
    finals: u32,
    final_grade: u32,
}

#[contract]
pub struct GradeChain;

#[contractimpl]
impl GradeChain {
    /// Register scores and automatically compute the final grade.
    /// Weights: Prelim 20%, Midterm 30%, Finals 50%
    pub fn compute_grades(env: Env, student: Address, prelim: u32, midterm: u32, finals: u32) -> u32 {
        // Calculate weighted average
        let final_grade = (prelim * 20 / 100) + (midterm * 30 / 100) + (finals * 50 / 100);
        
        // Create record
        let record = GradeRecord {
            student: student.clone(),
            prelim,
            midterm,
            finals,
            final_grade,
        };

        // Store in contract storage
        env.storage().instance().set(&StorageKey::GradeData, &record);
        
        // Emit an event for transparency
        log!(&env, "Grade Computed: Student {}, Final Grade: {}", student, final_grade);
        env.events().publish(
            (Symbol::new(&env, "grade_computed"), student),
            final_grade
        );

        final_grade
    }

    /// Verify and retrieve the stored grade
    pub fn verify_grade(env: Env) -> u32 {
        let record: GradeRecord = env.storage().instance()
            .get(&StorageKey::GradeData)
            .unwrap_or_else(|| panic!("No grades found"));
        
        record.final_grade
    }
  }
