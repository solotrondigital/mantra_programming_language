(module
  ;; Mantra WebAssembly Target (Browser & WASI Compatible)
  (import "env" "print" (func $print (param i64)))
  (import "env" "print_str" (func $print_str (param i32 i32)))
  (memory (export "memory") 1)

  (func $add (param $a i64) (param $b i64) (result i64)
    local.get $a
    local.get $b
    i64.add
    local.set $_t1
    local.get $_t1
    return
    i64.const 0
  )
  (func $multiply (param $x f64) (param $y f64) (result f64)
    local.get $x
    local.get $y
    i64.mul
    local.set $_t2
    local.get $_t2
    return
    i64.const 0
  )
  (func $factorial (param $n i64) (result i64)
    (local $result i64)
    (local $i i64)
    i64.const 1
    local.set $_t3
    local.get $_t3
    local.set $result
    i64.const 1
    local.set $_t4
    local.get $_t4
    local.set $i
    local.get $i
    local.get $n
    i64.le_s
    local.set $_t5
    local.get $result
    local.get $i
    i64.mul
    local.set $_t6
    local.get $_t6
    local.set $result
    i64.const 1
    local.set $_t7
    local.get $i
    local.get $_t7
    i64.add
    local.set $_t8
    local.get $_t8
    local.set $i
    local.get $result
    return
    i64.const 0
  )
  (func $main (result i64)
    (local $sum i64)
    local.get $_t9
    call $print
    i64.const 0
    local.set $_t10
    i64.const 20
    local.set $_t11
    i64.const 22
    local.set $_t12
    local.get $_t13
    local.set $sum
    local.get $sum
    call $print
    i64.const 0
    local.set $_t14
    i64.const 0
    local.set $_t15
    local.get $_t15
    return
    i64.const 0
  )
  (export "main" (func $main))
)

