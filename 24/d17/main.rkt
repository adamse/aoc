#lang rosette

(require rosette/lib/destruct)

; https://docs.racket-lang.org/rosette-guide/index.html

(define bit-width 64)

(define int? (bitvector bit-width))
(define (int n) (bv n int?))

(define io? (bitvector 3))
(define (io n) (bv n io?))

; fits 17 (1 + program len)
(define bv-ip? (bitvector 5))
(define (bv-ip n) (bv n bv-ip?))

(struct State (a b c ip end) #:transparent)
(struct Out (out state) #:transparent)
(struct End (state) #:transparent)

(define (mk-State p a b c)
  (State (int a) (int b) (int c) (bv-ip 0) (length-bv p bv-ip?)))

(define (combo a b c operand)
  (cond
    [(bvule operand (int 3)) operand]
    [(bveq operand (int 4)) a]
    [(bveq operand (int 5)) b]
    [(bveq operand (int 6)) c]
    [else (raise operand #t)]))

(define (interpret p s)
  (define (loop ip a b c)
    (if (bvuge ip (State-end s))
      (End (State a b c ip (State-end s)))
      (let
        ([opcode (list-ref-bv p ip)]
         [operand (zero-extend (list-ref-bv p (bvadd1 ip)) int?)]
         [ip (bvadd ip (bv-ip 2))])
        (cond
          [(equal? opcode (io 0))
            ; adv: a = a / (2^combo)
            (loop ip (bvlshr a (combo a b c operand)) b c)]
          [(equal? opcode (io 1))
            ; bxl: b = b ^ literal
            (loop ip a (bvxor b operand) c)]
          [(equal? opcode (io 2))
            ; bst: b = combo & 0b111
            (loop ip a (zero-extend (extract 2 0 (combo a b c operand)) int?) c)]
          [(equal? opcode (io 3))
            ; jnz: if a == 0: pass; else ip = literal; jump
            (if (bvzero? a)
              (loop ip a b c)
              (let*
                ([end1 (zero-extend (State-end s) int?)]
                 [ip1 (extract 4 0 (bvumin operand end1))])
                  (loop ip1 a b c)))]
          [(equal? opcode (io 4))
            ; bxc: state.b = state.b ^ state.c
            (loop ip a (bvxor b c) c)]
          [(equal? opcode (io 5))
            ; out: out combo & 0b111
            (Out (extract 2 0 (combo a b c operand)) (State a b c ip (State-end s)))]
          [(equal? opcode (io 6))
            ; bdv: b = a / combo
            (loop ip a (bvlshr a (combo a b c operand)) c)]
          [(equal? opcode (io 7))
            ; cdv: c = a / combo
            (loop ip a b (bvlshr a (combo a b c operand)))]
          [else (raise `("bad op" ,(bitvector->natural opcode) ,(bitvector->natural operand)) #t)]))))

  (destruct s
    [(State a b c ip _) (loop ip a b c)]
    [_ (raise "bad" #t)]))

(define (run-output p s)
  (define (loop s acc)
    (destruct (interpret p s)
      [(Out o s1) (loop s1 (cons o acc))]
      [(End s)
       (begin
         (map (lambda (x) (bitvector->natural x)) (reverse acc)))]))
  (loop s '()))

(define (mk-program xs)
  (map io xs))

(define ex1-p
  (mk-program '(0 1 5 4 3 0)))
(define ex1-s
  (mk-State ex1-p 729 0 0))

(define p1
  (mk-program '(2 4 1 1 7 5 0 3 1 4 4 4 5 5 3 0)))
(define s1
  (mk-State p1 30886132 0 0))

;(interpret ex1-p ex1-s)
;(run-output ex1-p ex1-s)
;(run-output p1 s1)

(define (r p a b c)
  (run-output (mk-program p) (mk-State p a b c)))

; (r '(2 6) 0 0 9)
; (r '(5 0 5 1 5 4) 10 0 0)
; (r '(0 1 5 4 3 0) 2024 0 0)
; (r '(1 7) 0 29 0)
; (int 26)
; (r '(4 0) 0 2024 43690)
; (int 44354)

(define (angelic p)
  (solve
    (begin
      (define-symbolic a int?)
      (define s (State a (int 0) (int 0) (bv-ip 0) (length-bv p bv-ip?)))

      (define (loop i s)
        (destruct (interpret p s)
          [(Out o s)
           (begin
             ; output is equal to progam at this point
             (assert (equal? (list-ref-bv p i) o))
             ; continue
             (loop (bvadd1 i) s))]
          [(End _)
           (begin
             ; this is a solution, but not the smallest, look for a better solution
             (assert (bvult a (bv #x0000b89ad7b82a2f 64)))
             ; this is a solution, but not the smallest, look for a better solution
             (assert (bvult a (bv #x0000b89ad7b82a2d 64)))
             ; this is the smallest solution!
             ;(assert (bvult a (bv #x0000b89ad7b82a2a 64)))
             (assert (equal? i (length-bv p bv-ip?))))]))

      (loop (bv-ip 0) s))))

#;(define (test p a)
  (run-output p (mk-State p a 0 0)))
;(angelic (mk-program '(0 3 5 4 3 0)))
;(test (mk-program '(0 3 5 4 3 0)) 117440)
(angelic (mk-program '(2 4 1 1 7 5 0 3 1 4 4 4 5 5 3 0)))
