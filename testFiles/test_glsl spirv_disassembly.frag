; SPIR-V
; Version: 1.3
; Generator: Khronos Glslang Reference Front End; 7
; Bound: 54
; Schema: 0
               OpCapability Shader
          %1 = OpExtInstImport "GLSL.std.450"
               OpMemoryModel Logical GLSL450
               OpEntryPoint Fragment %main "main" %uv %color
               OpExecutionMode %main OriginUpperLeft
               OpSource GLSL 450
               OpName %main "main"
               OpName %on "on"
               OpName %two "two"
               OpName %som "som"
               OpName %wut "wut"
               OpName %duh "duh"
               OpName %PushConsts "PushConsts"
               OpMemberName %PushConsts 0 "view"
               OpMemberName %PushConsts 1 "projection"
               OpMemberName %PushConsts 2 "lolz"
               OpMemberName %PushConsts 3 "model"
               OpName %push "push"
               OpName %uv "uv"
               OpName %color "color"
               OpMemberDecorate %PushConsts 0 ColMajor
               OpMemberDecorate %PushConsts 0 Offset 0
               OpMemberDecorate %PushConsts 0 MatrixStride 16
               OpMemberDecorate %PushConsts 1 ColMajor
               OpMemberDecorate %PushConsts 1 Offset 64
               OpMemberDecorate %PushConsts 1 MatrixStride 16
               OpMemberDecorate %PushConsts 2 Offset 128
               OpMemberDecorate %PushConsts 3 ColMajor
               OpMemberDecorate %PushConsts 3 Offset 144
               OpMemberDecorate %PushConsts 3 MatrixStride 16
               OpDecorate %PushConsts Block
               OpDecorate %uv Location 1
               OpDecorate %color Location 0
       %void = OpTypeVoid
          %3 = OpTypeFunction %void
      %float = OpTypeFloat 32
    %v3float = OpTypeVector %float 3
%_ptr_Function_v3float = OpTypePointer Function %v3float
    %float_1 = OpConstant %float 1
         %11 = OpConstantComposite %v3float %float_1 %float_1 %float_1
    %float_0 = OpConstant %float 0
         %14 = OpConstantComposite %v3float %float_0 %float_0 %float_0
    %v4float = OpTypeVector %float 4
%_ptr_Function_v4float = OpTypePointer Function %v4float
    %float_2 = OpConstant %float 2
         %19 = OpConstantComposite %v4float %float_2 %float_2 %float_2 %float_2
%_ptr_Function_float = OpTypePointer Function %float
%mat4v4float = OpTypeMatrix %v4float 4
 %PushConsts = OpTypeStruct %mat4v4float %mat4v4float %v3float %mat4v4float
%_ptr_PushConstant_PushConsts = OpTypePointer PushConstant %PushConsts
       %push = OpVariable %_ptr_PushConstant_PushConsts PushConstant
        %int = OpTypeInt 32 1
      %int_0 = OpConstant %int 0
%_ptr_PushConstant_mat4v4float = OpTypePointer PushConstant %mat4v4float
      %int_3 = OpConstant %int 3
      %int_2 = OpConstant %int 2
%_ptr_PushConstant_v3float = OpTypePointer PushConstant %v3float
    %v2float = OpTypeVector %float 2
%_ptr_Input_v2float = OpTypePointer Input %v2float
         %uv = OpVariable %_ptr_Input_v2float Input
%_ptr_Output_v4float = OpTypePointer Output %v4float
      %color = OpVariable %_ptr_Output_v4float Output
       %main = OpFunction %void None %3
          %5 = OpLabel
         %on = OpVariable %_ptr_Function_v3float Function
        %two = OpVariable %_ptr_Function_v3float Function
        %som = OpVariable %_ptr_Function_v4float Function
        %wut = OpVariable %_ptr_Function_float Function
        %duh = OpVariable %_ptr_Function_v4float Function
               OpStore %on %11
               OpStore %two %14
               OpStore %som %19
         %22 = OpLoad %v3float %on
         %23 = OpLoad %v3float %two
         %24 = OpDot %float %22 %23
               OpStore %wut %24
         %33 = OpAccessChain %_ptr_PushConstant_mat4v4float %push %int_0
         %34 = OpLoad %mat4v4float %33
         %35 = OpLoad %v4float %som
         %36 = OpMatrixTimesVector %v4float %34 %35
               OpStore %duh %36
         %38 = OpAccessChain %_ptr_PushConstant_mat4v4float %push %int_3
         %39 = OpLoad %mat4v4float %38
         %42 = OpAccessChain %_ptr_PushConstant_v3float %push %int_2
         %43 = OpLoad %v3float %42
         %44 = OpCompositeExtract %float %43 0
         %45 = OpCompositeExtract %float %43 1
         %46 = OpCompositeExtract %float %43 2
         %47 = OpCompositeConstruct %v4float %44 %45 %46 %float_1
         %48 = OpMatrixTimesVector %v4float %39 %47
               OpStore %duh %48
               OpReturn
               OpFunctionEnd