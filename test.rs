#![cfg(test)]
use super::*;
use soroban_sdk::{Env, Address};

#[test]
fn test_happy_path() {
    let env = Env::default();
    let student = Address::generate(&env);
    
    // Input scores
    let result = GradeChain::compute_grades(env.clone(), student, 85, 88, 90);
    
    // Expected calculation: (85*0.2) + (88*0.3) + (90*0.5) = 17 + 26.4 + 45 = 88.4 -> 88
    assert_eq!(result, 88);
}

#[test]
#[should_panic(expected = "No grades found")]
fn test_edge_case_no_data() {
    let env = Env::default();
    
    // Trying to verify when no grades have been entered
    GradeChain::verify_grade(env);
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let student = Address::generate(&env);
    
    // First compute
    GradeChain::compute_grades(env.clone(), student.clone(), 90, 95, 98);
    
    // Then verify storage
    let verified = GradeChain::verify_grade(env.clone());
    
    // Expected: 18 + 28.5 + 49 = 95.5 -> 95
    assert_eq!(verified, 95);
}
