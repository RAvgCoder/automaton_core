use std::fmt::Debug;

use crate::automaton_graph::{SimulationError, Symbol, Transition};
use crate::dfa::DFA;

impl DFA {
    pub fn simulate(&self, simulating_string: &str) -> Result<bool, SimulationError> {
        let mut curr_state = self.automaton_graph.get_start_state();

        for (idx, c) in simulating_string.chars().enumerate() {
            let transition = Transition::find_transition_by_symbol(curr_state.get_transitions(), c);

            if transition.len() == 0 { return Ok(false); }
            if transition.len() != 1 {
                // DFAs cannot have more than one transition on a symbol
                let error_message = format!(
                    "Invariant broken for current DFA. Multiple transitions found for character {} at index {} for symbols {:?} on state {}",
                    c,
                    idx,
                    curr_state
                        .get_transitions()
                        .iter()
                        .map(|t| t.transition_on())
                        .collect::<Vec<Symbol>>(),
                    &curr_state.alt_id
                );
                return Err(SimulationError::MultipleTransitionsFound(error_message));
            }

            curr_state = transition.first().unwrap().to();
        }

        Ok(curr_state.is_accept_state)
    }
}

#[cfg(test)]
mod simulation_tests {
    use crate::dfa::DFA;
    const TEST_STRINGS: [&str; 5] = [
        r#"{"type":"DFA","dfa":{"transitions":{"start":{"0":"start","1":"s0"},"s0":{"0":"s0","1":"s1"},"s1":{"0":"s1","1":"start"}},"startState":"start","acceptStates":["start"]},"states":{"start":{"isAccept":true},"s0":{"top":238.333333,"left":212.333333,"displayId":"s0"},"s1":{"top":271.66667650585936,"left":429.66670702343754,"displayId":"s1"}},"transitions":[{"stateA":"start","label":"0","stateB":"start"},{"stateA":"start","label":"1","stateB":"s0"},{"stateA":"s0","label":"0","stateB":"s0"},{"stateA":"s0","label":"1","stateB":"s1"},{"stateA":"s1","label":"0","stateB":"s1"},{"stateA":"s1","label":"1","stateB":"start"}],"bulkTests":{"accept":"\n000\n111\n010101\n111111111111111\n000000000011001\n10000000100000010111\n1010101010101010001","reject":"00001000000010000\n0100\n1\n1111\n0101101\n11111111111111\n00000100000001\n100000100100000010111\n10101011010101010001"}}"#,
        r#"{"type":"DFA","dfa":{"transitions":{"start":{"0":"s0","1":"s1"},"s0":{"0":"s0","1":"s2"},"s2":{"0":"s0","1":"s2"},"s1":{"0":"s3","1":"s1"},"s3":{"0":"s3","1":"s1"}},"startState":"start","acceptStates":["s0","start","s1"]},"states":{"start":{"isAccept":true},"s0":{"isAccept":true,"top":201.0000047529297,"left":195.00002001171876,"displayId":"0_accept"},"s1":{"isAccept":true,"top":362.66667650585936,"left":182.66667650585939,"displayId":"1_accept"},"s2":{"top":110.66667650585939,"left":416.66670702343754,"displayId":"1_DEAD"},"s3":{"top":449.66667650585936,"left":391.66670702343754,"displayId":"0_DEAD"}},"transitions":[{"stateA":"start","label":"0","stateB":"s0"},{"stateA":"start","label":"1","stateB":"s1"},{"stateA":"s0","label":"0","stateB":"s0"},{"stateA":"s0","label":"1","stateB":"s2"},{"stateA":"s2","label":"0","stateB":"s0"},{"stateA":"s2","label":"1","stateB":"s2"},{"stateA":"s1","label":"0","stateB":"s3"},{"stateA":"s1","label":"1","stateB":"s1"},{"stateA":"s3","label":"0","stateB":"s3"},{"stateA":"s3","label":"1","stateB":"s1"}],"bulkTests":{"accept":"101010101\n0010101000000010\n1010101011011111111111\n11111111111111111\n000000000000000\n100000001111111100001\n","reject":"1111100000000000000\n1111111111111110101010\n0000000000111111111\n0000000000111111100001\n101010"}}"#,
        r#"{"type":"DFA","dfa":{"transitions":{"start":{"C":"s0","I":"start","S":"start"},"s0":{"S":"s1","C":"start","I":"start"},"s1":{"C":"s2","I":"start","S":"start"},"s2":{"I":"s3","C":"start","S":"start"},"s3":{"I":"start","C":"start","S":"start"}},"startState":"start","acceptStates":["s3"]},"states":{"start":{},"s0":{"top":456.333333,"left":291.33333300000004,"displayId":"q0"},"s1":{"top":304.333333,"left":387.33333300000004,"displayId":"q1"},"s2":{"top":134.0000047529297,"left":398.0000200117188,"displayId":"q2"},"s3":{"isAccept":true,"top":64.0000047529297,"left":104.00002001171876,"displayId":"q3"}},"transitions":[{"stateA":"start","label":"C","stateB":"s0"},{"stateA":"start","label":"I","stateB":"start"},{"stateA":"start","label":"S","stateB":"start"},{"stateA":"s0","label":"S","stateB":"s1"},{"stateA":"s0","label":"C","stateB":"start"},{"stateA":"s0","label":"I","stateB":"start"},{"stateA":"s1","label":"C","stateB":"s2"},{"stateA":"s1","label":"I","stateB":"start"},{"stateA":"s1","label":"S","stateB":"start"},{"stateA":"s2","label":"I","stateB":"s3"},{"stateA":"s2","label":"C","stateB":"start"},{"stateA":"s2","label":"S","stateB":"start"},{"stateA":"s3","label":"I","stateB":"start"},{"stateA":"s3","label":"C","stateB":"start"},{"stateA":"s3","label":"S","stateB":"start"}],"bulkTests":{"accept":"CSCI\nCSCISCICSCI\nSISISCSCI\nCSCCSCSCI","reject":"DDD\nCDSICI\nCSCIADSDS"}}"#,
        r#"{"type":"DFA","dfa":{"transitions":{"start":{"0":"s2","1":"s0"},"s0":{"0":"s2","1":"s1"},"s2":{"0":"s2","1":"s2"},"s1":{"0":"s3","1":"s1"},"s3":{"0":"s4","1":"s1"},"s4":{"0":"s4","1":"s1"}},"startState":"start","acceptStates":["s3","s1","s0","start","s2"]},"states":{"start":{"isAccept":true},"s2":{"isAccept":true,"top":199.333333,"left":210.333333,"displayId":"DEAD"},"s0":{"isAccept":true,"top":363.333333,"left":164.333333,"displayId":"s0"},"s1":{"isAccept":true,"top":366.00002001171873,"left":381.0000200117188,"displayId":"s1"},"s3":{"isAccept":true,"top":279.00002001171873,"left":553.0000200117188,"displayId":"s3"},"s4":{"top":362.00002001171873,"left":718.0000200117188,"displayId":"s4"}},"transitions":[{"stateA":"start","label":"0","stateB":"s2"},{"stateA":"start","label":"1","stateB":"s0"},{"stateA":"s0","label":"0","stateB":"s2"},{"stateA":"s0","label":"1","stateB":"s1"},{"stateA":"s2","label":"0","stateB":"s2"},{"stateA":"s2","label":"1","stateB":"s2"},{"stateA":"s1","label":"0","stateB":"s3"},{"stateA":"s1","label":"1","stateB":"s1"},{"stateA":"s3","label":"0","stateB":"s4"},{"stateA":"s3","label":"1","stateB":"s1"},{"stateA":"s4","label":"0","stateB":"s4"},{"stateA":"s4","label":"1","stateB":"s1"}],"bulkTests":{"accept":"\n1101\n01110\n001111110110100\n1010\n01101\n01010101\n0000101001010\n001011100\n111111111\n0000000001\n001111111111\n11010101010","reject":"110111000\n1100100\n1100000010000\n110111111100100\n110110100\n110101010100\n110101010100\n110101010100"}}"#,
        r#"{"type":"DFA","dfa":{"transitions":{"start":{"0":"s1","1":"s0"},"s0":{"0":"s2","1":"s3"},"s1":{"0":"s3","1":"s4"},"s4":{"0":"s4","1":"s5"},"s5":{"0":"s6","1":"s5"},"s6":{"0":"s4","1":"s5"},"s2":{"0":"s7","1":"s2"},"s7":{"0":"s7","1":"s8"},"s8":{"0":"s7","1":"s2"},"s3":{"0":"s3","1":"s3"}},"startState":"start","acceptStates":["s6","s8"]},"states":{"start":{},"s1":{"top":395.66667650585936,"left":125.66667650585939,"displayId":"s1"},"s0":{"top":145.0000047529297,"left":156.00002001171876,"displayId":"s0"},"s2":{"top":112.0000047529297,"left":325.0000200117188,"displayId":"s2"},"s3":{"top":246.333333,"left":300.33333300000004,"displayId":"DEAD"},"s4":{"top":460.333333,"left":293.33333300000004,"displayId":"s4"},"s5":{"top":419.00002001171873,"left":489.0000200117188,"displayId":"s5"},"s6":{"isAccept":true,"top":473.333333,"left":687.333333,"displayId":"s6"},"s7":{"top":63.333333,"left":512.333333,"displayId":"s7"},"s8":{"isAccept":true,"top":109.0000047529297,"left":687.0000200117188,"displayId":"s8"}},"transitions":[{"stateA":"start","label":"0","stateB":"s1"},{"stateA":"start","label":"1","stateB":"s0"},{"stateA":"s0","label":"0","stateB":"s2"},{"stateA":"s0","label":"1","stateB":"s3"},{"stateA":"s1","label":"0","stateB":"s3"},{"stateA":"s1","label":"1","stateB":"s4"},{"stateA":"s4","label":"0","stateB":"s4"},{"stateA":"s4","label":"1","stateB":"s5"},{"stateA":"s5","label":"0","stateB":"s6"},{"stateA":"s5","label":"1","stateB":"s5"},{"stateA":"s6","label":"0","stateB":"s4"},{"stateA":"s6","label":"1","stateB":"s5"},{"stateA":"s2","label":"0","stateB":"s7"},{"stateA":"s2","label":"1","stateB":"s2"},{"stateA":"s7","label":"0","stateB":"s7"},{"stateA":"s7","label":"1","stateB":"s8"},{"stateA":"s8","label":"0","stateB":"s7"},{"stateA":"s8","label":"1","stateB":"s2"},{"stateA":"s3","label":"0","stateB":"s3"},{"stateA":"s3","label":"1","stateB":"s3"}],"bulkTests":{"accept":"0110\n1001\n10000001\n011111110\n011010\n010101010\n101010101\n101010101","reject":"\n1010\n01101\n01010101\n0000101001010\n001011100\n111111111\n0000000001\n001111111111"}}"#,
    ];

    use crate::parser::Parser;

    #[test]
    fn test_simulation_should_accept() {
        for test_string in TEST_STRINGS {
            let automaton = Parser::parse(test_string);
            let dfa = DFA::new(automaton);
            for accepting_str in &dfa.automaton_graph.tests.accepting_strings {
                assert_eq!(dfa.simulate(accepting_str).expect(&format!("Simulation failed for string {}", accepting_str)), true);
            }
        }
    }

    #[test]
    fn test_simulation_should_reject() {
        for test_string in TEST_STRINGS {
            let automaton = Parser::parse(test_string);
            let dfa = DFA::new(automaton);
            for rejecting_str in &dfa.automaton_graph.tests.rejecting_strings {
                assert_eq!(dfa.simulate(rejecting_str).expect("Simulation Expected to fail gracefully without errors for rejecting strings"), false);
            }
        }
    }
}
