#version 450

layout(set = 0, binding = 0) uniform sampler colorsampler;
layout(set = 0, binding = 1) uniform texture2D[{{input.len()}}] colormaps;

layout(location = 1) in vec2 uv;

// iter output data
{% for item in output %}
layout(location = {{loop.index-1}}) out {{ item.typ }} {{ item.identifier }};
{% endfor %}
void main() {
    // iter input reads
    {% for item in input %}
    {{ item.typ }} {{ item.identifier }} = texture(sampler2D(colormaps[{{loop.index-1}}], colorsampler), uv);
    {% endfor %}

    {{code}}
}