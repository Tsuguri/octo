
                              Capability Shader
               1:             ExtInstImport  "GLSL.std.450"
                              MemoryModel Logical GLSL450
                              EntryPoint Fragment 4  "main" 25 28
                              ExecutionMode 4 OriginUpperLeft
                              Source GLSL 450
                              Name 4  "main"
                              Name 8  "something"
                              Name 25  "uv"
                              Name 28  "color"
                              Decorate 25(uv) Location 1
                              Decorate 28(color) Location 0
               2:             TypeVoid
               3:             TypeFunction 2
               6:             TypeInt 32 1
               7:             TypePointer Function 6(int)
               9:      6(int) Constant 3
              11:      6(int) Constant 2
              12:             TypeBool
              16:      6(int) Constant 10
              18:      6(int) Constant 14
              20:      6(int) Constant 1
              22:             TypeFloat 32
              23:             TypeVector 22(float) 2
              24:             TypePointer Input 23(fvec2)
          25(uv):     24(ptr) Variable Input
              26:             TypeVector 22(float) 4
              27:             TypePointer Output 26(fvec4)
       28(color):     27(ptr) Variable Output
         4(main):           2 Function None 3
               5:             Label
    8(something):      7(ptr) Variable Function
                              Store 8(something) 9
              10:      6(int) Load 8(something)
              13:    12(bool) SGreaterThan 10 11
                              SelectionMerge 15 None
                              BranchConditional 13 14 17
              14:               Label
                                Store 8(something) 16
                                Branch 15
              17:               Label
                                Store 8(something) 18
                                Branch 15
              15:             Label
              19:      6(int) Load 8(something)
              21:      6(int) IAdd 19 20
                              Store 8(something) 21
                              Return
                              FunctionEnd