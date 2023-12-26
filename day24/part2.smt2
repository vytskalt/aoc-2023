(declare-const x Real)
(declare-const y Real)
(declare-const z Real)
(declare-const dx Real)
(declare-const dy Real)
(declare-const dz Real)

(declare-const a Real)
(declare-const b Real)
(declare-const c Real)

(assert (= (+ x (* a dx)) (- 212542581053874 (* 88 a))))
(assert (= (+ y (* a dy)) (- 357959731032403 (* 256 a))))
(assert (= (+ z (* a dz)) (- 176793474286781 (* 240 a))))

(assert (= (+ x (* b dx)) (+ 154677220587564 (* 184 b))))
(assert (= (+ y (* b dy)) (+ 207254130208265 (* 74 b))))
(assert (= (+ z (* b dz)) (+ 139183938188421 (* 235 b))))

(assert (= (+ x (* c dx)) (+ 216869547613134 (* 109 c))))
(assert (= (+ y (* c dy)) (+  38208083662943 (* 262 c))))
(assert (= (+ z (* c dz)) (- 397740686492049 (* 66 c))))

(check-sat)

(get-value (x y z))
(exit)
