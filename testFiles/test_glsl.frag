#version 450

layout(location = 1) in vec2 uv;
layout(location = 0) out vec4 color;

void main() {
    vec3 on = vec3(1.0);
    vec3 two = vec3(0.0);
    vec2 twoot = vec2(1.0, 15.0);
    vec3 doot = vec3(twoot, 13.0);
    bool ret = (on == two);
    float dotto = (on + two).x;
    int something = 3;
    if (something > 2){
        something = 10;
    } else {
        something = 14;
    }
    for(int i=0; i<5; i++){
        something = something + 1;
    }

}