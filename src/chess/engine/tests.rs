#![cfg(test)]

use super::*;

mod startpos {
    use super::*;

    #[test]
    fn depth_1() {
        let engine = Engine::new();
        assert_eq!(20, engine.unit_perft(1));
    }

    #[test]
    fn depth_2() {
        let engine = Engine::new();
        assert_eq!(400, engine.unit_perft(2));
    }

    #[test]
    fn depth_3() {
        let engine = Engine::new();
        assert_eq!(8_902, engine.unit_perft(3));
    }

    #[test]
    fn depth_4() {
        // run cargo test -r -- --nocapture to capture the test output and the accurate perft benchmark (because it does not waste time with iterative deepening on lower depths)
        let engine = Engine::new();

        let start = std::time::Instant::now();
        let positions = engine.unit_perft(4);
        let duration = start.elapsed();

        let nodes_per_second = positions as f64 / duration.as_secs_f64();

        assert_eq!(197_281, positions);
        println!(
            "{positions} positions at depth 4 (startpos) generated in {:.3} seconds, ({:.0} nodes per second)",
            duration.as_secs_f64(),
            nodes_per_second
        );
    }

    #[test]
    fn depth_5() {
        // if this passes, all previous depths have passed
        let engine = Engine::new();
        assert_eq!(4_865_609, engine.unit_perft(5));
    }

    #[test]
    #[ignore = "to slow to test yet, and i32 overflow"]
    fn depth_6() {
        // if this passes, all previous depths have passed
        let _engine = Engine::new();
        // assert_eq!(8_031_647_685, engine.unit_perft(6));
    }
}

mod kiwipete {
    use super::*;

    #[test]
    fn depth_1() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
            "w",
            "KQkq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(48, engine.unit_perft(1));
    }

    #[test]
    fn depth_2() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
            "w",
            "KQkq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(2_039, engine.unit_perft(2));
    }

    #[test]
    fn depth_3() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
            "w",
            "KQkq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(97_862, engine.unit_perft(3));
    }

    #[test]
    fn depth_4() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
            "w",
            "KQkq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(4_085_603, engine.unit_perft(4));
    }

    #[test]
    #[ignore = "to slow to test yet"]
    fn depth_5() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
            "w",
            "KQkq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(193_690_690, engine.unit_perft(5));
    }

    #[test]
    #[ignore = "to slow to test yet, and i32 overflow"]
    fn depth_6() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
            "w",
            "KQkq",
            "-",
            "0",
            "1",
        ]);
        // assert_eq!(8_031_647_685, engine.unit_perft(6));
    }
}

mod pos_3 {
    // position 3 from CPW perft results
    use super::*;

    #[test]
    fn depth_1() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8",
            "w",
            "-",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(14, engine.unit_perft(1));
    }

    #[test]
    fn depth_2() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8",
            "w",
            "-",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(191, engine.unit_perft(2));
    }

    #[test]
    fn depth_3() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8",
            "w",
            "-",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(2812, engine.unit_perft(3));
    }

    #[test]
    fn depth_4() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8",
            "w",
            "-",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(43_238, engine.unit_perft(4));
    }

    #[test]
    fn depth_5() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8",
            "w",
            "-",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(674_624, engine.unit_perft(5));
    }

    #[test]
    fn depth_6() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8",
            "w",
            "-",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(11_030_083, engine.unit_perft(6));
    }
}

mod pos_4 {
    use super::*;

    #[test]
    fn depth_1() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
            "w",
            "kq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(6, engine.unit_perft(1));
    }

    #[test]
    fn depth_2() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
            "w",
            "kq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(264, engine.unit_perft(2));
    }

    #[test]
    fn depth_3() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
            "w",
            "kq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(9_467, engine.unit_perft(3));
    }

    #[test]
    fn depth_4() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
            "w",
            "kq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(422_333, engine.unit_perft(4));
    }

    #[test]
    #[ignore = "to slow, but already tested and passed"]
    fn depth_5() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
            "w",
            "kq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(15_833_292, engine.unit_perft(5));
    }

    #[test]
    #[ignore = "to slow to test yet"]
    fn depth_6() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1",
            "w",
            "kq",
            "-",
            "0",
            "1",
        ]);
        assert_eq!(706_045_033, engine.unit_perft(6));
    }
}

mod pos_5 {
    use super::*;

    #[test]
    fn depth_1() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
            "w",
            "KQ",
            "-",
            "1",
            "8",
        ]);
        assert_eq!(44, engine.unit_perft(1));
    }

    #[test]
    fn depth_2() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
            "w",
            "KQ",
            "-",
            "1",
            "8",
        ]);
        assert_eq!(1_486, engine.unit_perft(2));
    }

    #[test]
    fn depth_3() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
            "w",
            "KQ",
            "-",
            "1",
            "8",
        ]);
        assert_eq!(62_379, engine.unit_perft(3));
    }

    #[test]
    fn depth_4() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
            "w",
            "KQ",
            "-",
            "1",
            "8",
        ]);
        assert_eq!(2_103_487, engine.unit_perft(4));
    }

    #[test]
    #[ignore = "to slow, but already passed (took 5 min to run 89 milion positions)"]
    fn depth_5() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R",
            "w",
            "KQ",
            "-",
            "1",
            "8",
        ]);
        assert_eq!(89_941_194, engine.unit_perft(5));
    }
}

mod pos_6 {
    use super::*;

    #[test]
    fn depth_1() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1",
            "w",
            "-",
            "-",
            "0",
            "10",
        ]);
        assert_eq!(46, engine.unit_perft(1));
    }

    #[test]
    fn depth_2() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1",
            "w",
            "-",
            "-",
            "0",
            "10",
        ]);
        assert_eq!(2_079, engine.unit_perft(2));
    }

    #[test]
    fn depth_3() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1",
            "w",
            "-",
            "-",
            "0",
            "10",
        ]);
        assert_eq!(89_890, engine.unit_perft(3));
    }

    #[test]
    fn depth_4() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1",
            "w",
            "-",
            "-",
            "0",
            "10",
        ]);
        assert_eq!(3_894_594, engine.unit_perft(4));
    }

    #[test]
    #[ignore = "to slow to test yet"]
    fn depth_5() {
        let mut engine = Engine::new();
        engine.load_from_fen(vec![
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1",
            "w",
            "-",
            "-",
            "0",
            "10",
        ]);
        assert_eq!(164_075_551, engine.unit_perft(5));
    }
}

mod etheral_test_suite {
    use super::*;

    mod pos_1 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec![
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
                "w",
                "KQkq",
                "-",
                "0",
                "1",
            ]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(20, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(400, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(8_902, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(197_281, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_865_609, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(119_060_324, engine().unit_perft(6));
        }
    }

    mod pos_2 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec![
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
                "w",
                "KQkq",
                "-",
                "0",
                "1",
            ]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(48, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(2_039, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(97_862, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(4_085_603, engine().unit_perft(4));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_5() {
            assert_eq!(193_690_690, engine().unit_perft(5));
        }
    }

    mod pos_3 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k3/8/8/8/8/8/8/4K2R", "w", "K", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(15, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(66, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_197, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(7_059, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(133_987, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(764_643, engine().unit_perft(6));
        }
    }

    mod pos_4 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k3/8/8/8/8/8/8/R3K3", "w", "Q", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(16, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(71, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_287, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(7_626, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(145_232, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(846_648, engine().unit_perft(6));
        }
    }

    mod pos_5 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k2r/8/8/8/8/8/8/4K3", "w", "k", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(75, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(459, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(8_290, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(47_635, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(899_442, engine().unit_perft(6));
        }
    }

    mod pos_6 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k3/8/8/8/8/8/8/4K3", "w", "q", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(80, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(493, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(8_897, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(52_710, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(1_001_523, engine().unit_perft(6));
        }
    }

    mod pos_7 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k3/8/8/8/8/8/8/R3K2R", "w", "KQ", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(26, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(112, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(3_189, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(17_945, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(532_933, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(2_788_982, engine().unit_perft(6));
        }
    }

    mod pos_8 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/4K3", "w", "kq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(130, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(782, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(22_180, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(118_882, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(3_517_770, engine().unit_perft(6));
        }
    }

    mod pos_9 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/8/6k1/4K2R", "w", "K", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(12, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(38, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(564, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(2_219, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(37_735, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(185_867, engine().unit_perft(6));
        }
    }

    mod pos_10 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/8/1k6/R3K3", "w", "Q", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(15, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(65, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_018, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(4_573, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(80_619, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(413_018, engine().unit_perft(6));
        }
    }

    mod pos_11 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k2r/6K1/8/8/8/8/8/8", "w", "k", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(32, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(134, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(2_073, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(10_485, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(179_869, engine().unit_perft(6));
        }
    }

    mod pos_12 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k3/1K6/8/8/8/8/8/8", "w", "q", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(49, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(243, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_991, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(20_780, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(367_724, engine().unit_perft(6));
        }
    }

    mod pos_13 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/R3K2R", "w", "KQkq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(26, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(568, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_744, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(314_346, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_594_526, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(179_862_938, engine().unit_perft(6));
        }
    }

    mod pos_14 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/1R2K2R", "w", "Kkq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(567, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(14_095, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(328_965, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(8_153_719, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(195_629_489, engine().unit_perft(6));
        }
    }

    mod pos_15 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/2R1K2R", "w", "Kkq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(548, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_502, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(312_835, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_736_373, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(184_411_439, engine().unit_perft(6));
        }
    }

    mod pos_16 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/R3K1R1", "w", "Qkq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(547, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_579, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(316_214, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_878_456, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(189_224_276, engine().unit_perft(6));
        }
    }

    mod pos_17 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["1r2k2r/8/8/8/8/8/8/R3K2R", "w", "KQk", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(26, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(583, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(14_252, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(334_705, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(8_198_901, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(198_328_929, engine().unit_perft(6));
        }
    }

    mod pos_18 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["2r1k2r/8/8/8/8/8/8/R3K2R", "w", "KQk", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(560, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_592, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(317_324, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_710_115, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(185_959_088, engine().unit_perft(6));
        }
    }

    mod pos_19 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k1r1/8/8/8/8/8/8/R3K2R", "w", "KQq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(560, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_607, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(320_792, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_848_606, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(190_755_813, engine().unit_perft(6));
        }
    }

    mod pos_20 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k3/8/8/8/8/8/8/4K2R", "b", "K", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(75, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(459, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(8_290, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(47_635, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(899_442, engine().unit_perft(6));
        }
    }

    mod pos_21 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k3/8/8/8/8/8/8/R3K3", "b", "Q", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(80, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(493, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(8_897, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(52_710, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(1_001_523, engine().unit_perft(6));
        }
    }

    mod pos_22 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k2r/8/8/8/8/8/8/4K3", "b", "k", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(15, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(66, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_197, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(7_059, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(133_987, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(764_643, engine().unit_perft(6));
        }
    }

    mod pos_23 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k3/8/8/8/8/8/8/4K3", "b", "q", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(16, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(71, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_287, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(7_626, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(145_232, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(846_648, engine().unit_perft(6));
        }
    }

    mod pos_24 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k3/8/8/8/8/8/8/R3K2R", "b", "KQ", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(130, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(782, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(22_180, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(118_882, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(3_517_770, engine().unit_perft(6));
        }
    }

    mod pos_25 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/4K3", "b", "kq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(26, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(112, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(3_189, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(17_945, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(532_933, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(2_788_982, engine().unit_perft(6));
        }
    }

    mod pos_26 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/8/6k1/4K2R", "b", "K", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(32, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(134, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(2_073, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(10_485, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(179_869, engine().unit_perft(6));
        }
    }

    mod pos_27 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/8/1k6/R3K3", "b", "Q", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(49, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(243, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_991, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(20_780, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(367_724, engine().unit_perft(6));
        }
    }

    mod pos_28 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k2r/6K1/8/8/8/8/8/8", "b", "k", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(12, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(38, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(564, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(2_219, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(37_735, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(185_867, engine().unit_perft(6));
        }
    }

    mod pos_29 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k3/1K6/8/8/8/8/8/8", "b", "q", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(15, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(65, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_018, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(4_573, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(80_619, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(413_018, engine().unit_perft(6));
        }
    }

    mod pos_30 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/R3K2R", "b", "KQkq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(26, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(568, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_744, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(314_346, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_594_526, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(179_862_938, engine().unit_perft(6));
        }
    }

    mod pos_31 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/1R2K2R", "b", "Kkq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(26, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(583, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(14_252, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(334_705, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(8_198_901, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(198_328_929, engine().unit_perft(6));
        }
    }

    mod pos_32 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/2R1K2R", "b", "Kkq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(560, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_592, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(317_324, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_710_115, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(185_959_088, engine().unit_perft(6));
        }
    }

    mod pos_33 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k2r/8/8/8/8/8/8/R3K1R1", "b", "Qkq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(560, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_607, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(320_792, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_848_606, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(190_755_813, engine().unit_perft(6));
        }
    }

    mod pos_34 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["1r2k2r/8/8/8/8/8/8/R3K2R", "b", "KQk", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(567, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(14_095, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(328_965, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(8_153_719, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(195_629_489, engine().unit_perft(6));
        }
    }

    mod pos_35 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["2r1k2r/8/8/8/8/8/8/R3K2R", "b", "KQk", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(548, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_502, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(312_835, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_736_373, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(184_411_439, engine().unit_perft(6));
        }
    }

    mod pos_36 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["r3k1r1/8/8/8/8/8/8/R3K2R", "b", "KQq", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(25, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(547, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(13_579, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(316_214, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_878_456, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(189_224_276, engine().unit_perft(6));
        }
    }

    mod pos_37 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/1n4N1/2k5/8/8/5K2/1N4n1/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(14, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(195, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(2_760, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(38_675, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(570_726, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(8_107_539, engine().unit_perft(6));
        }
    }

    mod pos_38 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/1k6/8/5N2/8/4n3/8/2K5", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(11, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(156, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_636, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(20_534, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(223_507, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(2_594_412, engine().unit_perft(6));
        }
    }

    mod pos_39 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/4k3/3Nn3/3nN3/4K3/8/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(19, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(289, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(4_442, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(73_584, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_198_299, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(19_870_403, engine().unit_perft(6));
        }
    }

    mod pos_40 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["K7/8/2n5/1n6/8/8/8/k6N", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(51, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(345, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(5_301, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(38_348, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(588_695, engine().unit_perft(6));
        }
    }

    mod pos_41 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/2N5/1N6/8/8/8/K6n", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(17, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(54, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(835, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(5_910, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(92_250, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(688_780, engine().unit_perft(6));
        }
    }

    mod pos_42 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/1n4N1/2k5/8/8/5K2/1N4n1/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(15, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(193, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(2_816, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(40_039, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(582_642, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(8_503_277, engine().unit_perft(6));
        }
    }

    mod pos_43 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/1k6/8/5N2/8/4n3/8/2K5", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(16, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(180, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(2_290, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(24_640, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(288_141, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(3_147_566, engine().unit_perft(6));
        }
    }

    mod pos_44 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/3K4/3Nn3/3nN3/4k3/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(68, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_118, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(16_199, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(281_190, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(4_405_103, engine().unit_perft(6));
        }
    }

    mod pos_45 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["K7/8/2n5/1n6/8/8/8/k6N", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(17, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(54, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(835, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(5_910, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(92_250, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(688_780, engine().unit_perft(6));
        }
    }

    mod pos_46 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/2N5/1N6/8/8/8/K6n", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(51, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(345, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(5_301, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(38_348, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(588_695, engine().unit_perft(6));
        }
    }

    mod pos_47 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["B6b/8/8/8/2K5/4k3/8/b6B", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(17, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(278, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(4_607, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(76_778, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_320_507, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(22_823_890, engine().unit_perft(6));
        }
    }

    mod pos_48 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/1B6/7b/7k/8/2B1b3/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(21, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(316, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(5_744, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(93_338, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_713_368, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(28_861_171, engine().unit_perft(6));
        }
    }

    mod pos_49 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/B7/1B6/1B6/8/8/8/K6b", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(21, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(144, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(3_242, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(32_955, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(787_524, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(7_881_673, engine().unit_perft(6));
        }
    }

    mod pos_50 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["K7/b7/1b6/1b6/8/8/8/k6B", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(7, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(143, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_416, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(31_787, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(310_862, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(7_382_896, engine().unit_perft(6));
        }
    }

    mod pos_51 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["B6b/8/8/8/2K5/5k2/8/b6B", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(6, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(106, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_829, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(31_151, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(530_585, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(9_250_746, engine().unit_perft(6));
        }
    }

    mod pos_52 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/1B6/7b/7k/8/2B1b3/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(17, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(309, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(5_133, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(93_603, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_591_064, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(29_027_891, engine().unit_perft(6));
        }
    }

    mod pos_53 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/B7/1B6/1B6/8/8/8/K6b", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(7, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(143, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_416, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(31_787, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(310_862, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(7_382_896, engine().unit_perft(6));
        }
    }

    mod pos_54 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["K7/b7/1b6/1b6/8/8/8/k6B", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(21, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(144, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(3_242, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(32_955, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(787_524, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(7_881_673, engine().unit_perft(6));
        }
    }

    mod pos_55 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/RR6/8/8/8/8/rr6/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(19, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(275, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(5_300, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(104_342, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(2_161_211, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(44_956_585, engine().unit_perft(6));
        }
    }

    mod pos_56 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["R6r/8/8/2K5/5k2/8/8/r6R", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(36, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(1_027, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(29_215, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(771_461, engine().unit_perft(4));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_5() {
            assert_eq!(20_506_480, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(525_169_084, engine().unit_perft(6));
        }
    }

    mod pos_57 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/RR6/8/8/8/8/rr6/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(19, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(275, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(5_300, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(104_342, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(2_161_211, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(44_956_585, engine().unit_perft(6));
        }
    }

    mod pos_58 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["R6r/8/8/2K5/5k2/8/8/r6R", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(36, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(1_027, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(29_227, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(771_368, engine().unit_perft(4));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_5() {
            assert_eq!(20_521_342, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(524_966_748, engine().unit_perft(6));
        }
    }

    mod pos_59 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["6kq/8/8/8/8/8/8/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(2, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(36, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(143, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(14_893, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(391_507, engine().unit_perft(6));
        }
    }

    mod pos_60 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["6KQ/8/8/8/8/8/8/7k", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(2, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(36, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(143, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(14_893, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(391_507, engine().unit_perft(6));
        }
    }

    mod pos_61 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["K7/8/8/3Q4/4q3/8/8/7k", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(6, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(35, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(495, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(8_349, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(166_741, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(3_370_175, engine().unit_perft(6));
        }
    }

    mod pos_62 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["6qk/8/8/8/8/8/8/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(22, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(43, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(1_015, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(4_167, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(105_749, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(419_369, engine().unit_perft(6));
        }
    }

    mod pos_63 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["6KQ/8/8/8/8/8/8/7k", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(2, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(36, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(143, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(14_893, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(391_507, engine().unit_perft(6));
        }
    }

    mod pos_64 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["K7/8/8/3Q4/4q3/8/8/7k", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(6, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(35, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(495, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(8_349, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(166_741, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(3_370_175, engine().unit_perft(6));
        }
    }

    mod pos_65 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/K7/P7/k7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(7, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(43, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(199, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_347, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(6_249, engine().unit_perft(6));
        }
    }

    mod pos_66 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/7K/7P/7k", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(7, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(43, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(199, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_347, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(6_249, engine().unit_perft(6));
        }
    }

    mod pos_67 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["K7/p7/k7/8/8/8/8/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(1, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(3, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(12, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(80, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(342, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(2_343, engine().unit_perft(6));
        }
    }

    mod pos_68 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7K/7p/7k/8/8/8/8/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(1, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(3, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(12, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(80, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(342, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(2_343, engine().unit_perft(6));
        }
    }

    mod pos_69 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/2k1p3/3pP3/3P2K1/8/8/8/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(7, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(35, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(210, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(1_091, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_028, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(34_834, engine().unit_perft(6));
        }
    }

    mod pos_70 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/K7/P7/k7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(1, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(3, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(12, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(80, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(342, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(2_343, engine().unit_perft(6));
        }
    }

    mod pos_71 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/7K/7P/7k", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(1, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(3, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(12, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(80, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(342, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(2_343, engine().unit_perft(6));
        }
    }

    mod pos_72 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["K7/p7/k7/8/8/8/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(7, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(43, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(199, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_347, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(6_249, engine().unit_perft(6));
        }
    }

    mod pos_73 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7K/7p/7k/8/8/8/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(7, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(43, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(199, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_347, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(6_249, engine().unit_perft(6));
        }
    }

    mod pos_74 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/2k1p3/3pP3/3P2K1/8/8/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(35, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(182, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(1_091, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(5_408, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(34_822, engine().unit_perft(6));
        }
    }

    mod pos_75 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/8/8/8/4k3/4P3/4K3", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(2, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(8, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(44, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(282, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_814, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(11_848, engine().unit_perft(6));
        }
    }

    mod pos_76 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["4k3/4p3/4K3/8/8/8/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(2, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(8, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(44, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(282, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_814, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(11_848, engine().unit_perft(6));
        }
    }

    mod pos_77 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/7k/7p/7P/7K/8/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(9, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(57, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(360, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_969, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(10_724, engine().unit_perft(6));
        }
    }

    mod pos_78 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/k7/p7/P7/K7/8/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(9, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(57, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(360, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_969, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(10_724, engine().unit_perft(6));
        }
    }

    mod pos_79 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/3k4/3p4/3P4/3K4/8/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(25, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(180, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(1_294, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(8_296, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(53_138, engine().unit_perft(6));
        }
    }

    mod pos_80 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/3k4/3p4/8/3P4/3K4/8/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(8, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(61, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(483, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_213, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(23_599, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(157_093, engine().unit_perft(6));
        }
    }

    mod pos_81 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/3k4/3p4/8/3P4/3K4/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(8, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(61, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(411, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_213, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(21_637, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(158_065, engine().unit_perft(6));
        }
    }

    mod pos_82 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/3p4/8/3P4/8/8/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(15, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(90, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(534, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(3_450, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(20_960, engine().unit_perft(6));
        }
    }

    mod pos_83 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/7k/7p/7P/7K/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(9, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(57, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(360, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_969, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(10_724, engine().unit_perft(6));
        }
    }

    mod pos_84 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/k7/p7/P7/K7/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(9, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(57, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(360, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_969, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(10_724, engine().unit_perft(6));
        }
    }

    mod pos_85 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/3k4/3p4/3P4/3K4/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(25, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(180, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(1_294, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(8_296, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(53_138, engine().unit_perft(6));
        }
    }

    mod pos_86 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/3k4/3p4/8/3P4/3K4/8/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(8, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(61, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(411, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_213, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(21_637, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(158_065, engine().unit_perft(6));
        }
    }

    mod pos_87 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/8/3k4/3p4/8/3P4/3K4/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(8, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(61, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(483, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(3_213, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(23_599, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(157_093, engine().unit_perft(6));
        }
    }

    mod pos_88 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/3p4/8/3P4/8/8/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(15, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(89, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(537, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(3_309, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(21_104, engine().unit_perft(6));
        }
    }

    mod pos_89 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/3p4/8/8/3P4/8/8/K7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(19, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(117, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(720, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_661, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(32_191, engine().unit_perft(6));
        }
    }

    mod pos_90 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/8/3p4/8/8/3P4/K7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(19, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(116, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(716, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_786, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(30_980, engine().unit_perft(6));
        }
    }

    mod pos_91 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/8/7p/6P1/8/8/K7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(22, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(139, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(877, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(6_112, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(41_874, engine().unit_perft(6));
        }
    }

    mod pos_92 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/7p/8/8/6P1/8/K7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_354, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(29_679, engine().unit_perft(6));
        }
    }

    mod pos_93 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/8/6p1/7P/8/8/K7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(22, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(139, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(877, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(6_112, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(41_874, engine().unit_perft(6));
        }
    }

    mod pos_94 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/6p1/8/8/7P/8/K7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_354, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(29_679, engine().unit_perft(6));
        }
    }

    mod pos_95 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/8/3p4/4p3/8/8/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(3, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(15, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(84, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(573, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(3_013, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(22_886, engine().unit_perft(6));
        }
    }

    mod pos_96 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/3p4/8/8/4P3/8/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_271, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(28_662, engine().unit_perft(6));
        }
    }

    mod pos_97 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/3p4/8/8/3P4/8/8/K7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(19, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(117, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(720, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(5_014, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(32_167, engine().unit_perft(6));
        }
    }

    mod pos_98 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/8/3p4/8/8/3P4/K7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(19, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(117, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(712, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_658, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(30_749, engine().unit_perft(6));
        }
    }

    mod pos_99 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/8/7p/6P1/8/8/K7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(22, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(139, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(877, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(6_112, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(41_874, engine().unit_perft(6));
        }
    }

    mod pos_100 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/7p/8/8/6P1/8/K7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_354, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(29_679, engine().unit_perft(6));
        }
    }

    mod pos_101 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/8/6p1/7P/8/8/K7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(22, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(139, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(877, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(6_112, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(41_874, engine().unit_perft(6));
        }
    }

    mod pos_102 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/6p1/8/8/7P/8/K7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_354, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(29_679, engine().unit_perft(6));
        }
    }

    mod pos_103 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/8/3p4/4p3/8/8/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(15, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(102, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(569, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_337, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(22_579, engine().unit_perft(6));
        }
    }

    mod pos_104 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/8/3p4/8/8/4P3/8/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_271, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(28_662, engine().unit_perft(6));
        }
    }

    mod pos_105 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/8/p7/1P6/8/8/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(22, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(139, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(877, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(6_112, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(41_874, engine().unit_perft(6));
        }
    }

    mod pos_106 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/p7/8/8/1P6/8/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_354, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(29_679, engine().unit_perft(6));
        }
    }

    mod pos_107 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/8/1p6/P7/8/8/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(22, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(139, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(877, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(6_112, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(41_874, engine().unit_perft(6));
        }
    }

    mod pos_108 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/1p6/8/8/P7/8/7K", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_354, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(29_679, engine().unit_perft(6));
        }
    }

    mod pos_109 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/7p/8/8/8/8/6P1/K7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(25, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(161, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(1_035, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_574, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(55_338, engine().unit_perft(6));
        }
    }

    mod pos_110 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/6p1/8/8/8/8/7P/K7", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(25, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(161, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(1_035, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_574, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(55_338, engine().unit_perft(6));
        }
    }

    mod pos_111 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["3k4/3pp3/8/8/8/8/3PP3/3K4", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(7, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(49, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(378, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(2_902, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(24_122, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(199_002, engine().unit_perft(6));
        }
    }

    mod pos_112 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/8/p7/1P6/8/8/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(22, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(139, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(877, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(6_112, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(41_874, engine().unit_perft(6));
        }
    }

    mod pos_113 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/p7/8/8/1P6/8/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_354, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(29_679, engine().unit_perft(6));
        }
    }

    mod pos_114 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/8/1p6/P7/8/8/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(22, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(139, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(877, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(6_112, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(41_874, engine().unit_perft(6));
        }
    }

    mod pos_115 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["7k/8/1p6/8/8/P7/8/7K", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(4, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(16, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(101, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(637, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(4_354, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(29_679, engine().unit_perft(6));
        }
    }

    mod pos_116 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/7p/8/8/8/8/6P1/K7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(25, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(161, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(1_035, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_574, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(55_338, engine().unit_perft(6));
        }
    }

    mod pos_117 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["k7/6p1/8/8/8/8/7P/K7", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(5, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(25, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(161, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(1_035, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(7_574, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(55_338, engine().unit_perft(6));
        }
    }

    mod pos_118 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["3k4/3pp3/8/8/8/8/3PP3/3K4", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(7, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(49, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(378, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(2_902, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(24_122, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(199_002, engine().unit_perft(6));
        }
    }

    mod pos_119 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/Pk6/8/8/8/8/6Kp/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(11, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(97, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(887, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(8_048, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(90_606, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(1_030_499, engine().unit_perft(6));
        }
    }

    mod pos_120 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["n1n5/1Pk5/8/8/8/8/5Kp1/5N1N", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(24, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(421, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(7_421, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(124_608, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(2_193_768, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(37_665_329, engine().unit_perft(6));
        }
    }

    mod pos_121 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/PPPk4/8/8/8/8/4Kppp/8", "w", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(18, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(270, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(4_699, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(79_355, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_533_145, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(28_859_283, engine().unit_perft(6));
        }
    }

    mod pos_122 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec![
                "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N",
                "w",
                "-",
                "-",
                "0",
                "1",
            ]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(24, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(496, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(9_483, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(182_838, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(3_605_103, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(71_179_139, engine().unit_perft(6));
        }
    }

    mod pos_123 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/Pk6/8/8/8/8/6Kp/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(11, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(97, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(887, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(8_048, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(90_606, engine().unit_perft(5));
        }
        #[test]
        fn depth_6() {
            assert_eq!(1_030_499, engine().unit_perft(6));
        }
    }

    mod pos_124 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["n1n5/1Pk5/8/8/8/8/5Kp1/5N1N", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(24, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(421, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(7_421, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(124_608, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(2_193_768, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(37_665_329, engine().unit_perft(6));
        }
    }

    mod pos_125 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec!["8/PPPk4/8/8/8/8/4Kppp/8", "b", "-", "-", "0", "1"]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(18, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(270, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(4_699, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(79_355, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(1_533_145, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(28_859_283, engine().unit_perft(6));
        }
    }

    mod pos_126 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec![
                "n1n5/PPPk4/8/8/8/8/4Kppp/5N1N",
                "b",
                "-",
                "-",
                "0",
                "1",
            ]);
            e
        }

        #[test]
        fn depth_1() {
            assert_eq!(24, engine().unit_perft(1));
        }
        #[test]
        fn depth_2() {
            assert_eq!(496, engine().unit_perft(2));
        }
        #[test]
        fn depth_3() {
            assert_eq!(9_483, engine().unit_perft(3));
        }
        #[test]
        fn depth_4() {
            assert_eq!(182_838, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(3_605_103, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(71_179_139, engine().unit_perft(6));
        }
    }

    mod pos_127 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec![
                "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8",
                "w",
                "-",
                "-",
                "0",
                "1",
            ]);
            e
        }

        #[test]
        fn depth_4() {
            assert_eq!(43_238, engine().unit_perft(4));
        }
        #[test]
        fn depth_5() {
            assert_eq!(674_624, engine().unit_perft(5));
        }
        #[test]
        #[ignore = "too slow to test"]
        fn depth_6() {
            assert_eq!(11_030_083, engine().unit_perft(6));
        }
    }

    mod pos_128 {
        use super::super::*;

        fn engine() -> Engine {
            let mut e = Engine::new();
            e.load_from_fen(vec![
                "rnbqkb1r/ppppp1pp/7n/4Pp2/8/8/PPPP1PPP/RNBQKBNR",
                "w",
                "KQkq",
                "f6",
                "0",
                "3",
            ]);
            e
        }

        #[test]
        #[ignore = "too slow to test"]
        fn depth_5() {
            assert_eq!(11_139_762, engine().unit_perft(5));
        }
    }
}
